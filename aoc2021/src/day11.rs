use crate::field2d::{array_field::ArrayField, Field2D, Idx};

const WIDTH: usize = 10;
const HEIGHT: usize = 10;
const NUM_STEPS: usize = 100;

type Field = ArrayField<u8, WIDTH, HEIGHT>;

fn try_flash(idx: Idx, field: &mut Field) -> usize {
    let mut flashes = 0;

    if field[idx] > 9 {
        // Flash!
        field[idx] = 0;
        flashes += 1;

        // Increment and check adjacents
        // NOTE: flatten instead of filter_map is a perf regression
        for adj in field.adjacents(idx).into_iter().filter_map(|i| i) {
            // Don't inc things that have already flashed
            if field[adj] != 0 {
                field[adj] += 1;
            }
            flashes += try_flash(adj, field);
        }
    }

    flashes
}

#[aoc_generator(day11)]
pub fn parse_input(input: &str) -> Field {
    Field {
        field: input
            .lines()
            .map(|l| {
                l.bytes()
                    .map(|n| n - 48)
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
    }
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &Field) -> usize {
    let mut field = input.clone();
    let mut flashes = 0;

    for _ in 1..=NUM_STEPS {
        // Increment all octopi
        field
            .field
            .iter_mut()
            .flat_map(|l| l.iter_mut())
            .for_each(|n| *n += 1);

        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                flashes += try_flash((x, y), &mut field);
            }
        }
    }

    flashes
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &Field) -> usize {
    let mut field = input.clone();
    let mut day = 0;

    // Apparently this is faster than a hashmap
    while !field
        .field
        .iter()
        .flat_map(|c| c.iter())
        .all(|n| n == &0_u8)
    {
        day += 1;

        // Increment all octopi
        field
            .field
            .iter_mut()
            .flat_map(|l| l.iter_mut())
            .for_each(|n| *n += 1);

        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                try_flash((x, y), &mut field);
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
