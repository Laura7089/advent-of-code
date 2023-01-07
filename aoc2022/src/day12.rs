use aoc_helpers::{Adjacents, UPoint as Point};
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

        let mut to_return = Self {
            map: Array2::zeros((height, width)),
            start: (0, 0),
            end: (0, 0),
        };

        for (i, line) in input.lines().enumerate() {
            for (j, c) in line.bytes().enumerate() {
                to_return.map[(i, j)] = match c {
                    b'S' => {
                        to_return.start = (i, j);
                        b'a'
                    }
                    b'E' => {
                        to_return.end = (i, j);
                        b'z'
                    }
                    _ => c,
                } - b'a';
            }
        }
        to_return
    }

    fn valid_moves(&self, pos: Point) -> impl Iterator<Item = Point> + '_ {
        Adjacents::<4>::new(pos)
            .constrain(self.map.dim())
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

#[aoc(day12, part2)]
fn solve_part2(field: &Field) -> usize {
    field
        .map
        .indexed_iter()
        // Find all the a's
        .filter_map(|(i, &e)| if e == 0 { Some(i) } else { None })
        // Ignore anything that doesn't find a path
        .filter_map(|i| field.path_from(i).map(|(_, n)| n))
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
