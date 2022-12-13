mod parse {
    use super::Packet;
    use nom::{
        branch::alt,
        character::{complete::char, streaming::line_ending},
        combinator::map,
        multi::separated_list0,
        sequence::{delimited, separated_pair},
        IResult,
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

    fn packet(input: &str) -> IResult<&str, Packet> {
        alt((int, list))(input)
    }

    pub(super) fn packet_pair(input: &str) -> IResult<&str, (Packet, Packet)> {
        separated_pair(packet, line_ending, packet)(input)
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Packet {
    Int(u32),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering;
        use Packet::*;

        match (self, other) {
            (&Int(ref l), &Int(ref r)) => PartialOrd::partial_cmp(l, r),
            (&List(ref l), &List(ref r)) => {
                // TODO: will rust's inbuilt PartialOrd impl for Vecs do this?
                for (l, r) in l.iter().zip(r.iter()) {
                    let ordering = PartialOrd::partial_cmp(l, r);
                    if ordering.is_some() && ordering != Some(Ordering::Equal) {
                        return ordering;
                    }
                }
                PartialOrd::partial_cmp(&l.len(), &r.len())
            }
            (i @ &Int(_), l @ &List(_)) => {
                PartialOrd::partial_cmp(&Packet::List(vec![i.clone()]), l)
            }
            (l @ &List(_), i @ &Int(_)) => {
                PartialOrd::partial_cmp(l, &Packet::List(vec![i.clone()]))
            }
        }
    }
}

#[aoc_generator(day13)]
fn generate(input: &str) -> Vec<(Packet, Packet)> {
    use nom::Finish;
    input
        .split("\n\n")
        .map(|pair| parse::packet_pair(pair).finish().unwrap().1)
        .collect()
}

#[aoc(day13, part1)]
fn solve_part1(input: &[(Packet, Packet)]) -> usize {
    input
        .iter()
        .enumerate()
        .filter_map(|(i, (l, r))| if l < r { Some(i + 1) } else { None })
        .sum()
}

#[aoc(day13, part2)]
fn solve_part2(input: &[(Packet, Packet)]) -> usize {
    todo!()
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
        assert_eq!(solve_part1(&generate(&crate::get_input(13))), todo!());
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&generate(SAMPLE_INPUT)), todo!());
    }

    #[test]
    fn part2_mine() {
        assert_eq!(solve_part2(&generate(&crate::get_input(13))), todo!());
    }
}
