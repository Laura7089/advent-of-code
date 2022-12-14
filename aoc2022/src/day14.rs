use itertools::Itertools;
use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use ndarray::Array2;

type Point = (usize, usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Tile {
    Air,
    Rock,
    Sand,
}

impl Tile {
    fn into_char(&self) -> char {
        match self {
            Tile::Air => '.',
            Tile::Rock => '#',
            Tile::Sand => 'o',
        }
    }
}

#[derive(Clone, Debug)]
struct Cave {
    grid: Array2<Tile>,
    /// Top-left, bottom-right
    limits: (Point, Point),
}

impl Cave {
    fn fall_from(&self, (sx, y): Point) -> Result<Option<Point>, ()> {
        for x in [sx, sx - 1, sx + 1] {
            let next = (x, y + 1);
            if !self.contains(next) {
                return Err(());
            }
            if self[next] == Tile::Air {
                return Ok(Some(next));
            }
        }
        Ok(None)
    }

    fn contains(&self, (_, y): Point) -> bool {
        ((self.limits.0 .1)..(self.limits.1 .1)).contains(&y)
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let full: String = self
            .grid
            .columns()
            .into_iter()
            .map(|column| {
                column
                    .iter()
                    .map(Tile::into_char)
                    .chain(['\n'].into_iter())
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect();
        write!(f, "{full}")
    }
}

impl Index<Point> for Cave {
    type Output = Tile;
    fn index(&self, mut index: Point) -> &Self::Output {
        index.0 -= self.limits.0 .0;
        index.1 -= self.limits.0 .1;
        &self.grid[index]
    }
}

impl IndexMut<Point> for Cave {
    fn index_mut(&mut self, mut index: Point) -> &mut Self::Output {
        index.0 -= self.limits.0 .0;
        index.1 -= self.limits.0 .1;
        &mut self.grid[index]
    }
}

const SAND_SOURCE: Point = (500, 0);

#[aoc_generator(day14)]
fn generate(input: &str) -> Cave {
    let strata: Vec<Vec<Point>> = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|pair| {
                    let mut pair = pair.split(",");
                    (
                        pair.next().unwrap().parse().unwrap(),
                        pair.next().unwrap().parse().unwrap(),
                    )
                })
                .collect()
        })
        .collect();

    // TODO: this is fucking awful
    let (minx, maxx) = strata
        .iter()
        .flatten()
        .map(|p| p.0)
        .minmax()
        .into_option()
        .unwrap();
    let (miny, maxy) = strata
        .iter()
        .flatten()
        .map(|p| p.1)
        .minmax()
        .into_option()
        .unwrap();
    let minx = minx.min(SAND_SOURCE.0);
    let maxx = maxx.max(SAND_SOURCE.0) + 2;
    let miny = miny.min(SAND_SOURCE.1);
    let maxy = maxy.max(SAND_SOURCE.1) + 2;

    let mut cave = Cave {
        grid: Array2::from_elem((maxx - minx, maxy - miny), Tile::Air),
        limits: ((minx, miny), (maxx, maxy)),
    };

    for stratum in strata {
        let mut stratum = stratum.into_iter();
        let mut last_point = stratum.next().unwrap();
        for (px, py) in stratum {
            if px != last_point.0 {
                // horizontal
                for x in px.min(last_point.0)..=px.max(last_point.0) {
                    cave[(x, py)] = Tile::Rock;
                }
            } else {
                // vertical
                for y in py.min(last_point.1)..=py.max(last_point.1) {
                    cave[(px, y)] = Tile::Rock;
                }
            }
            last_point = (px, py);
        }
    }

    cave
}

#[aoc(day14, part1)]
fn solve_part1(cave: &Cave) -> usize {
    let mut cave = cave.clone();
    let mut particles = 0;

    let sand_spawn = (SAND_SOURCE.0, SAND_SOURCE.1 + 1);
    loop {
        let mut current = sand_spawn.clone();
        while let Ok(Some(next)) = cave.fall_from(current) {
            current = next;
        }
        match cave.fall_from(current) {
            Ok(None) => {
                cave[current] = Tile::Sand;
                particles += 1;
            }
            // It has fallen into the Å̸̉͊̂̇̈́̃ͣ҉̘̮̳̫̤͠B̶̢͓̤̠̜̯͚̘̮̟͖͎̄̓̆̽̀͑ͭ̇̕͜͝ͅY͒ͤ͆͐͌͆ͨͦ̌̚͏̹̮̞̲̼̼͉̭̮̪͜͡S̸̛̘̯̲̭̊̃̀̓ͥ̔͝S̡̭̥̖̭͙̼͓͔͎̭̬̭͕̹͉̯̗ͫͨͭ͑͛̐ͮ̊̔̊ͮ͂̓͡
            Err(_) => break,
            _ => unreachable!(),
        }
    }

    println!("{cave}");
    particles
}

#[aoc(day14, part2)]
fn solve_part2(input: &Cave) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), 24);
    }

    #[test]
    fn part1_mine() {
        assert_eq!(solve_part1(&generate(&crate::get_input(14))), todo!());
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&generate(SAMPLE_INPUT)), todo!());
    }

    #[test]
    fn part2_mine() {
        assert_eq!(solve_part2(&generate(&crate::get_input(14))), todo!());
    }
}
