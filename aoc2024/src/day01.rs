mod parse {
    use winnow::ascii::digit1;
    use winnow::combinator::{separated, separated_pair};
    use winnow::prelude::*;
    use winnow::Result;

    fn num(input: &mut &str) -> Result<usize> {
        digit1.parse_to().parse_next(input)
    }

    fn num_pair(input: &mut &str) -> Result<(usize, usize)> {
        separated_pair(num, "   ", num).parse_next(input)
    }

    pub fn whole_input(input: &mut &str) -> Result<Vec<(usize, usize)>> {
        separated(0.., num_pair, '\n').parse_next(input)
    }
}

use winnow::Parser;

type Generated = Vec<(usize, usize)>;

#[aoc_generator(day01)]
fn generate(mut input: &str) -> Generated {
    parse::whole_input.parse(&mut input).unwrap()
}

#[aoc(day01, part1)]
fn solve_part1(input: &Generated) -> usize {
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
fn solve_part2(input: &Generated) -> usize {
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
    use test_case::test_case;

    const SAMPLE_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    fn sample_parsed() -> Generated {
        generate(SAMPLE_INPUT)
    }

    fn mine_parsed() -> Generated {
        generate(&crate::get_input(01))
    }

    #[test_case(sample_parsed(), 11; "sample")]
    #[test_case(mine_parsed(), 3714264; "mine")]
    fn part1(parsed: Generated, solution: usize) {
        assert_eq!(solve_part1(&parsed), solution);
    }

    #[test_case(sample_parsed(), 31; "sample")]
    #[test_case(mine_parsed(), 18805872; "mine")]
    fn part2(parsed: Generated, solution: usize) {
        assert_eq!(solve_part2(&parsed), solution);
    }
}
