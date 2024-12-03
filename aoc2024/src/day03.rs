mod parse {
    use nom::{
        bytes::complete::{tag, take_while_m_n},
        character::complete::anychar,
        combinator::{map, map_res},
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

    fn mul_instr(input: &str) -> IResult<(usize, usize)> {
        preceded(
            tag("mul"),
            delimited(tag("("), separated_pair(number, tag(","), number), tag(")")),
        )(input)
    }

    fn rubbish_then_instr(input: &str) -> IResult<(usize, usize)> {
        map(many_till(anychar, mul_instr), |(_chars, instr)| instr)(input)
    }

    pub fn get_all_muls(input: &str) -> IResult<Vec<(usize, usize)>> {
        many1(rubbish_then_instr)(input)
    }
}

#[aoc_generator(day03)]
fn generate(input: &str) -> Vec<(usize, usize)> {
    parse::get_all_muls(input).expect("parse failure").1
}

#[aoc(day03, part1)]
fn solve_part1(input: &[(usize, usize)]) -> usize {
    input.iter().map(|&(l, r)| l * r).sum()
}

#[aoc(day03, part2)]
fn solve_part2(_input: &[(usize, usize)]) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;

    const SAMPLE_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), 161);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part1(&generate(&crate::get_input(03))), 169021493);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part2(&generate(SAMPLE_INPUT)), todo!());
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part2(&generate(&crate::get_input(03))), todo!());
        }
    }
}
