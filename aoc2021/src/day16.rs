use crate::bits::{get_bits, BITSPacket, BITSPacketVersioned};

#[aoc_generator(day16)]
pub fn parse_input(input: &str) -> BITSPacketVersioned {
    BITSPacketVersioned::parse(&get_bits(input)).0
}

fn sum_version(pack: &BITSPacketVersioned) -> usize {
    match &pack.packet {
        BITSPacket::Literal(_) => pack.version as usize,
        BITSPacket::Operator(_, subs) => {
            subs.iter().map(sum_version).sum::<usize>() + pack.version as usize
        }
    }
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &BITSPacketVersioned) -> usize {
    sum_version(input)
}

#[aoc(day16, part2)]
pub fn solve_part2(_input: &BITSPacketVersioned) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "A0016C880162017C3686B18A3D4780";

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&parse_input(&EXAMPLE_INPUT)), 31);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&parse_input(&EXAMPLE_INPUT)), unimplemented!());
    }

    #[test]
    fn part1_myinput() {
        let _input = crate::get_input_for_day(16);
        assert_eq!(solve_part1(&parse_input(&_input)), unimplemented!());
    }

    #[test]
    fn part2_myinput() {
        let _input = crate::get_input_for_day(16);
        assert_eq!(solve_part2(&parse_input(&_input)), unimplemented!());
    }
}
