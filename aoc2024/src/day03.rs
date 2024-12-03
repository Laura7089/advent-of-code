#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Instruction {
    Mul(usize, usize),
    Do,
    Dont,
}

mod parse {
    use super::Instruction;
    use nom::{
        branch::alt,
        bytes::complete::{tag, take_while_m_n},
        character::complete::anychar,
        combinator::{map, map_res, value},
        multi::{many1, many_till},
        sequence::{delimited, preceded, separated_pair},
    };

    type IResult<'a, T> = nom::IResult<&'a str, T>;

    fn number(input: &str) -> IResult<usize> {
        // select 1..=3 ascii digits and parse them into a integer
        map_res(
            take_while_m_n(1, 3, |c: char| c.is_ascii_digit()),
            |raw: &str| raw.parse(),
        )(input)
    }

    fn mul_instr(input: &str) -> IResult<Instruction> {
        map(
            preceded(
                tag("mul"),
                delimited(tag("("), separated_pair(number, tag(","), number), tag(")")),
            ),
            |(l, r)| Instruction::Mul(l, r),
        )(input)
    }

    fn do_instr(input: &str) -> IResult<Instruction> {
        value(Instruction::Do, tag("do()"))(input)
    }

    fn dont_instr(input: &str) -> IResult<Instruction> {
        value(Instruction::Dont, tag("don't()"))(input)
    }

    fn instr(input: &str) -> IResult<Instruction> {
        alt((mul_instr, do_instr, dont_instr))(input)
    }

    fn rubbish_then_instr(input: &str) -> IResult<Instruction> {
        map(many_till(anychar, instr), |(_chars, instr)| instr)(input)
    }

    pub fn get_all_muls(input: &str) -> IResult<Vec<Instruction>> {
        many1(rubbish_then_instr)(input)
    }
}

#[aoc_generator(day03)]
fn generate(input: &str) -> Vec<Instruction> {
    parse::get_all_muls(input).expect("parse failure").1
}

#[aoc(day03, part1)]
fn solve_part1(input: &[Instruction]) -> usize {
    input
        .iter()
        .filter_map(|ins| match ins {
            Instruction::Mul(l, r) => Some(l * r),
            _ => None,
        })
        .sum()
}

#[aoc(day03, part2)]
fn solve_part2(input: &[Instruction]) -> usize {
    let mut enabled = true;
    let mut total = 0;

    for &ins in input {
        match ins {
            Instruction::Mul(l, r) if enabled => total += l * r,
            Instruction::Do if !enabled => enabled = true,
            Instruction::Dont if enabled => enabled = false,
            _ => continue,
        }
    }

    total
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;

    const SAMPLE_INPUT_PART1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part1(&generate(SAMPLE_INPUT_PART1)), 161);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part1(&generate(&crate::get_input(03))), 169021493);
        }
    }

    const SAMPLE_INPUT_PART2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part2(&generate(SAMPLE_INPUT_PART2)), 48);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part2(&generate(&crate::get_input(03))), 111762583);
        }
    }
}
