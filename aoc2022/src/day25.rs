type SNAFU = i32;

mod parse {
    use super::SNAFU;
    use nom::{
        character::complete::{line_ending, one_of},
        combinator::map,
        multi::{many1, separated_list1},
    };

    type IResult<'a, T> = nom::IResult<&'a str, T>;

    fn digit(input: &str) -> IResult<SNAFU> {
        map(one_of("012-="), |c| match c as u8 {
            c @ b'0'..=b'2' => (c - b'0').into(),
            b'-' => -1,
            b'=' => -2,
            _ => unreachable!(),
        })(input)
    }

    fn number(input: &str) -> IResult<SNAFU> {
        let (input, digits) = many1(digit)(input)?;
        let val = digits
            .into_iter()
            .rev()
            .enumerate()
            .map(|(i, d)| d * 5i32.pow(i as u32))
            .sum();
        Ok((input, val))
    }

    pub(super) fn num_list(input: &str) -> IResult<Vec<SNAFU>> {
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

fn canonicalise_snafu(decimal: SNAFU) -> String {
    // Find x where 5^x gives the lowest possible value above `decimal`
    let max_exp = (decimal as f32).log(5.0).ceil() as u32;

    // // If we can't subtract enough from this particular exponent, skip it
    // if 5i32.pow(max_exp) - (2 * (5i32.pow(max_exp - 1))) > decimal {
    //     max_exp = max_exp.saturating_sub(1);
    // }
    // println!("decimal={decimal} max_exp={max_exp}");

    let mut to_return = String::with_capacity(max_exp as usize + 1);
    let mut remaining = decimal;

    for exp in (0..=max_exp).rev() {
        let place_value = 5i32.pow(exp);

        let digit_value = *[-2, -1, 0, 1, 2]
            // Find the minimum absolute value of:
            // |remaining - (digit * place_value)|
            .select_nth_unstable_by(0, |lhs, rhs| {
                let lhs_key = (remaining - (lhs * place_value)).abs();
                let rhs_key = (remaining - (rhs * place_value)).abs();
                lhs_key.partial_cmp(&rhs_key).unwrap()
            })
            .1;

        remaining -= digit_value * place_value;
        to_return.push(match digit_value {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => unreachable!(),
        });
    }

    if to_return.chars().next() == Some('0') {
        to_return.remove(0);
    }
    to_return
}

#[aoc_generator(day25)]
fn generate(input: &str) -> Vec<SNAFU> {
    parse::num_list(input).unwrap().1
}

#[aoc(day25, part1)]
fn solve_part1(input: &[SNAFU]) -> String {
    let total: SNAFU = input.iter().sum();
    canonicalise_snafu(total)
}

#[aoc(day25, part2)]
fn solve_part2(_input: &[SNAFU]) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;
    use proptest::proptest;

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

    proptest! {
        #[test]
        fn canonicalise(s in r"[12][=\-012]{1, 6}") {
            let dec = generate(&s)[0];
            let canonicalised = canonicalise_snafu(dec);
            let reparsed = generate(&canonicalised)[0];
            assert_eq!(s, canonicalised, "{dec} got converted to {reparsed}");
        }
    }

    #[test]
    fn canonicalise_example() {
        assert_eq!(canonicalise_snafu(4890), "2=-1=0");
    }

    fn parsed_sum(input: &str) -> i32 {
        generate(input).into_iter().sum::<i32>()
    }

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(parsed_sum(SAMPLE_INPUT), 4890);
            assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), "2=-1=0".to_owned());
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part1(&generate(&crate::get_input(25))), "".to_owned());
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
            assert_eq!(solve_part2(&generate(&crate::get_input(25))), todo!());
        }
    }
}
