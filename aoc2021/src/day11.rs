use crate::compressed_field::{CompressedField, Idx};
use std::collections::HashSet;

const DIMENSIONS: (usize, usize) = (10, 10);
const NUM_STEPS: usize = 100;

fn flash(idx: Idx, field: &mut CompressedField<u8>, flashed: &mut HashSet<Idx>) -> usize {
    let mut flashes = 0;

    if field[idx] > 9 && !flashed.contains(&idx) {
        // Flash!
        field[idx] = 0;
        flashed.insert(idx);
        flashes += 1;

        // Increment and check adjacents
        for adj in field.adjacents_diag(idx).into_iter().filter_map(|i| i) {
            if !flashed.contains(&adj) {
                field[adj] += 1;
            }
            flashes += flash(adj, field, flashed);
        }
    }

    flashes
}

#[aoc_generator(day11)]
fn parse_input(input: &str) -> CompressedField<u8> {
    CompressedField {
        map: input.lines().flat_map(str::bytes).map(|n| n - 48).collect(),
        row_len: DIMENSIONS.0,
    }
}

#[aoc(day11, part1)]
fn solve_part1(input: &CompressedField<u8>) -> usize {
    let mut field = input.clone();
    let mut flashes = 0;

    for day in 1..=NUM_STEPS {
        // Increment all octopi
        field.map.iter_mut().for_each(|n| *n += 1);

        let mut flashed = HashSet::new();

        for x in 0..DIMENSIONS.0 {
            for y in 0..DIMENSIONS.1 {
                flashes += flash((x, y), &mut field, &mut flashed);
            }
        }
    }

    flashes
}

#[aoc(day11, part2)]
fn solve_part2(input: &CompressedField<u8>) -> usize {
    let mut field = input.clone();
    let mut flashed = HashSet::new();
    let mut day = 0;

    while flashed.len() != DIMENSIONS.0 * DIMENSIONS.1 {
        // Set up the day
        flashed = HashSet::new();
        day += 1;

        // Increment all octopi
        field.map.iter_mut().for_each(|n| *n += 1);

        for x in 0..DIMENSIONS.0 {
            for y in 0..DIMENSIONS.1 {
                flash((x, y), &mut field, &mut flashed);
            }
        }
    }

    day
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&parse_input(&EXAMPLE_INPUT)), 1656);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&parse_input(&EXAMPLE_INPUT)), 195);
    }

    #[test]
    fn part1_myinput() {
        let _input = crate::get_input_for_day(11);
        assert_eq!(solve_part1(&parse_input(&_input)), 1659);
    }

    #[test]
    fn part2_myinput() {
        let _input = crate::get_input_for_day(11);
        assert_eq!(solve_part2(&parse_input(&_input)), 227);
    }
}
