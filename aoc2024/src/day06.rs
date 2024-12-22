use std::cmp::{max_by_key, min_by_key};
use std::collections::BTreeSet;

type Point = (usize, usize);

#[aoc_generator(day06)]
fn generate(input: &str) -> (Field, Point) {
    let mut obstacles = BTreeSet::new();
    let mut start = None;

    for (y, line) in input.lines().rev().enumerate() {
        for (x, ch) in line.bytes().enumerate() {
            match ch {
                b'#' => {
                    obstacles.insert((x, y));
                }
                b'^' => start = Some((x, y)),
                _ => {}
            }
        }
    }

    (
        Field {
            obstacles,
            height: input.lines().count(),
            width: input.lines().next().unwrap().len(),
        },
        start.expect("no start found"),
    )
}

struct Field {
    width: usize,
    height: usize,
    obstacles: BTreeSet<Point>,
}

impl Field {
    fn visited_spaces(&self, start: &Point) -> BTreeSet<Point> {
        fn insert_vert(visited: &mut BTreeSet<Point>, x: usize, min: usize, max: usize) {
            for y in min..max {
                visited.insert((x, y));
            }
        }

        fn insert_hor(visited: &mut BTreeSet<Point>, y: usize, min: usize, max: usize) {
            for x in min..max {
                visited.insert((x, y));
            }
        }

        let mut visited = BTreeSet::new();
        let mut pos = *start;

        loop {
            // up
            let hit = self
                .obstacles
                .iter()
                .filter(|&&(x, y)| x == pos.0 && y > pos.1)
                .reduce(|min, point| min_by_key(min, point, |(_x, y)| y));
            if let Some(&(_, hit_y)) = hit {
                insert_vert(&mut visited, pos.0, pos.1, hit_y);
                pos.1 = hit_y - 1;
            } else {
                insert_vert(&mut visited, pos.0, pos.1, self.height);
                break;
            }

            // right
            let hit = self
                .obstacles
                .iter()
                .filter(|&&(x, y)| x > pos.0 && y == pos.1)
                .reduce(|cur_min, point| min_by_key(cur_min, point, |(x, _y)| x));
            if let Some(&(hit_x, _)) = hit {
                insert_hor(&mut visited, pos.1, pos.0, hit_x);
                pos.0 = hit_x - 1;
            } else {
                insert_hor(&mut visited, pos.1, pos.0, self.width);
                break;
            }

            // down
            let hit = self
                .obstacles
                .iter()
                .filter(|&&(x, y)| x == pos.0 && y < pos.1)
                .reduce(|cur_min, point| max_by_key(cur_min, point, |(_x, y)| y));
            if let Some(&(_, hit_y)) = hit {
                insert_vert(&mut visited, pos.0, hit_y + 1, pos.1);
                pos.1 = hit_y + 1;
            } else {
                insert_vert(&mut visited, pos.0, 0, pos.1);
                break;
            };

            // left
            let hit = self
                .obstacles
                .iter()
                .filter(|&&(x, y)| x < pos.0 && y == pos.1)
                .reduce(|cur_min, point| max_by_key(cur_min, point, |(x, _y)| x));
            if let Some(&(hit_x, _)) = hit {
                insert_hor(&mut visited, pos.1, hit_x + 1, pos.0);
                pos.0 = hit_x + 1;
            } else {
                insert_hor(&mut visited, pos.1, 0, pos.0);
                break;
            }
        }

        visited
    }
}

#[aoc(day06, part1)]
fn solve_part1((field, start): &(Field, Point)) -> usize {
    field.visited_spaces(start).len()
}

#[aoc(day06, part2)]
fn solve_part2(_input: &(Field, Point)) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;

    const SAMPLE_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), 41);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part1(&generate(&crate::get_input(06))), 5534);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part2(&generate(SAMPLE_INPUT)), 6);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part2(&generate(&crate::get_input(06))), todo!());
        }
    }
}
