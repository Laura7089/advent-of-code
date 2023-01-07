use std::fmt::Display;

use crate::{grids::Offset, UPoint as Point};
use itertools::Itertools;
use ndarray::s;

type Cave = Offset<Tile>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
enum Tile {
    #[default]
    Air,
    Rock,
    Sand,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Air => '.',
                Tile::Rock => '#',
                Tile::Sand => 'o',
            }
        )
    }
}

const SAND_SRC: Point = (500, 0);

#[aoc_generator(day14)]
fn generate(input: &str) -> Cave {
    let strata: Vec<Vec<Point>> = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|seg| {
                    let mut seg = seg.split(',');
                    (
                        seg.next().unwrap().parse().unwrap(),
                        seg.next().unwrap().parse().unwrap(),
                    )
                })
                .collect()
        })
        .collect();

    let (x0, x1) = strata
        .iter()
        .flatten()
        .chain([&SAND_SRC].into_iter())
        .map(|p| p.0)
        .minmax()
        .into_option()
        .unwrap();
    let y1 = strata.iter().flatten().map(|p| p.1).max().unwrap();

    let mut cave = Cave::new((x0, SAND_SRC.1), (x1, y1), Tile::Air);

    for stratum in strata {
        let mut prev = stratum[0];
        for corner @ (sx, sy) in stratum.into_iter().skip(1) {
            if sx == prev.0 {
                // vertical
                for y in sy.min(prev.1)..=sy.max(prev.1) {
                    cave[(sx, y)] = Tile::Rock;
                }
            } else {
                // horizontal
                for x in sx.min(prev.0)..=sx.max(prev.0) {
                    cave[(x, sy)] = Tile::Rock;
                }
            }
            prev = corner;
        }
    }

    cave
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum SandFall {
    /// Sand falls to the given position
    Falls(Point),
    /// Sand has come to rest
    Rests,
    /// It has fallen into the Å̸̉͊̂̇̈́̃ͣ҉̘̮̳̫̤͠B̶̢͓̤̠̜̯͚̘̮̟͖͎̄̓̆̽̀͑ͭ̇̕͜͝ͅY͒ͤ͆͐͌͆ͨͦ̌̚͏̹̮̞̲̼̼͉̭̮̪͜͡S̸̛̘̯̲̭̊̃̀̓ͥ̔͝S̡̭̥̖̭͙̼͓͔͎̭̬̭͕̹͉̯̗ͫͨͭ͑͛̐ͮ̊̔̊ͮ͂̓͡
    LostToAbyss,
}

fn fall_from(cave: &Cave, (sx, y): Point) -> SandFall {
    for x in [sx, sx - 1, sx + 1] {
        let next = (x, y + 1);
        if !cave.contains(next) {
            return SandFall::LostToAbyss;
        }
        if cave[next] == Tile::Air {
            return SandFall::Falls(next);
        }
    }
    SandFall::Rests
}

#[aoc(day14, part1)]
fn solve_part1(cave: &Cave) -> usize {
    let mut cave = cave.clone();
    let mut particles = 0;

    loop {
        let mut current = SAND_SRC;
        while let SandFall::Falls(next) = fall_from(&cave, current) {
            current = next;
        }

        // fall_from will eventually get stuck when called
        // repeatedly, so we can call again here to check the value again
        // without any fear
        match fall_from(&cave, current) {
            SandFall::Rests => {
                cave[current] = Tile::Sand;
                particles += 1;
            }
            SandFall::LostToAbyss => return particles,
            SandFall::Falls(_) => unreachable!(),
        }
    }
}

const FLOOR_OFFSET: usize = 2;

#[aoc(day14, part2)]
fn solve_part2(cave: &Cave) -> usize {
    let mut cave = cave.clone();
    let side_expand = cave.true_dim().1;
    cave.expand(0, FLOOR_OFFSET, side_expand, side_expand, Tile::Air);

    let floor_height = cave.limits.1 .1;
    cave.grid.slice_mut(s![.., floor_height]).fill(Tile::Rock);

    let mut particles = 0;
    loop {
        let mut current = SAND_SRC;

        // Let it keep falling
        while let SandFall::Falls(next) = fall_from(&cave, current) {
            current = next;
        }

        match fall_from(&cave, current) {
            SandFall::Rests => {
                cave[current] = Tile::Sand;
                particles += 1;
            }
            SandFall::LostToAbyss => {
                panic!(
                    "Particle #{} has fallen into the void from {:?}, limits {:?} {:?}",
                    particles + 1,
                    current,
                    cave.limits.0,
                    cave.limits.1,
                );
            }
            SandFall::Falls(_) => unreachable!(),
        }

        // There's sand blocking the spawn point
        if cave[SAND_SRC] == Tile::Sand {
            return particles;
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;

    const SAMPLE_INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), 24);
    }

    #[test]
    fn part1_mine() {
        assert_eq!(solve_part1(&generate(&crate::get_input(14))), 994);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&generate(SAMPLE_INPUT)), 93);
    }

    #[test]
    fn part2_mine() {
        assert_eq!(solve_part2(&generate(&crate::get_input(14))), 26283);
    }
}
