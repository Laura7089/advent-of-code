use std::collections::BTreeMap;
use std::ops::{Add, Div, Mul, Sub};

use crate::arith::{FullOp, Op};

mod parse {
    use super::*;
    use crate::parse::*;

    use nom::{
        branch::alt,
        bytes::complete::{tag, take},
        character::complete::line_ending,
        combinator::{map, value},
        multi::separated_list1,
        sequence::{delimited, separated_pair as seppair, tuple},
    };

    fn name(input: &str) -> IResult<Name> {
        let (input, raw) = take(4usize)(input)?;
        let raw = raw.as_bytes();
        Ok((input, [raw[0], raw[1], raw[2], raw[3]]))
    }

    fn op(input: &str) -> IResult<Op> {
        alt((
            value(Op::Add, tag("+")),
            value(Op::Sub, tag("-")),
            value(Op::Mul, tag("*")),
            value(Op::Div, tag("/")),
        ))(input)
    }

    fn job(input: &str) -> IResult<Job> {
        alt((
            map(usize, |n| Job::Literal(Fixed(n))),
            map(
                tuple((name, delimited(tag(" "), op, tag(" ")), name)),
                |(l, op, r)| Job::Op(op, l, r),
            ),
        ))(input)
    }

    fn monkey(input: &str) -> IResult<(Name, Job)> {
        seppair(name, tag(": "), job)(input)
    }

    pub(super) fn monkeys(input: &str) -> IResult<Vec<(Name, Job)>> {
        separated_list1(line_ending, monkey)(input)
    }
}

/// A simple mathematical expression
///
/// It can either be a known value, which is just mutated as other known-value operations are
/// applied, or it can be unknown, which will hold the operations applied to it for later
/// evaluation.
#[derive(Clone, Debug, PartialEq)]
enum Expr {
    Var(Vec<FullOp<usize>>),
    Fixed(usize),
}

use Expr::*;

impl Expr {
    /// Tries to mathematically simplify the expression by combining similar operations or pruning
    /// no-ops
    fn simplify(&mut self) {
        // We can only simplify Exprs with variables in them
        if let Var(ref mut ops) = self {
            let mut i = 0;

            // TODO: this would probably be faster backwards...
            while i < ops.len() - 1 {
                if let Some(newop) = ops[i].try_combine(ops[i + 1]) {
                    ops.remove(i);
                    ops.remove(i);
                    if !newop.is_noop() {
                        ops.insert(i, newop);
                    }
                }

                // Remove noops
                if ops[i].is_noop() {
                    ops.remove(i);
                } else {
                    i += 1;
                }
            }
        }
    }

    fn push_op(&mut self, op: FullOp<usize>) {
        if let Var(ref mut list) = self {
            list.push(op);
            self.simplify();
        } else {
            panic!("called push_op on a known Expr");
        }
    }
}

impl Add<Self> for Expr {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (mut v @ Var(_), Fixed(n)) | (Fixed(n), mut v @ Var(_)) => {
                v.push_op(FullOp(Op::Add, n));
                v
            }
            (Fixed(l), Fixed(r)) => Fixed(l + r),
            (Var(_), Var(_)) => unreachable!("Tried to perform an add on two variable expressions"),
        }
    }
}

impl Sub<Self> for Expr {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (mut v @ Var(_), Fixed(n)) | (Fixed(n), mut v @ Var(_)) => {
                v.push_op(FullOp(Op::Sub, n));
                v
            }
            (Fixed(l), Fixed(r)) => Fixed(l - r),
            (Var(_), Var(_)) => panic!("Tried to perform a sub on two unknown expressions"),
        }
    }
}

impl Mul<Self> for Expr {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (mut v @ Var(_), Fixed(n)) | (Fixed(n), mut v @ Var(_)) => {
                v.push_op(FullOp(Op::Mul, n));
                v
            }
            (Fixed(l), Fixed(r)) => Fixed(l * r),
            _ => panic!("Tried to perform a mul on two unknown expressions"),
        }
    }
}

impl Div<Self> for Expr {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (mut v @ Var(_), Fixed(n)) | (Fixed(n), mut v @ Var(_)) => {
                v.push_op(FullOp(Op::Div, n));
                v
            }
            (Fixed(l), Fixed(r)) => Fixed(l / r),
            _ => panic!("Tried to perform a div on two unknown expressions"),
        }
    }
}

type Name = [u8; 4];

/// A job held by a monkey
#[derive(Debug, Clone, PartialEq)]
enum Job {
    Literal(Expr),
    Op(Op, Name, Name),
}

impl Job {
    fn resolve(&self, map: &BTreeMap<Name, Job>) -> Expr {
        match self {
            Job::Literal(n) => n.clone(),
            Job::Op(op, l, r) => {
                // TODO: cloning here could be a potential slowdown, but it
                // seems impossible to avoid
                // Perhaps each monkey is only used once, so we can pull out of the map?
                let l = map[l].clone();
                let r = map[r].clone();
                op.apply(l.resolve(map), r.resolve(map))
            }
        }
    }
}

#[aoc_generator(day21)]
fn generate(input: &str) -> BTreeMap<Name, Job> {
    let monkeys = parse::monkeys(input).unwrap().1;

    let mut map = BTreeMap::new();
    for (name, job) in monkeys {
        map.insert(name, job);
    }
    map
}

#[aoc(day21, part1)]
fn solve_part1(input: &BTreeMap<Name, Job>) -> usize {
    if let Fixed(n) = input[b"root"].resolve(input) {
        n
    } else {
        panic!("part 1 somehow has a variable expression in it")
    }
}

#[aoc(day21, part2)]
fn solve_part2(input: &BTreeMap<Name, Job>) -> usize {
    let mut input = input.clone();
    input.insert(*b"humn", Job::Literal(Var(Vec::with_capacity(input.len()))));

    let (mut current, to_solve) = {
        let Job::Op(_, l, r) = input[b"root"] else {
            unreachable!()
        };
        let root_solutions = (input[&l].resolve(&input), input[&r].resolve(&input));
        match root_solutions {
            (Fixed(n), Var(l)) | (Var(l), Fixed(n)) => (n, l),
            _ => unreachable!(),
        }
    };

    println!("{current} from {to_solve:?}");
    for op in to_solve.into_iter().rev() {
        current = op.apply_rev(current);
    }

    current
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;

    const SAMPLE_INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), 152);
        }

        #[test]
        fn mine() {
            assert_eq!(
                solve_part1(&generate(&crate::get_input(21))),
                121868120894282
            );
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part2(&generate(SAMPLE_INPUT)), 301);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part2(&generate(&crate::get_input(21))), todo!());
        }
    }
}
