use itertools::Itertools;
use ndarray::s;

use crate::{manhattan_dist_signed, manhattan_dist_unsigned, UPoint as Point};

#[derive(Copy, Clone, PartialEq, Debug)]
enum BeaconState {
    Possible,
    Impossible,
    Confirmed,
}

type Cave = crate::OffsetGrid<BeaconState>;

mod parse {
    use crate::{make_isize, IPoint, IResult};
    use nom::{
        bytes::complete::tag,
        character::complete::line_ending,
        multi::separated_list1,
        sequence::{preceded as pre, separated_pair as seppair},
    };

    fn pair(input: &str) -> IResult<IPoint> {
        let x = pre(tag("x="), make_isize);
        let y = pre(tag("y="), make_isize);
        seppair(x, tag(", "), y)(input)
    }

    fn beacon(input: &str) -> IResult<(IPoint, IPoint)> {
        seppair(
            pre(tag("Sensor at "), pair),
            tag(": closest beacon is at "),
            pair,
        )(input)
    }

    pub fn beacons(input: &str) -> IResult<Vec<(IPoint, IPoint)>> {
        separated_list1(line_ending, beacon)(input)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn beacon_sanity() {
            assert_eq!(
                beacon("Sensor at x=551202, y=3971545: closest beacon is at x=-595451, y=3788543")
                    .unwrap()
                    .1,
                ((551202, 3971545), (-595451, 3788543))
            );

            assert_eq!(
                beacons(
                    "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16"
                )
                .unwrap()
                .1,
                vec![((2, 18), (-2, 15)), ((9, 16), (10, 16))]
            );
        }
    }
}

fn get_offset(seq: impl IntoIterator<Item = isize>) -> usize {
    seq.into_iter().min().unwrap().min(0).abs() as usize
}

#[aoc_generator(day15)]
fn generate(input: &str) -> (Vec<(Point, Point)>, Point, usize) {
    let points_raw = parse::beacons(input).unwrap().1;

    let max_dist = points_raw
        .iter()
        .map(|&(l, r)| manhattan_dist_signed(l, r))
        .max()
        .unwrap();

    let x_off = get_offset(points_raw.iter().map(|&((lx, _), (rx, _))| lx.min(rx)));
    let y_off = get_offset(points_raw.iter().map(|&((_, ly), (_, ry))| ly.min(ry)));

    (
        points_raw
            .into_iter()
            .map(|((sx, sy), (bx, by))| {
                (
                    (
                        (x_off + max_dist).saturating_add_signed(sx),
                        (y_off + max_dist).saturating_add_signed(sy),
                    ),
                    (
                        (x_off + max_dist).saturating_add_signed(bx),
                        (y_off + max_dist).saturating_add_signed(by),
                    ),
                )
            })
            .collect(),
        (x_off, y_off),
        max_dist,
    )
}

#[derive(Debug, Clone)]
struct SensorDiamond {
    sensor: Point,
    max_dist: usize,
    current_dist: usize,
    arc_dist: usize,
}

impl SensorDiamond {
    fn new(sensor: Point, beacon: Point) -> Self {
        let dist = manhattan_dist_unsigned(sensor, beacon);
        Self {
            sensor,
            max_dist: dist,
            current_dist: 0,
            arc_dist: 0,
        }
    }
}

impl Iterator for SensorDiamond {
    type Item = [Point; 4];

    fn next(&mut self) -> Option<Self::Item> {
        // First return the sensor itself
        if self.current_dist == 0 {
            self.current_dist += 1;
            // TODO: this is stupid
            return Some([self.sensor; 4]);
        }

        // If we're at the end of the arc, move to the next ring
        if self.arc_dist == self.current_dist {
            self.arc_dist = 0;
            self.current_dist += 1;
        }

        // If we've exceeded the sensor's range, don't return anything
        if self.current_dist == self.max_dist + 1 {
            return None;
        }

        let inc = self.arc_dist;
        let dec = self.current_dist - self.arc_dist;
        let (sx, sy) = self.sensor;

        self.arc_dist += 1;

        Some([
            (sx.saturating_add(inc), sy.saturating_add(dec)),
            (sx.saturating_add(dec), sy.saturating_sub(inc)),
            (sx.saturating_sub(inc), sy.saturating_sub(dec)),
            (sx.saturating_sub(dec), sy.saturating_add(inc)),
        ])
    }
}

// // Function removed because it used allocation
// fn sensor_diamond(sensor @ (sx, sy): Point, beacon: Point) -> Vec<Point> {
//     let dist = manhattan_dist_unsigned(sensor, beacon);

//     // 4 * sum of 1..dist
//     // add one for the sensor itself
//     let total_points = ((dist * (dist + 1)) * 2) + 1;
//     let mut points = Vec::with_capacity(total_points);
//     points.push(sensor);

//     for d in 1..=dist {
//         for i in 0..d {
//             let inc = i;
//             let dec = d - i;
//             points.extend_from_slice(&[
//                 (sx.saturating_add(inc), sy.saturating_add(dec)),
//                 (sx.saturating_add(dec), sy.saturating_sub(inc)),
//                 (sx.saturating_sub(inc), sy.saturating_sub(dec)),
//                 (sx.saturating_sub(dec), sy.saturating_add(inc)),
//             ]);
//         }
//     }

//     points
// }

fn part1_inner(
    (pairs, (_, y_off), max_dist): &(Vec<(Point, Point)>, Point, usize),
    goal: usize,
) -> usize {
    let (x0, x1) = pairs
        .iter()
        .map(|&((lx, _), (rx, _))| [lx, rx])
        .flatten()
        .minmax()
        .into_option()
        .unwrap();
    let (y0, y1) = pairs
        .iter()
        .map(|&((_, ly), (_, ry))| [ly, ry])
        .flatten()
        .minmax()
        .into_option()
        .unwrap();

    let mut possibles = Cave::new(
        (x0 - max_dist, y0 - max_dist),
        (x1 + max_dist, y1 + max_dist),
        BeaconState::Possible,
    );
    let goal_line = goal + y_off + max_dist;

    for &(sensor, beacon) in pairs.iter() {
        for quad in SensorDiamond::new(sensor, beacon) {
            for point in quad {
                possibles[point] = BeaconState::Impossible;
            }
        }
    }

    for &(_, beacon) in pairs.iter() {
        possibles[beacon] = BeaconState::Confirmed;
    }

    possibles
        .grid
        .slice(s![.., goal_line])
        .iter()
        .filter(|&&s| s == BeaconState::Impossible)
        .count()
}

const GOAL_LINE: usize = 2_000_000;

#[aoc(day15, part1)]
fn solve_part1(args: &(Vec<(Point, Point)>, Point, usize)) -> usize {
    part1_inner(args, GOAL_LINE)
}

#[aoc(day15, part2)]
fn solve_part2((pairs, (_, y_off), max_dist): &(Vec<(Point, Point)>, Point, usize)) -> usize {
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
    fn sensor_diamond() {
        let mut points = super::sensor_diamond((5, 5), (6, 7));
        points.sort_unstable();
        assert_eq!(
            points,
            vec![
                (2, 5),
                (3, 4),
                (3, 5),
                (3, 6),
                (4, 3),
                (4, 4),
                (4, 5),
                (4, 6),
                (4, 7),
                (5, 2),
                (5, 3),
                (5, 4),
                (5, 5),
                (5, 6),
                (5, 7),
                (5, 8),
                (6, 3),
                (6, 4),
                (6, 5),
                (6, 6),
                (6, 7),
                (7, 4),
                (7, 5),
                (7, 6),
                (8, 5),
            ]
        );
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1_inner(&generate(SAMPLE_INPUT), 10), 26);
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
