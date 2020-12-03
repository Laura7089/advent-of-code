use std::ops::Index;
const TREE_CHAR: u8 = '#' as u8;
const PART_TWO_STEPS: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

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

#[aoc_generator(day3)]
pub fn get_slope(input: &str) -> ForestedSlope {
    let grid: Vec<Vec<bool>> = input
        .lines()
        .map(|l| l.as_bytes().iter().map(|s| s == &TREE_CHAR).collect())
        .collect();
    ForestedSlope {
        length: grid.len(),
        width: grid[0].len(),
        grid,
    }
}

#[aoc(day3, part1)]
pub fn solve_input_part1(input: &ForestedSlope) -> usize {
    (0..input.length).filter(|&i| input[(i * 3, i)]).count()
}

#[aoc(day3, part2)]
pub fn solve_input_part2(input: &ForestedSlope) -> usize {
    // let mut total = 1;
    // for (xmul, ymul) in PART_TWO_STEPS.iter() {
    //     total *= (0..input.length / ymul)
    //         .filter(|&i| input[(i * xmul, i * ymul)])
    //         .count();
    // }
    // total

    PART_TWO_STEPS.iter().fold(1, |prev, (xmul, ymul)| {
        prev * (0..input.length / ymul)
            .filter(|&i| input[(i * xmul, i * ymul)])
            .count()
    })
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
