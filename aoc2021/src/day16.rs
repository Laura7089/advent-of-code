use crate::bits::{get_bits, parse, BITSPacket, BITSPacketVersioned};

#[aoc_generator(day16)]
pub fn parse_input(input: &str) -> BITSPacketVersioned {
    parse(&get_bits(input)).0
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
pub fn solve_part2(input: &BITSPacketVersioned) -> usize {
    input.eval()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let example_input = "A0016C880162017C3686B18A3D4780";
        assert_eq!(solve_part1(&parse_input(&example_input)), 31);
    }

    #[test]
    fn part2_example() {
        let example_input = "9C0141080250320F1802104A08";
        assert_eq!(solve_part2(&parse_input(&example_input)), 1);
    }

    #[test]
    fn part1_myinput() {
        let _input = crate::get_input_for_day(16);
        assert_eq!(solve_part1(&parse_input(&_input)), 947);
    }

    #[test]
    fn part2_myinput() {
        let _input = crate::get_input_for_day(16);
        assert_eq!(solve_part2(&parse_input(&_input)), 660797830937);
    }
}
