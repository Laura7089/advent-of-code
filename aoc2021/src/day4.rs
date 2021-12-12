type Generated = (Vec<usize>, Vec<BingoBoard<5>>);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct BingoBoard<const SIZE: usize> {
    pub numbers: [[usize; SIZE]; SIZE],
    pub marks: [[bool; SIZE]; SIZE],
}

impl<const SIZE: usize> BingoBoard<SIZE> {
    pub fn has_won(&self) -> bool {
        // Rows
        for y in 0..SIZE {
            if (0..SIZE).all(|x| self.marks[y][x]) {
                return true;
            }
        }

        // Columns
        for x in 0..SIZE {
            if (0..SIZE).all(|y| self.marks[y][x]) {
                return true;
            }
        }

        false
    }

    pub fn search_mark(&mut self, winner: usize) -> bool {
        for x in 0..SIZE {
            for y in 0..SIZE {
                if self.numbers[y][x] == winner {
                    self.marks[y][x] = true;
                    return true;
                }
            }
        }
        false
    }

    pub fn total_unmarked(&self) -> usize {
        self.numbers
            .iter()
            .flatten()
            .zip(self.marks.iter().flatten())
            .filter_map(|(num, mark)| match mark {
                false => Some(num),
                true => None,
            })
            .sum()
    }
}

#[aoc_generator(day4)]
pub fn parse_input(input: &str) -> Generated {
    let called_numbers = input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(str::parse)
        .collect::<Result<Vec<usize>, _>>()
        .unwrap();

    let boards = input
        .split("\n\n")
        .skip(1) // Skip the called numbers list
        .map(|board_raw| {
            let mut numbers = [[0_usize; 5]; 5];
            for (y, row) in board_raw.split("\n").enumerate() {
                for (x, n) in row.split_whitespace().enumerate() {
                    numbers[y][x] = n.parse().unwrap();
                }
            }

            BingoBoard {
                numbers,
                marks: [[false; 5]; 5],
            }
        })
        .collect();

    (called_numbers, boards)
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &Generated) -> usize {
    let mut boards = input.1.clone();

    for call in input.0.iter() {
        for i in 0..boards.len() {
            // Note: mutates board
            let marked = boards[i].search_mark(*call);
            if marked && boards[i].has_won() {
                return boards[i].total_unmarked() * call;
            }
        }
    }
    panic!("No winners found!")
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &Generated) -> usize {
    let mut boards = input.1.clone();

    for call in input.0.iter() {
        // Find winning boards
        let mut to_pop = Vec::new();
        for i in 0..boards.len() {
            let marked = boards[i].search_mark(*call);
            if marked && boards[i].has_won() {
                to_pop.push(i);
            }
        }

        // Remove winning boards
        to_pop.sort();
        to_pop.into_iter().rev().for_each(|i| {
            boards.remove(i);
        });

        // Check
        if boards.len() == 1 {
            return boards[0].total_unmarked() * call;
        }
    }

    panic!("No one won before we ran out of calls!")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_RAW: &'static str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(INPUT_RAW),
            (
                vec![
                    7_usize, 4_usize, 9_usize, 5_usize, 11_usize, 17_usize, 23_usize, 2_usize,
                    0_usize, 14_usize, 21_usize, 24_usize, 10_usize, 16_usize, 13_usize, 6_usize,
                    15_usize, 25_usize, 12_usize, 22_usize, 18_usize, 20_usize, 8_usize, 19_usize,
                    3_usize, 26_usize, 1_usize
                ],
                vec![
                    BingoBoard {
                        numbers: [
                            [22, 13, 17, 11, 0],
                            [8, 2, 23, 4, 24],
                            [21, 9, 14, 16, 7],
                            [6, 10, 3, 18, 5],
                            [1, 12, 20, 15, 19],
                        ],
                        marks: [[false; 5]; 5]
                    },
                    BingoBoard {
                        numbers: [
                            [3, 15, 0, 2, 22],
                            [9, 18, 13, 17, 5],
                            [19, 8, 7, 25, 23],
                            [20, 11, 10, 24, 4],
                            [14, 21, 16, 12, 6],
                        ],
                        marks: [[false; 5]; 5]
                    },
                    BingoBoard {
                        numbers: [
                            [14, 21, 17, 24, 4],
                            [10, 16, 15, 9, 19],
                            [18, 8, 23, 26, 20],
                            [22, 11, 13, 6, 5],
                            [2, 0, 12, 3, 7],
                        ],
                        marks: [[false; 5]; 5]
                    }
                ]
            )
        );
    }

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&parse_input(INPUT_RAW)), 4512);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&parse_input(INPUT_RAW)), 1924);
    }

    #[test]
    fn board_winner1() {
        assert!(BingoBoard {
            numbers: [[0; 3]; 3],
            marks: [
                [true, true, true],
                [false, false, false],
                [false, false, false]
            ]
        }
        .has_won());
    }

    #[test]
    fn board_winner2() {
        assert!(BingoBoard {
            numbers: [[0; 3]; 3],
            marks: [
                [true, false, false],
                [true, false, false],
                [true, false, false]
            ]
        }
        .has_won());
    }

    #[test]
    fn part1_myinput() {
        let input = crate::get_input_for_day(4);
        assert_eq!(solve_part1(&parse_input(&input)), 25410);
    }

    #[test]
    fn part2_myinput() {
        let input = crate::get_input_for_day(4);
        assert_eq!(solve_part2(&parse_input(&input)), unimplemented!());
    }
}
