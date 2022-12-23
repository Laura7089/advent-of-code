#![allow(dead_code)]
#![allow(unused_variables)]

use crate::{adjacents_filtered, Point};
use ndarray::Array2;

#[derive(Clone, Debug)]
struct Field {
    map: Array2<u8>,
    start: Point,
    end: Point,
}

#[aoc_generator(day12)]
fn generate(input: &str) -> Field {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let mut map = Array2::zeros((height, width));
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.bytes().enumerate() {
            map[(i, j)] = match c {
                b'S' => {
                    start = (i, j);
                    b'a'
                }
                b'E' => {
                    end = (i, j);
                    b'z'
                }
                _ => c,
            } - b'a';
        }
    }
    Field { map, start, end }
}

#[aoc(day12, part1)]
fn solve_part1(field: &Field) -> usize {
    let (w, h) = field.map.dim();
    let mut visited = Vec::with_capacity((w * h) / 2);
    visited.push(field.start);
    next_step(field, field.start, field.start, 0, &mut visited).unwrap()
}

fn next_step(
    field: &Field,
    prev: Point,
    current: Point,
    dist: usize,
    visited: &mut [Point],
) -> Option<usize> {
    let current_val = field.map[current];

    let is_valid = |point: Point| {
        field.map[point] >= current_val
            && field.map[point].abs_diff(current_val) <= 1
            && !visited.contains(&point)
    };

    adjacents_filtered::<4>(current, field.map.dim())
        .filter_map(|i| {
            if !is_valid(i) {
                None
            } else if i == field.end {
                Some(dist + 1)
            } else {
                let mut visited = visited.to_owned();
                visited.push(i);
                next_step(field, current, i, dist + 1, &mut visited)
            }
        })
        .min()
}

#[aoc(day12, part2)]
fn solve_part2(field: &Field) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;

    const SAMPLE_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn part1_sanity() {
        assert_eq!(solve_part1(&generate("SbcdefghijklmnopqrstuvwxyE")), 25);
    }

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), 31);
    }

    #[test]
    fn part1_mine() {
        assert_eq!(solve_part1(&generate(&crate::get_input(12))), todo!());
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&generate(SAMPLE_INPUT)), todo!());
    }

    #[test]
    fn part2_mine() {
        assert_eq!(solve_part2(&generate(&crate::get_input(12))), todo!());
    }
}
