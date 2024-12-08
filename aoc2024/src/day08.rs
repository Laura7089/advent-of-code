use std::collections::{BTreeSet, HashMap};

type Point = (usize, usize);
#[derive(Debug, Clone)]
struct Field {
    antennas: HashMap<u8, Vec<Point>>,
    width: usize,
    height: usize,
}

impl Field {
    fn offset_point(&self, (x, y): Point, xdiff: isize, ydiff: isize) -> Option<Point> {
        let x = x.checked_add_signed(xdiff)?;
        let y = y.checked_add_signed(ydiff)?;

        (x < self.width && y < self.height).then_some((x, y))
    }

    fn raycast(&self, start: Point, xdiff: isize, ydiff: isize) -> SteppedRaycast {
        SteppedRaycast {
            field: self,
            cursor: Some(start),
            xdiff,
            ydiff,
        }
    }
}

#[inline]
#[allow(clippy::cast_possible_wrap)]
fn get_vector(first: Point, second: Point) -> (isize, isize) {
    (
        second.0 as isize - first.0 as isize,
        second.1 as isize - first.1 as isize,
    )
}

#[aoc_generator(day08)]
fn generate(input: &str) -> Field {
    let mut antennas = HashMap::new();

    for (y, line) in input.lines().rev().enumerate() {
        for (x, ch) in line.bytes().enumerate() {
            if ch == b'.' {
                continue;
            }

            antennas.entry(ch).or_insert_with(Vec::new).push((x, y));
        }
    }

    Field {
        antennas,
        width: input.lines().next().unwrap().as_bytes().len(),
        height: input.lines().count(),
    }
}

fn antinodes_p1(field: &Field) -> BTreeSet<Point> {
    let mut nodes = BTreeSet::new();

    for antennas in field.antennas.values() {
        for i in 1..antennas.len() {
            let (&[.., first], rem) = antennas.split_at(i) else {
                continue;
            };

            for &second in rem {
                let (xdiff, ydiff) = get_vector(second, first);
                if let Some(first_antinode) = field.offset_point(first, xdiff, ydiff) {
                    nodes.insert(first_antinode);
                }
                if let Some(second_antinode) = field.offset_point(second, -xdiff, -ydiff) {
                    nodes.insert(second_antinode);
                }
            }
        }
    }

    nodes
}

#[aoc(day08, part1)]
fn solve_part1(input: &Field) -> usize {
    antinodes_p1(input).len()
}

#[derive(Clone, Debug)]
struct SteppedRaycast<'a> {
    field: &'a Field,
    cursor: Option<Point>,
    xdiff: isize,
    ydiff: isize,
}

impl Iterator for SteppedRaycast<'_> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        self.cursor = self
            .field
            .offset_point(self.cursor?, self.xdiff, self.ydiff);
        self.cursor
    }
}

fn antinodes_p2(field: &Field) -> BTreeSet<Point> {
    let mut nodes = BTreeSet::new();

    for antennas in field.antennas.values() {
        for i in 1..antennas.len() {
            let (&[.., first], rem) = antennas.split_at(i) else {
                continue;
            };

            for &second in rem {
                let (xdiff, ydiff) = get_vector(second, first);
                for node in field.raycast(first, -xdiff, -ydiff) {
                    nodes.insert(node);
                }
                for node in field.raycast(second, xdiff, ydiff) {
                    nodes.insert(node);
                }
            }
        }
    }

    nodes
}

#[aoc(day08, part2)]
fn solve_part2(input: &Field) -> usize {
    antinodes_p2(input).len()
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;

    const SAMPLE_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    mod part1 {
        use super::*;

        #[test]
        fn partial_example1() {
            let partial = "..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
..........";
            let field = generate(partial);
            println!("{field:?}");
            let antinodes = antinodes_p1(&field);
            println!("{antinodes:?}");
            assert!(antinodes.contains(&(6, 2)));
            assert!(antinodes.contains(&(3, 8)));
        }

        #[test]
        fn full_example() {
            assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), 14);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part1(&generate(&crate::get_input(08))), 332);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part2(&generate(SAMPLE_INPUT)), 34);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part2(&generate(&crate::get_input(08))), 1174);
        }
    }
}
