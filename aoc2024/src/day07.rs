mod parse {
    use nom::{
        bytes::complete::{tag, take_while},
        character::complete::newline,
        combinator::map_res,
        multi::separated_list0,
        sequence::separated_pair,
    };

    type IResult<'a, T> = nom::IResult<&'a str, T>;

    fn num(input: &str) -> IResult<usize> {
        map_res(take_while(|c: char| c.is_ascii_digit()), |raw: &str| {
            raw.parse()
        })(input)
    }

    fn equation(input: &str) -> IResult<(usize, Vec<usize>)> {
        let operands = separated_list0(tag(" "), num);

        separated_pair(num, tag(": "), operands)(input)
    }

    pub fn equations(input: &str) -> IResult<Vec<(usize, Vec<usize>)>> {
        separated_list0(newline, equation)(input)
    }
}

#[aoc_generator(day07)]
fn generate(input: &str) -> Vec<(usize, Vec<usize>)> {
    parse::equations(input).expect("parse error").1
}

fn valid_eq_p1(target: usize, oprs: &[usize]) -> bool {
    let Some((&next, oprs)) = oprs.split_last() else {
        // base case: no more operands, have we reached 0?
        return target == 0;
    };
    if next > target {
        // base case: our next operand is larger than the target
        // and therefore cannot reduce it in a valid way
        return false;
    }

    // recursive cases
    if valid_eq_p1(target - next, oprs) {
        return true;
    }
    if target % next == 0 {
        valid_eq_p1(target / next, oprs)
    } else {
        false
    }
}

#[aoc(day07, part1)]
fn solve_part1(input: &[(usize, Vec<usize>)]) -> usize {
    input
        .iter()
        .filter(|(target, oprs)| valid_eq_p1(*target, oprs))
        .map(|(tv, _)| tv)
        .sum()
}

fn valid_eq_p2(target: usize, oprs: &[usize]) -> bool {
    let Some((&next, oprs)) = oprs.split_last() else {
        // base case: no more operands, have we reached 0?
        return target == 0;
    };
    if next > target {
        // base case: our next operand is larger than the target
        // and therefore cannot reduce it in a valid way
        return false;
    }

    // recursive cases
    // addition
    if valid_eq_p2(target - next, oprs) {
        return true;
    }

    // multiplication
    if target % next == 0 && valid_eq_p2(target / next, oprs) {
        return true;
    }

    // concatenation
    let mag = 10usize.pow(next.ilog10() + 1);
    target % mag == next && valid_eq_p2(target / mag, oprs)
}

#[aoc(day07, part2)]
fn solve_part2(input: &[(usize, Vec<usize>)]) -> usize {
    input
        .iter()
        .filter(|(target, oprs)| valid_eq_p2(*target, oprs))
        .map(|(tv, _)| tv)
        .sum()
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;

    const SAMPLE_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), 3749);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part1(&generate(&crate::get_input(07))), 303766880536);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part2(&generate(SAMPLE_INPUT)), 11387);
        }

        #[test]
        fn mine() {
            assert_eq!(
                solve_part2(&generate(&crate::get_input(07))),
                337041851384440
            );
        }
    }
}
