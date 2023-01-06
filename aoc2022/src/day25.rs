#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]

type Snafu = i64;

mod parse {
    use super::Snafu;
    use crate::parse::*;
    use nom::{
        character::complete::{line_ending, one_of},
        combinator::map,
        multi::{many1, separated_list1},
    };

    fn digit(input: &str) -> IResult<Snafu> {
        map(one_of("012-="), |c| match c as u8 {
            c @ b'0'..=b'2' => (c - b'0').into(),
            b'-' => -1,
            b'=' => -2,
            _ => unreachable!(),
        })(input)
    }

    fn number(input: &str) -> IResult<Snafu> {
        map(many1(digit), |ds| {
            ds.into_iter()
                .rev()
                .enumerate()
                .map(|(i, d)| d * 5i64.pow(i as u32))
                .sum()
        })(input)
    }

    pub(super) fn num_list(input: &str) -> IResult<Vec<Snafu>> {
        separated_list1(line_ending, number)(input)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn example() {
            for (canon, dec) in super::super::tests::EXAMPLE_CONVERSIONS {
                assert_eq!(num_list(canon).unwrap().1[0], *dec);
            }
        }
    }
}

fn canonicalise_snafu(decimal: Snafu) -> String {
    // Find x where 5^x gives the lowest possible value above `decimal`
    let max_exp = (decimal as f32).log(5.0).ceil() as u32;

    let to_return: String = (0..=max_exp)
        .rev()
        .scan(decimal, |rem, exp| {
            let place_value = 5i64.pow(exp);

            let (_, &mut digit_value, _) = [-2, -1, 0, 1, 2]
                // Find the minimum absolute value of:
                // |remaining - (digit * place_value)|
                .select_nth_unstable_by(0, |lhs: &i64, rhs: &i64| {
                    // Guard against overflows where we can
                    if lhs.checked_mul(place_value).is_none() {
                        return std::cmp::Ordering::Greater;
                    } else if rhs.checked_mul(place_value).is_none() {
                        return std::cmp::Ordering::Less;
                    }

                    let lhs_key = rem.abs_diff(lhs * place_value);
                    let rhs_key = rem.abs_diff(rhs * place_value);
                    lhs_key.partial_cmp(&rhs_key).unwrap()
                });

            *rem -= digit_value * place_value;
            Some(match digit_value {
                -2 => '=',
                -1 => '-',
                0 => '0',
                1 => '1',
                2 => '2',
                _ => unreachable!(),
            })
        })
        .collect();

    // Knock off any leading 0s
    to_return.trim_start_matches('0').to_owned()
}

#[aoc_generator(day25)]
fn generate(input: &str) -> Vec<Snafu> {
    parse::num_list(input).unwrap().1
}

#[aoc(day25, part1)]
fn solve_part1(input: &[Snafu]) -> String {
    let total: Snafu = input.iter().sum();
    canonicalise_snafu(total)
}

#[aoc(day25, part2)]
fn solve_part2(_input: &[Snafu]) -> usize {
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

    #[cfg(test)]
    pub(super) const EXAMPLE_CONVERSIONS: &[(&str, i64)] = &[
        ("1", 1),
        ("2", 2),
        ("1=", 3),
        ("1-", 4),
        ("10", 5),
        ("11", 6),
        ("12", 7),
        ("2=", 8),
        ("2-", 9),
        ("20", 10),
        ("21", 11),
        ("1=0", 15),
        ("1-0", 20),
        ("111", 31),
        ("112", 32),
        ("122", 37),
        ("1-12", 107),
        ("2=0=", 198),
        ("2=01", 201),
        ("1=-1=", 353),
        ("12111", 906),
        ("20012", 1257),
        ("1=-0-2", 1747),
        ("1=11-2", 2022),
        ("1-0---0", 12345),
        ("1121-1110-1=0", 314159265),
    ];

    mod canonicalise {
        use super::*;
        use proptest::proptest;

        proptest! {
            #[test]
            fn prop(s in r"[12][=\-012]{1, 12}") {
                let dec = generate(&s)[0];
                let canonicalised = canonicalise_snafu(dec);
                let reparsed = generate(&canonicalised)[0];
                assert_eq!(s, canonicalised, "{dec} got converted to {reparsed}");
            }
        }

        #[test]
        fn examples() {
            for (canon, dec) in EXAMPLE_CONVERSIONS {
                assert_eq!(&canonicalise_snafu(*dec), canon);
            }
        }
    }

    fn parsed_sum(input: &str) -> Snafu {
        generate(input).into_iter().sum::<Snafu>()
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
            assert_eq!(
                solve_part1(&generate(&crate::get_input(25))),
                "2=222-2---22=1=--1-2".to_owned()
            );
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
