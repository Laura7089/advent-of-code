type SnafuNum = i32;

mod parse {
    use super::SnafuNum;
    use nom::{
        character::complete::{line_ending, one_of},
        combinator::map,
        multi::{many1, separated_list1},
    };

    type IResult<'a, T> = nom::IResult<&'a str, T>;

    fn digit(input: &str) -> IResult<SnafuNum> {
        map(one_of("012-="), |c| match c as u8 {
            c @ b'0'..=b'2' => (c - b'0').into(),
            b'-' => -1,
            b'=' => -2,
            _ => unreachable!(),
        })(input)
    }

    fn number(input: &str) -> IResult<SnafuNum> {
        let (input, digits) = many1(digit)(input)?;
        let val = digits
            .into_iter()
            .rev()
            .enumerate()
            .map(|(i, d)| d * 5i32.pow(i as u32))
            .sum();
        Ok((input, val))
    }

    pub(super) fn num_list(input: &str) -> IResult<Vec<SnafuNum>> {
        separated_list1(line_ending, number)(input)
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;

        #[test_case("1=-0-2" => 1747)]
        #[test_case("12111" => 906)]
        #[test_case("2=0=" => 198)]
        #[test_case("21" => 11)]
        #[test_case("2=01" => 201)]
        #[test_case("111" => 31)]
        #[test_case("20012" => 1257)]
        #[test_case("112" => 32)]
        #[test_case("1=-1=" => 353)]
        #[test_case("1-12" => 107)]
        #[test_case("12" => 7)]
        #[test_case("1=" => 3)]
        #[test_case("122" => 37)]
        fn example(input: &str) -> i32 {
            super::number(input).unwrap().1
        }
    }
}

#[aoc_generator(day25)]
fn generate(input: &str) -> Vec<SnafuNum> {
    parse::num_list(input).unwrap().1
}

#[aoc(day25, part1)]
fn solve_part1(input: &[SnafuNum]) -> String {
    todo!()
}

#[aoc(day25, part2)]
fn solve_part2(_input: &[SnafuNum]) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;

    const SAMPLE_INPUT: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    #[test]
    fn part1_example_decimal() {
        assert_eq!(generate(SAMPLE_INPUT).into_iter().sum::<i32>(), 4890);
    }

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), "2=-1=0".to_owned());
    }

    #[test]
    fn part1_mine() {
        assert_eq!(solve_part1(&generate(&crate::get_input(25))), "".to_owned());
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&generate(SAMPLE_INPUT)), todo!());
    }

    #[test]
    fn part2_mine() {
        assert_eq!(solve_part2(&generate(&crate::get_input(25))), todo!());
    }
}
