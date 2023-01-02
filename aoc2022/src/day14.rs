use crate::{OffsetGrid, UPoint as Point};
use itertools::Itertools;
use ndarray::s;

type Cave = OffsetGrid<Tile>;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Tile {
    Air,
    Rock,
    Sand,
}

impl From<Tile> for char {
    fn from(value: Tile) -> Self {
        match value {
            Tile::Air => '.',
            Tile::Rock => '#',
            Tile::Sand => 'o',
        }
    }
}

fn fall_from(cave: &Cave, (sx, y): Point) -> Result<Option<Point>, ()> {
    for x in [sx, sx - 1, sx + 1] {
        let next = (x, y + 1);
        if !cave.contains_vert(next.1) {
            return Err(());
        }
        if cave[next] == Tile::Air {
            return Ok(Some(next));
        }
    }
    Ok(None)
}

mod parse {
    use crate::IResult;
    use nom::{
        bytes::complete::tag,
        character::complete::{char, u32},
        multi::separated_list1,
        sequence::separated_pair,
    };

    fn point(input: &str) -> IResult<super::Point> {
        let (i, (x, y)) = separated_pair(u32, char(','), u32)(input)?;
        Ok((i, (x as usize, y as usize)))
    }

    pub fn stratum(input: &str) -> IResult<Vec<super::Point>> {
        separated_list1(tag(" -> "), point)(input)
    }
}

const SAND_SRC: Point = (500, 0);
// TODO: tweak maths so this can be removed
const HALO: usize = 2;

#[aoc_generator(day14)]
fn generate(input: &str) -> Cave {
    let strata: Vec<Vec<Point>> = input
        .lines()
        .map(|line| parse::stratum(line).unwrap().1)
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

    let mut cave = Cave::new(
        (x0 - HALO, SAND_SRC.1.saturating_sub(HALO)),
        (x1 + HALO, y1 + HALO),
        Tile::Air,
    );

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

#[aoc(day14, part1)]
fn solve_part1(cave: &Cave) -> usize {
    let mut cave = cave.clone();
    let mut particles = 0;

    let sand_spawn = (SAND_SRC.0, SAND_SRC.1 + 1);
    loop {
        let mut current = sand_spawn.clone();
        while let Ok(Some(next)) = fall_from(&cave, current) {
            current = next;
        }
        if fall_from(&cave, current) == Ok(None) {
            // Sand has come to rest
            cave[current] = Tile::Sand;
            particles += 1;
        } else {
            // It has fallen into the Å̸̉͊̂̇̈́̃ͣ҉̘̮̳̫̤͠B̶̢͓̤̠̜̯͚̘̮̟͖͎̄̓̆̽̀͑ͭ̇̕͜͝ͅY͒ͤ͆͐͌͆ͨͦ̌̚͏̹̮̞̲̼̼͉̭̮̪͜͡S̸̛̘̯̲̭̊̃̀̓ͥ̔͝S̡̭̥̖̭͙̼͓͔͎̭̬̭͕̹͉̯̗ͫͨͭ͑͛̐ͮ̊̔̊ͮ͂̓͡
            break;
        }
    }

    particles
}

const FLOOR_OFFSET: usize = 2;

#[aoc(day14, part2)]
fn solve_part2(cave: &Cave) -> usize {
    #[allow(clippy::trivially_copy_pass_by_ref)]
    fn is_rock(tile: &Tile) -> bool {
        tile == &Tile::Rock
    }

    let cave_old = cave.clone();

    let extend_l = cave_old
        .grid
        .rows()
        .into_iter()
        .enumerate()
        // TODO: think we actually need to iterate from the top
        .find(|(_, r)| !r.iter().any(is_rock))
        .unwrap()
        .0
        - HALO;
    let extend_r = cave_old.grid.nrows()
        - cave_old
            .grid
            .rows()
            .into_iter()
            .enumerate()
            .skip(HALO)
            .find(|(_, r)| !r.iter().any(is_rock))
            .unwrap()
            .0;
    let (mut topleft, mut bottomright) = cave.limits;
    topleft.0 -= extend_l;
    bottomright.0 += extend_r;
    bottomright.1 += FLOOR_OFFSET;

    let mut cave = Cave::new(topleft, bottomright, Tile::Air);
    cave.grid
        .slice_mut(s![
            (extend_l as isize)..-(extend_r as isize),
            ..-(FLOOR_OFFSET as isize)
        ])
        .assign(&cave_old.grid);

    let floor = cave_old
        .grid
        .columns()
        .into_iter()
        .enumerate()
        // Skip air at the start
        .skip_while(|(_, c)| !c.iter().any(is_rock))
        // Find the next line which is just air again
        .find(|(_, c)| !c.iter().any(is_rock))
        .unwrap()
        .0
        + FLOOR_OFFSET;
    (topleft.0..bottomright.0).for_each(|x| cave[(x, floor)] = Tile::Rock);

    let mut particles = 0;

    let sand_spawn = (SAND_SRC.0, SAND_SRC.1 + 1);
    loop {
        let mut current = sand_spawn.clone();
        while let Ok(Some(next)) = fall_from(&cave, current) {
            current = next;
        }
        if fall_from(&cave, current) == Ok(None) {
            // Sand has come to rest
            cave[current] = Tile::Sand;
            particles += 1;
        }
        if cave[sand_spawn] == Tile::Sand {
            break;
        }
    }

    particles
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
        assert_eq!(solve_part2(&generate(&crate::get_input(14))), todo!());
    }
}
