use ndarray::Array2;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct ValveNetwork {
    adjacency: Array2<bool>,
    rates: Vec<u32>,
    start: usize,
}

mod parse {
    use nom::{
        branch::alt,
        bytes::complete::{is_a, tag},
        character::complete::line_ending,
        multi::separated_list1 as seplist,
        sequence::preceded as prec,
        IResult,
    };

    pub(super) type Label<'a> = &'a str;
    const ALPHA: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    fn line(input: &str) -> IResult<&str, (Label, u32, Vec<Label>)> {
        let name = &input[6..8];
        let (input, rate) = nom::character::complete::u32(&input[23..])?;

        let links_parse = seplist(tag(", "), is_a(ALPHA));
        let (input, links) = prec(alt((tag("s "), tag(" "))), links_parse)(&input[23..])?;
        Ok((input, (name, rate, links)))
    }

    pub(super) fn valves(input: &str) -> IResult<&str, Vec<(Label, u32, Vec<Label>)>> {
        seplist(line_ending, line)(input)
    }
}

#[aoc_generator(day16)]
fn generate(input: &str) -> ValveNetwork {
    let valves = parse::valves(input).unwrap().1;
    let num_valves = valves.len();

    // Mapping of labels to final indices
    let indices: HashMap<parse::Label, usize> = {
        let mut map = HashMap::with_capacity(num_valves);
        for (i, &(name, _, _)) in valves.iter().enumerate() {
            map.insert(name, i);
        }
        map
    };

    let mut network = ValveNetwork {
        adjacency: Array2::from_elem((num_valves, num_valves), false),
        rates: valves.iter().map(|&(_, r, _)| r).collect(),
        start: indices["AA"],
    };

    for (valve, (_, _, links)) in valves.into_iter().enumerate() {
        for link in links.into_iter().map(|l| indices[l]) {
            network.adjacency[(valve, link)] = true;
            network.adjacency[(link, valve)] = true;
        }
    }

    network
}

const TIME: usize = 30;

#[aoc(day16, part1)]
fn solve_part1(input: &ValveNetwork) -> u32 {
    todo!()
}

#[aoc(day16, part2)]
fn solve_part2(input: &ValveNetwork) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;

    const SAMPLE_INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), 1651);
    }

    #[test]
    fn part1_mine() {
        assert_eq!(solve_part1(&generate(&crate::get_input(16))), todo!());
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&generate(SAMPLE_INPUT)), todo!());
    }

    #[test]
    fn part2_mine() {
        assert_eq!(solve_part2(&generate(&crate::get_input(16))), todo!());
    }
}
