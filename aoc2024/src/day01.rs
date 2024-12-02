mod parse {
    use nom::bytes::complete::take_while;
    use nom::character::complete::{newline, space1};
    use nom::combinator::map_res;
    use nom::multi::separated_list0;
    use nom::sequence::separated_pair;

    type Input<'a> = &'a str;
    type IResult<'a, T> = nom::IResult<Input<'a>, T>;

    fn num(input: Input) -> IResult<usize> {
        map_res(take_while(|c: char| c.is_ascii_digit()), |raw| {
            usize::from_str_radix(raw, 10)
        })(input)
    }

    fn num_pair(input: Input) -> IResult<(usize, usize)> {
        separated_pair(num, space1, num)(input)
    }

    pub fn whole_input(input: Input) -> IResult<Vec<(usize, usize)>> {
        separated_list0(newline, num_pair)(input)
    }
}

#[aoc_generator(day01)]
fn generate(input: &str) -> Vec<(usize, usize)> {
    parse::whole_input(input).unwrap().1
}

#[aoc(day01, part1)]
fn solve_part1(input: &[(usize, usize)]) -> usize {
    let mut left = Vec::with_capacity(input.len());
    let mut right = Vec::with_capacity(input.len());

    for &(l, r) in input {
        left.push(l);
        right.push(r);
    }

    left.sort_unstable();
    right.sort_unstable();

    left.into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

use std::collections::BTreeMap;

#[aoc(day01, part2)]
fn solve_part2(input: &[(usize, usize)]) -> usize {
    let mut left = BTreeMap::new();
    let mut right = BTreeMap::new();
    for &(l, r) in input {
        left.entry(l).and_modify(|count| *count += 1).or_insert(1);
        right.entry(r).and_modify(|count| *count += 1).or_insert(1);
    }

    left.into_iter()
        .filter_map(|(num, lc)| right.get(&num).map(|rc| lc * rc * num))
        .sum()
}

#[cfg(test)]
mod test {
    #![allow(unreachable_code)]
    use super::*;

    const SAMPLE_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), 11);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part1(&generate(&crate::get_input(01))), 3714264);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part2(&generate(SAMPLE_INPUT)), 31);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part2(&generate(&crate::get_input(01))), 18805872);
        }
    }
}
