type Worry = u128;

#[derive(Debug, Copy, Clone)]
enum Operand {
    Old,
    Literal(Worry),
}

impl Operand {
    fn get_value(self, value: Worry) -> Worry {
        match self {
            Self::Old => value,
            Self::Literal(x) => x,
        }
    }
}

impl From<&str> for Operand {
    fn from(input: &str) -> Self {
        match input.trim() {
            "old" => Self::Old,
            x => Self::Literal(x.parse().unwrap()),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Add(Operand),
    Sub(Operand),
    Mul(Operand),
    Div(Operand),
}

impl From<&str> for Operation {
    fn from(input: &str) -> Self {
        let operand = input.split(' ').nth(1).unwrap().into();
        match input.split(' ').next().unwrap() {
            "*" => Self::Mul(operand),
            "/" => Self::Div(operand),
            "+" => Self::Add(operand),
            "-" => Self::Sub(operand),
            other => panic!("Unsupported operation literal {other}"),
        }
    }
}

impl Operation {
    fn apply(&self, worry: &mut Worry) {
        match self {
            Self::Mul(o) => *worry *= o.get_value(*worry),
            Self::Div(o) => *worry /= o.get_value(*worry),
            Self::Add(o) => *worry += o.get_value(*worry),
            Self::Sub(o) => *worry -= o.get_value(*worry),
        }
    }

    fn try_simplify(lhs: Self, rhs: Self) -> Option<Self> {
        use Operand::Literal;
        use Operation::*;

        match (lhs, rhs) {
            (Mul(Literal(x)), Div(Literal(y))) if x % y == 0 => Some(Mul(Literal(x / y))),

            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    holding: Vec<(Worry, Vec<Operation>)>,
    op: Operation,
    test_mod: Worry,
    on_success: usize,
    on_fail: usize,
    inspections: usize,
}

impl From<&str> for Monkey {
    fn from(input: &str) -> Self {
        let mut lines = input.lines().skip(1);

        Monkey {
            holding: lines.next().unwrap()[18..]
                .split(", ")
                .map(|i| (i.parse().unwrap(), vec![]))
                .collect(),
            op: lines.next().unwrap()[23..].into(),
            test_mod: lines.next().unwrap()[21..].parse().unwrap(),
            on_success: lines.next().unwrap()[29..].parse().unwrap(),
            on_fail: lines.next().unwrap()[30..].parse().unwrap(),
            inspections: 0,
        }
    }
}

#[aoc_generator(day11)]
fn generate(input: &str) -> Vec<Monkey> {
    input.split("\n\n").map(From::from).collect()
}

const PART1_ROUNDS: usize = 20;

#[aoc(day11, part1)]
fn solve_part1(input: &[Monkey]) -> usize {
    let mut monkeys = input.to_owned();

    for _round in 0..PART1_ROUNDS {
        for m in 0..monkeys.len() {
            for (mut item, _) in monkeys[m].holding.drain(..).collect::<Vec<_>>() {
                monkeys[m].op.apply(&mut item);
                item /= 3;
                if item % monkeys[m].test_mod == 0 {
                    let on_success = monkeys[m].on_success;
                    monkeys[on_success].holding.push((item, vec![]));
                } else {
                    let on_fail = monkeys[m].on_fail;
                    monkeys[on_fail].holding.push((item, vec![]));
                }
                monkeys[m].inspections += 1;
            }
        }
    }

    let mut interactions: Vec<usize> = monkeys.into_iter().map(|m| m.inspections).collect();
    interactions.sort_unstable();
    interactions.pop().unwrap() * interactions.pop().unwrap()
}

const PART2_ROUNDS: usize = 10_000;

#[aoc(day11, part2)]
fn solve_part2(input: &[Monkey]) -> u128 {
    // TODO: chinese remainder theorem
    let mut monkeys = input.to_owned();

    for _round in 0..PART2_ROUNDS {
        for m in 0..monkeys.len() {
            for (item, mut ops) in monkeys[m].holding.clone() {
                ops.push(monkeys[m].op);
                if item % monkeys[m].test_mod == 0 {
                    let on_success = monkeys[m].on_success;
                    monkeys[on_success].holding.push((item, ops));
                } else {
                    let on_fail = monkeys[m].on_fail;
                    monkeys[on_fail].holding.push((item, ops));
                }
                monkeys[m].inspections += 1;
            }
            monkeys[m].holding.clear();
        }
    }

    let mut interactions: Vec<usize> = monkeys.into_iter().map(|m| m.inspections).collect();
    interactions.sort_unstable();
    interactions.pop().unwrap() as u128 * interactions.pop().unwrap() as u128
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;

    const SAMPLE_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), 10605);
    }

    #[test]
    fn part1_mine() {
        assert_eq!(solve_part1(&generate(&crate::get_input(11))), 67830);
    }

    #[test]
    #[ignore]
    fn part2_example() {
        assert_eq!(solve_part2(&generate(SAMPLE_INPUT)), 2713310158);
    }

    #[test]
    #[ignore]
    fn part2_mine() {
        assert_eq!(solve_part2(&generate(&crate::get_input(11))), todo!());
    }
}
