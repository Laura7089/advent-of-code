use ndarray::{Array2, Axis};

#[aoc_generator(day8)]
fn generate(input: &str) -> Array2<u8> {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let mut array = Array2::zeros((width, height));

    for (j, row) in input.lines().enumerate() {
        for (i, tree) in row.bytes().enumerate() {
            array[[i, j]] = tree - b'0';
        }
    }

    array
}

#[aoc(day8, part1)]
fn solve_part1(input: &Array2<u8>) -> usize {
    let mut total = 0;

    for column in input.axis_iter(Axis(0)) {
        // Top to bottom
        let mut last_height = 0;
        for tree in column {
            if last_height < *tree {
                total += 1;
            }
            last_height = *tree;
        }

        // Bottom to top
        let mut last_height = 0;
        for tree in column.iter().rev() {
            if last_height < *tree {
                total += 1;
            }
            last_height = *tree;
        }
    }

    for row in input.axis_iter(Axis(1)) {
        // Left to right
        let mut last_height = 0;
        for tree in row {
            if last_height < *tree {
                total += 1;
            }
            last_height = *tree;
        }

        // Right to left
        let mut last_height = 0;
        for tree in row.iter().rev() {
            if last_height < *tree {
                total += 1;
            }
            last_height = *tree;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), 21);
    }
}
