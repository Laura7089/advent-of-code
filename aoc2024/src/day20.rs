use crate::grid::{Grid, Point};
use crate::iter_ext::IterExt;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Square {
    Wall,
    Space,
}

#[aoc_generator(day20)]
fn generate(input: &str) -> Vec<Point> {
    let mut start = (0, 0);
    let mut end = (0, 0);

    let mut grid = Vec::new();
    for (y, line) in input.lines().enumerate() {
        let mut parsed_line = Vec::with_capacity(line.len());
        for (x, c) in line.chars().enumerate() {
            parsed_line.push(match c {
                '#' => Square::Wall,
                '.' => Square::Space,
                'S' => {
                    start = (x, y);
                    Square::Space
                }
                'E' => {
                    end = (x, y);
                    Square::Space
                }
                _ => panic!("unsupported square char {c}"),
            })
        }
        grid.push(parsed_line);
    }

    let maze: Grid<Square, crate::grid::Orthogonal> = Grid::new(grid);
    let mut visited = Vec::new();
    let mut current = start;

    while current != end {
        let last = *visited.last().unwrap_or(&start);
        let next = maze
            .neighbours(current)
            .find(|&(p, sq)| p != last && sq != &Square::Wall)
            .expect("found a dead end")
            .0;
        visited.push(current);
        current = next;
    }

    visited
}

const LOOK_AHEAD: usize = 4;
const MIN_SAVE: usize = 100;

fn count_cheats<const MAX_CHEAT_LEN: usize>(route: &[Point]) -> usize {
    route
        .iter()
        .enumerate()
        .cart_prod_with(|(start, _)| route.iter().enumerate().skip(start + LOOK_AHEAD))
        .filter(|((skip_start, &(sx, sy)), (skip_end, &(ex, ey)))| {
            let cheat_len = sx.abs_diff(ex) + sy.abs_diff(ey);
            if cheat_len > MAX_CHEAT_LEN {
                return false;
            }

            let skipped_len = skip_end - skip_start;
            let time_saved = skipped_len.saturating_sub(cheat_len);
            time_saved >= MIN_SAVE
        })
        .count()
}

#[aoc(day20, part1)]
fn solve_part1(route: &Vec<Point>) -> usize {
    count_cheats::<2>(route)
}

#[aoc(day20, part2)]
fn solve_part2(route: &Vec<Point>) -> usize {
    count_cheats::<20>(route)
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn mine() {
            assert_eq!(solve_part1(&generate(&crate::get_input(20))), 1445);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn mine() {
            assert_eq!(solve_part2(&generate(&crate::get_input(20))), todo!());
        }
    }
}
