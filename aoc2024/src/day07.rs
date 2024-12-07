mod parse {
    use nom::{
        bytes::complete::{tag, take_while},
        character::{complete::newline, is_digit},
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

fn is_valid_equation_inner_p1(test_value: usize, accumulator: usize, operands: &[usize]) -> bool {
    let Some((&next, operands)) = operands.split_first() else {
        // base case: the operands array is empty
        return test_value == accumulator;
    };

    // try adding
    // we assume this won't overflow(!)
    let accumulator_add = accumulator + next;
    if is_valid_equation_inner_p1(test_value, accumulator_add, operands) {
        return true;
    }

    // try multiplying
    let Some(accumulator_mul) = accumulator.checked_mul(next) else {
        // we can't multiply reasonably so neither addition nor multiplication work
        return false;
    };
    is_valid_equation_inner_p1(test_value, accumulator_mul, operands)
}

fn is_valid_equation_p1(test_value: usize, operands: &[usize]) -> bool {
    // use the first value as the accumulator to avoid resolving a
    // zero multiplication in error (as we would if we used acc=0 for first call)
    let Some((&acc, operands)) = operands.split_first() else {
        unreachable!("empty operands array in input");
    };

    is_valid_equation_inner_p1(test_value, acc, operands)
}

#[aoc(day07, part1)]
fn solve_part1(input: &[(usize, Vec<usize>)]) -> usize {
    input
        .iter()
        .filter(|(test_value, operands)| is_valid_equation_p1(*test_value, &operands))
        .map(|(tv, _)| tv)
        .sum()
}

fn is_valid_equation_inner_p2(test_value: usize, accumulator: usize, operands: &[usize]) -> bool {
    let Some((&next, operands)) = operands.split_first() else {
        // base case: the operands array is empty
        return test_value == accumulator;
    };

    // try adding
    // we assume this won't overflow(!)
    let accumulator_add = accumulator + next;
    if is_valid_equation_inner_p2(test_value, accumulator_add, operands) {
        return true;
    }

    // try multiplying
    let Some(accumulator_mul) = accumulator.checked_mul(next) else {
        // we can't multiply reasonably so neither addition nor multiplication work
        return false;
    };
    if is_valid_equation_inner_p2(test_value, accumulator_mul, operands) {
        return true;
    }

    // try concatenating
    let accumulator_con = {
        let len_next = (next as f64).log10() as u32 + 1;
        (accumulator * 10usize.pow(len_next)) + next
    };
    is_valid_equation_inner_p2(test_value, accumulator_con, operands)
}

fn is_valid_equation_p2(test_value: usize, operands: &[usize]) -> bool {
    // use the first value as the accumulator to avoid resolving a
    // zero multiplication in error (as we would if we used acc=0 for first call)
    let Some((&acc, operands)) = operands.split_first() else {
        unreachable!("empty operands array in input");
    };

    is_valid_equation_inner_p2(test_value, acc, operands)
}

#[aoc(day07, part2)]
fn solve_part2(input: &[(usize, Vec<usize>)]) -> usize {
    input
        .iter()
        .filter(|(test_value, operands)| is_valid_equation_p2(*test_value, &operands))
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
