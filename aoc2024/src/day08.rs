use crate::grid::{Grid, Point};
use std::collections::{BTreeSet, HashMap};

type Field = (HashMap<u8, Vec<Point>>, Grid<()>);

#[aoc_generator(day08)]
fn generate(input: &str) -> Field {
    let mut antennas = HashMap::new();

    for (y, line) in input.lines().rev().enumerate() {
        for (x, ch) in line.bytes().enumerate() {
            if ch == b'.' {
                continue;
            }

            antennas
                .entry(ch)
                .or_insert_with(Vec::new)
                .push((x, y).into());
        }
    }

    (
        antennas,
        Grid::empty(
            input.lines().next().unwrap().as_bytes().len(),
            input.lines().count(),
        ),
    )
}

fn antinodes_p1((antennas, grid): &Field) -> BTreeSet<Point> {
    let mut nodes = BTreeSet::new();

    for antennas in antennas.values() {
        for (i, &first) in antennas.iter().enumerate() {
            for &second in antennas.iter().skip(i + 1) {
                let (xdiff, ydiff) = second.vector(first);
                if let Some(first_antinode) = grid.offset_point(first, (xdiff, ydiff)) {
                    nodes.insert(first_antinode);
                }
                if let Some(second_antinode) = grid.offset_point(second, (-xdiff, -ydiff)) {
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

fn antinodes_p2((antennas, grid): &Field) -> BTreeSet<Point> {
    let mut nodes = BTreeSet::new();

    for antennas in antennas.values() {
        for (i, &first) in antennas.iter().enumerate() {
            for &second in antennas.iter().skip(i + 1) {
                let (xdiff, ydiff) = Point::vector(second, first);
                for node in grid
                    .raycast(first, (-xdiff, -ydiff))
                    .chain(grid.raycast(second, (xdiff, ydiff)))
                {
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
            let antinodes = antinodes_p1(&field);
            assert!(antinodes.contains(&(6, 2).into()));
            assert!(antinodes.contains(&(3, 8).into()));
            assert_eq!(antinodes.len(), 2);
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
