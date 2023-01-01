type Point = (u32, u32);

mod parse {
    use super::Point;
    use nom::{
        bytes::complete::tag,
        character::complete::none_of,
        sequence::{preceded as pre, separated_pair as seppair, tuple},
    };
    type IResult<'a, T> = nom::IResult<&'a str, T>;

    fn pair(input: &str) -> IResult<Point> {
        use nom::character::complete::u32;
        seppair(pre(tag("x="), u32), tag(", "), pre(tag("y="), u32))(input)
    }

    pub fn beacon(input: &str) -> IResult<(Point, Point)> {
        tuple((pre(none_of("x"), pair), pre(none_of("x"), pair)))(input)
    }
}

#[aoc_generator(day15)]
fn generate(input: &str) -> Vec<(Point, Point)> {
    input.lines().map(|l| parse::beacon(l).unwrap().1).collect()
}

#[aoc(day15, part1)]
fn solve_part1(input: &[(Point, Point)]) -> usize {
    todo!()
}

#[aoc(day15, part2)]
fn solve_part2(input: &[(Point, Point)]) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;

    const SAMPLE_INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), 26);
    }

    #[test]
    fn part1_mine() {
        assert_eq!(solve_part1(&generate(&crate::get_input(15))), todo!());
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&generate(SAMPLE_INPUT)), todo!());
    }

    #[test]
    fn part2_mine() {
        assert_eq!(solve_part2(&generate(&crate::get_input(15))), todo!());
    }
}
