use std::collections::BTreeSet;

use crate::grid::Point;

type TopoMap = crate::grid::Grid<u8, crate::grid::Orthogonal>;

#[aoc_generator(day10)]
fn generate(input: &str) -> TopoMap {
    TopoMap::new(
        input
            .lines()
            .map(|line| line.bytes().map(|b| b - b'0').collect())
            .collect(),
    )
}

fn get_score_p1(map: &TopoMap, trailhead: Point, visited: &mut BTreeSet<Point>) -> usize {
    let t_height = map[trailhead];

    // base case
    if t_height == 9 && !visited.contains(&trailhead) {
        visited.insert(trailhead);
        return 1;
    }

    // recursive case(s)
    map.neighbours(trailhead)
        .filter(|(_, &h)| (h == t_height + 1))
        .map(|(p, _)| get_score_p1(map, p, visited))
        .sum()
}

#[aoc(day10, part1)]
fn solve_part1(input: &TopoMap) -> usize {
    #[allow(clippy::filter_map_bool_then)]
    input
        .iter_all()
        .filter_map(|(p, &sq)| (sq == 0).then(|| get_score_p1(input, p, &mut BTreeSet::new())))
        .sum()
}

fn get_score_p2(map: &TopoMap, trailhead: Point) -> usize {
    let t_height = map[trailhead];

    // base case
    if t_height == 9 {
        return 1;
    }

    // recursive case(s)
    map.neighbours(trailhead)
        .filter(|(_, &h)| (h == t_height + 1))
        .map(|(p, _)| get_score_p2(map, p))
        .sum()
}

#[aoc(day10, part2)]
fn solve_part2(input: &TopoMap) -> usize {
    #[allow(clippy::filter_map_bool_then)]
    input
        .iter_all()
        .filter_map(|(p, &sq)| (sq == 0).then(|| get_score_p2(input, p)))
        .sum()
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;

    const SAMPLE_INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), 36);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part1(&generate(&crate::get_input(10))), 841);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part2(&generate(SAMPLE_INPUT)), 81);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part2(&generate(&crate::get_input(10))), 1875);
        }
    }
}
