use crate::{manhattan, Pair, Range, UPoint as Point};

mod parse {
    use crate::{parse::*, IPoint};
    use nom::{
        bytes::complete::tag,
        character::complete::line_ending,
        multi::separated_list1,
        sequence::{preceded as pre, separated_pair as seppair},
    };

    fn pair(input: &str) -> IResult<IPoint> {
        let x = pre(tag("x="), isize);
        let y = pre(tag("y="), isize);
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

type SensPair = Pair<Point>;

#[aoc_generator(day15)]
fn generate(input: &str) -> (Vec<SensPair>, Pair<usize>) {
    let points_raw = parse::beacons(input).unwrap().1;

    let x_off = get_offset(points_raw.iter().flat_map(|&((l, _), (r, _))| [l, r]));
    let y_off = get_offset(points_raw.iter().flat_map(|&((_, l), (_, r))| [l, r]));

    (
        points_raw
            .into_iter()
            .map(|((sx, sy), (bx, by))| {
                (
                    (
                        (x_off).saturating_add_signed(sx),
                        (y_off).saturating_add_signed(sy),
                    ),
                    (
                        (x_off).saturating_add_signed(bx),
                        (y_off).saturating_add_signed(by),
                    ),
                )
            })
            .collect(),
        (x_off, y_off),
    )
}

const MERGE_PASSES: usize = 3;

fn part1_inner((pairs, (_, y_off)): &(Vec<SensPair>, Point), goal: usize) -> usize {
    let goal_line = goal + y_off;
    let mut intersects = Vec::new();

    for &(sensor, beacon) in pairs.iter() {
        // Maximum distance the sensor can see
        let max_dist = manhattan::distu(sensor, beacon);
        // Vertical distance from sensor to goal line
        let goal_dist = sensor.1.abs_diff(goal_line);

        // If the sensor diamond won't reach our goal line, skip it
        if goal_dist > max_dist {
            continue;
        }

        // Half (rounded down) the length of the intersection
        // between the sensor diamond and the goal line
        let reach = max_dist - goal_dist;

        let pair = Range {
            start: sensor.0 - reach,
            end: sensor.0 + reach,
        };
        let mut combined = false;

        for i in 0..intersects.len() {
            if let Some(new_pair) = pair.union(intersects[i]) {
                intersects.remove(i);
                intersects.push(new_pair);
                combined = true;
                break;
            }
        }

        if !combined {
            intersects.push(pair);
        }
    }

    for _ in 0..MERGE_PASSES {
        for pairi in 1..intersects.len() {
            // Since we're shrinking intersects, we need to ensure we don't
            // overrun it
            if pairi == intersects.len() {
                break;
            }

            let (&[.., lpair], remaining) = intersects.split_at(pairi)
            else { unreachable!() };

            for i in 0..remaining.len() {
                match lpair.union(remaining[i]) {
                    Some(new_pair) => {
                        intersects.remove(pairi - 1);
                        intersects.remove(i + pairi - 1);
                        intersects.push(new_pair);
                        break;
                    }
                    None => {}
                }
            }
        }
    }

    intersects.into_iter().map(|r| r.len() - 1).sum()
}

const GOAL_LINE: usize = 2_000_000;

#[aoc(day15, part1)]
fn solve_part1(args: &(Vec<SensPair>, Point)) -> usize {
    part1_inner(args, GOAL_LINE)
}

fn part2_inner(
    (pairs, (x_off, y_off)): &(Vec<SensPair>, Point),
    (lower, upper): Pair<usize>,
) -> usize {
    let pairs: Vec<_> = pairs
        .iter()
        .map(|&(sensor, beacon)| (sensor, manhattan::distu(sensor, beacon)))
        .collect();

    for y in (lower + y_off)..(upper + y_off) {
        let xs = Range {
            start: lower + x_off,
            end: upper + x_off,
        };

        let intersects = pairs.iter().filter_map(|&((sx, sy), dist)| {
            let height_diff = sy.abs_diff(y);
            if height_diff > dist {
                None
            } else {
                let arm_len = dist - height_diff;
                Some(Range {
                    start: sx.saturating_sub(arm_len),
                    end: sx + arm_len,
                })
            }
        });

        if let Some(x) = xs.demolish(intersects) {
            println!("Found suitable range: {x:?}");
            return (x.to_single().unwrap() - x_off) * 4_000_000 + (y - y_off);
        }

        println!("Couldn't find any matches at y={y}");
    }

    panic!("No suitable location found");
}

#[aoc(day15, part2)]
fn solve_part2(args: &(Vec<SensPair>, Point)) -> usize {
    part2_inner(args, (0, 4_000_000))
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
        assert_eq!(part1_inner(&generate(SAMPLE_INPUT), 10), 26);
    }

    #[test]
    fn part1_mine() {
        assert_eq!(solve_part1(&generate(&crate::get_input(15))), 4879972);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2_inner(&generate(SAMPLE_INPUT), (0, 20)), 56000011);
    }

    #[test]
    fn part2_mine() {
        assert_eq!(solve_part2(&generate(&crate::get_input(15))), todo!());
    }
}
