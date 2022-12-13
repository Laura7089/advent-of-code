use itertools::Itertools;
mod parse {
    use super::Packet;
    use nom::{
        branch::alt, character::complete::char, combinator::map, multi::separated_list0,
        sequence::delimited, IResult,
    };

    fn int(input: &str) -> IResult<&str, Packet> {
        use nom::character::complete::u32;
        map(u32, Packet::Int)(input)
    }

    fn list(input: &str) -> IResult<&str, Packet> {
        map(
            delimited(char('['), separated_list0(char(','), packet), char(']')),
            Packet::List,
        )(input)
    }

    pub(super) fn packet(input: &str) -> IResult<&str, Packet> {
        alt((int, list))(input)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Packet {
    Int(u32),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use Packet::*;

        match (self, other) {
            (&Int(ref l), &Int(ref r)) => PartialOrd::partial_cmp(l, r),
            (&List(ref l), &List(ref r)) => PartialOrd::partial_cmp(l, r),
            (i @ &Int(_), l @ &List(_)) => {
                PartialOrd::partial_cmp(&Packet::List(vec![i.clone()]), l)
            }
            (l @ &List(_), i @ &Int(_)) => {
                PartialOrd::partial_cmp(l, &Packet::List(vec![i.clone()]))
            }
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[aoc_generator(day13)]
fn generate(input: &str) -> Vec<Packet> {
    use nom::Finish;
    input
        .split("\n\n")
        .flat_map(|pair| pair.lines().map(|l| parse::packet(l).finish().unwrap().1))
        .collect()
}

#[aoc(day13, part1)]
fn solve_part1(input: &[Packet]) -> usize {
    input
        .iter()
        .chunks(2)
        .into_iter()
        .enumerate()
        .filter_map(|(i, mut c)| {
            if c.next().unwrap() < c.next().unwrap() {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day13, part2)]
fn solve_part2(input: &[Packet]) -> usize {
    let mut input = input.to_owned();

    let dividers = [
        parse::packet("[[2]]").unwrap().1,
        parse::packet("[[6]]").unwrap().1,
    ];
    input.push(dividers[0].clone());
    input.push(dividers[1].clone());

    input.sort_unstable();

    dividers
        .into_iter()
        .map(|divider| {
            input
                .iter()
                .enumerate()
                .find_map(|(i, p)| if p == &divider { Some(i + 1) } else { None })
                .unwrap()
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), 13);
    }

    #[test]
    fn part1_mine() {
        assert_eq!(solve_part1(&generate(&crate::get_input(13))), 5760);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&generate(SAMPLE_INPUT)), 140);
    }

    #[test]
    fn part2_mine() {
        assert_eq!(solve_part2(&generate(&crate::get_input(13))), 26670);
    }
}
