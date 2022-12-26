#![allow(dead_code)]
#![allow(unused_variables)]

use crate::{adjacents_filtered, Point};
use ndarray::Array2;
use pathfinding::prelude::dijkstra;

#[derive(Clone, Debug)]
struct Field {
    map: Array2<u8>,
    start: Point,
    end: Point,
}

impl Field {
    fn new(input: &str) -> Self {
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
        Self { map, start, end }
    }

    fn valid_moves<'a>(&'a self, pos: Point) -> impl Iterator<Item = Point> + 'a {
        adjacents_filtered::<4>(pos, self.map.dim())
            .filter(move |i| self.map[*i].saturating_sub(self.map[pos]) <= 1)
    }

    fn path_start_end(&self) -> Option<(Vec<Point>, usize)> {
        self.path_from(self.start)
    }

    fn path_from(&self, start: Point) -> Option<(Vec<Point>, usize)> {
        dijkstra(
            &start,
            // We don't care about the weighting so it's always 1
            |p| self.valid_moves(*p).map(|p| (p, 1)),
            |p| p == &self.end,
        )
    }
}

#[aoc_generator(day12)]
fn generate(input: &str) -> Field {
    Field::new(input)
}

#[aoc(day12, part1)]
fn solve_part1(field: &Field) -> usize {
    field.path_start_end().unwrap().1
}

// // TODO: is a hashset faster?
// fn next_step(field: &Field, cur: Point, dist: usize, visited: &[Point]) -> Option<usize> {
//     adjacents_filtered::<4>(cur, field.map.dim())
//         .filter_map(|i| {
//             let height_diff = field.map[i].saturating_sub(field.map[cur]);
//             if height_diff > 1 || visited.contains(&i) {
//                 None
//             } else if i == field.end {
//                 // Yay, we made it!
//                 Some(dist + 1)
//             } else {
//                 // Recurse, and record that we visited the current place
//                 let mut visited = visited.to_owned();
//                 visited.push(i);
//                 next_step(field, i, dist + 1, &visited)
//             }
//         })
//         .min()
// }

#[aoc(day12, part2)]
fn solve_part2(field: &Field) -> usize {
    field
        .map
        .indexed_iter()
        // Find all the a's
        .filter_map(|(i, &e)| if e == 0 { Some(i) } else { None })
        // Ignore anything that doesn't find a path
        .filter_map(|i| field.path_from(i).map(|(p, n)| n))
        .min()
        .unwrap()
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
        assert_eq!(solve_part1(&generate(&crate::get_input(12))), 517);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&generate(SAMPLE_INPUT)), 29);
    }

    #[test]
    fn part2_mine() {
        assert_eq!(solve_part2(&generate(&crate::get_input(12))), 512);
    }
}
