use crate::compressed_field::CompressedField;
use std::collections::HashSet;
use std::ops::Mul;

const NUM_LARGEST: usize = 3;

pub fn low_points(field: &CompressedField<usize>) -> Vec<(usize, usize)> {
    let mut low_points = Vec::with_capacity(field.height());

    for y in 0..field.height() {
        for x in 0..field.row_len {
            let current = field[(x, y)];
            let is_low_point = field
                .adjacents((x, y))
                .iter()
                .filter_map(|p| *p)
                .all(|point| field[point] > current);

            if is_low_point {
                low_points.push((x, y));
            }
        }
    }

    low_points
}

#[aoc_generator(day9)]
fn parse_input(input: &str) -> CompressedField<usize> {
    let row_len = input.lines().next().unwrap().len();

    CompressedField {
        row_len,
        map: input
            .lines()
            .flat_map(str::bytes)
            .map(|p| p as usize - 48)
            .collect(),
    }
}

#[aoc(day9, part1)]
fn solve_part1(input: &CompressedField<usize>) -> usize {
    let low_points = low_points(input);
    low_points.iter().map(|i| input[*i]).sum::<usize>() + low_points.len()
}

#[aoc(day9, part2)]
fn solve_part2(input: &CompressedField<usize>) -> usize {
    let mut largest_sizes = [0_usize; NUM_LARGEST];

    for (lx, ly) in low_points(input).into_iter() {
        // Note: Use the field width as a capacity so it scales correctly
        // Stores the final basin
        let mut basin = HashSet::with_capacity(input.row_len);
        // Stores the uninspected edges
        let mut basin_edge = Vec::with_capacity(input.row_len);
        basin.insert((lx, ly));
        basin_edge.push((lx, ly));

        // Look through the outer edge of the basin
        while let Some((x, y)) = basin_edge.pop() {
            let current_val = input[(x, y)];

            // Iterate through the squares adjacent to it
            for adj in input.adjacents((x, y)).into_iter().filter_map(|p| p) {
                let adj_val = input[adj];
                // If:
                // - it's not 9
                // - it's higher than the current square
                // - we haven't already recorded it
                if adj_val != 9 && current_val < adj_val && !basin.contains(&adj) {
                    basin.insert(adj);
                    basin_edge.push(adj);
                }
            }
        }

        // Add it to the array of largest sizes if it's larger than any of them
        largest_sizes.sort();
        for n in largest_sizes.iter_mut() {
            if &basin.len() > n {
                *n = basin.len();
                break;
            } else {
            }
        }
    }

    largest_sizes.into_iter().reduce(usize::mul).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&parse_input(&EXAMPLE_INPUT)), 15);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&parse_input(&EXAMPLE_INPUT)), 1134);
    }

    #[test]
    fn part1_myinput() {
        let input = crate::get_input_for_day(9);
        assert_eq!(solve_part1(&parse_input(&input)), 475);
    }

    #[test]
    fn part2_myinput() {
        let _input = crate::get_input_for_day(9);
        assert_eq!(solve_part2(&parse_input(&_input)), 1092012);
    }
}
