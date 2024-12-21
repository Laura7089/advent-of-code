use crate::grid::{Grid, Point};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Square {
    Wall,
    Space,
}

type Maze = Grid<Square>;

#[aoc_generator(day20)]
fn generate(input: &str) -> (Maze, Point, Point) {
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

    (Grid::new(grid), start, end)
}

fn route_from_maze(maze: &Maze, start: Point, end: Point) -> Vec<Point> {
    let mut visited = Vec::new();
    let mut current = start;

    while current != end {
        let last = *visited.last().unwrap_or(&start);
        let next = maze
            .adj_coords_orth(current)
            .find(|p| p != &last && maze[*p] != Square::Wall)
            .expect("found a dead end");
        visited.push(current);
        current = next;
    }

    visited
}

const LOOK_AHEAD: usize = 4;
const MIN_SKIP: usize = 100;

#[aoc(day20, part1)]
fn solve_part1((maze, start, end): &(Maze, Point, Point)) -> usize {
    let route = route_from_maze(maze, *start, *end);

    let mut count = 0;
    for (skip_start, &(sx, sy)) in route.iter().enumerate() {
        for (skip_end, &(ex, ey)) in route.iter().enumerate().skip(skip_start + LOOK_AHEAD) {
            let mut jumps = [sx.abs_diff(ex), sy.abs_diff(ey)];
            jumps.sort_unstable();
            if jumps == [0, 2] {
                let skip = skip_end - skip_start - 1;
                if skip >= MIN_SKIP {
                    count += 1;
                }
            }
        }
    }

    count
}

#[aoc(day20, part2)]
fn solve_part2(_input: &(Maze, Point, Point)) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;

    const SAMPLE_INPUT: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

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
        fn example() {
            assert_eq!(solve_part2(&generate(SAMPLE_INPUT)), todo!());
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part2(&generate(&crate::get_input(20))), todo!());
        }
    }
}
