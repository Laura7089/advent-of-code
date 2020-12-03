use std::ops::Index;

const TREE_CHAR: u8 = '#' as u8;

#[derive(Debug, PartialEq)]
pub struct ForestedSlope {
    grid: Vec<Vec<bool>>,
    pub length: usize,
    pub width: usize,
}

impl Index<(usize, usize)> for ForestedSlope {
    type Output = bool;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        if y >= self.length {
            panic!("Index out of bounds - off the end of the slope");
        }
        &self.grid[y][x % self.width]
    }
}

impl ForestedSlope {
    pub fn from(input: &str) -> Self {
        let grid: Vec<Vec<bool>> = input
            .lines()
            .map(|l| l.as_bytes().iter().map(|s| s == &TREE_CHAR).collect())
            .collect();
        Self {
            length: grid.len(),
            width: grid[0].len(),
            grid,
        }
    }

    pub fn trees_along_slope(&self, (xmul, ymul): &(usize, usize)) -> usize {
        (0..self.length / ymul)
            .filter(|&i| self[(i * xmul, i * ymul)])
            .count()
    }
}

#[aoc_generator(day3)]
pub fn get_slope(input: &str) -> ForestedSlope {
    ForestedSlope::from(input)
}

#[aoc(day3, part1)]
pub fn solve_input_part1(input: &ForestedSlope) -> usize {
    input.trees_along_slope(&(3, 1))
}

#[aoc(day3, part2)]
pub fn solve_input_part2(input: &ForestedSlope) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .fold(1, |prev, slope| prev * input.trees_along_slope(slope))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const EXAMPLE_STRING: &'static str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn parser_example() {
        let expected_grid = vec![
            vec![
                false, false, true, true, false, false, false, false, false, false, false,
            ],
            vec![
                true, false, false, false, true, false, false, false, true, false, false,
            ],
            vec![
                false, true, false, false, false, false, true, false, false, true, false,
            ],
            vec![
                false, false, true, false, true, false, false, false, true, false, true,
            ],
            vec![
                false, true, false, false, false, true, true, false, false, true, false,
            ],
            vec![
                false, false, true, false, true, true, false, false, false, false, false,
            ],
            vec![
                false, true, false, true, false, true, false, false, false, false, true,
            ],
            vec![
                false, true, false, false, false, false, false, false, false, false, true,
            ],
            vec![
                true, false, true, true, false, false, false, true, false, false, false,
            ],
            vec![
                true, false, false, false, true, true, false, false, false, false, true,
            ],
            vec![
                false, true, false, false, true, false, false, false, true, false, true,
            ],
        ];

        assert_eq!(
            get_slope(EXAMPLE_STRING),
            ForestedSlope {
                grid: expected_grid,
                width: 11,
                length: 11,
            }
        );
    }

    #[test_case(get_slope(EXAMPLE_STRING), 7)]
    #[test_case(get_slope(EXAMPLE_STRING), 6 => panics "")]
    fn part1(input: ForestedSlope, output: usize) {
        assert_eq!(solve_input_part1(&input), output);
    }

    #[test_case(get_slope(EXAMPLE_STRING), 336)]
    #[test_case(get_slope(EXAMPLE_STRING), 335 => panics "")]
    fn part2(input: ForestedSlope, output: usize) {
        assert_eq!(solve_input_part2(&input), output);
    }
}
