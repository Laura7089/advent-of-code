use crate::field2d::compressed_field::CompressedField;
use std::cmp::Ordering::*;

type Line = [[usize; 2]; 2];

#[aoc_generator(day5)]
pub fn parse_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(" -> ");
            let mut line = [[0_usize; 2]; 2];
            for side in &mut line {
                let mut raw = split.next().unwrap().split(',');
                for o in 0..2 {
                    side[o] = raw.next().unwrap().parse().unwrap();
                }
            }
            line
        })
        .collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[Line]) -> usize {
    let size_limit = input.iter().flatten().flatten().max().unwrap() + 1;
    let mut field = CompressedField::new(vec![0_usize; size_limit.pow(2)], size_limit);

    for line in input.iter() {
        let (x0, x1) = (line[0][0], line[1][0]);
        let (y0, y1) = (line[0][1], line[1][1]);

        match (x0.cmp(&x1), y0.cmp(&y1)) {
            // Vertical
            (Equal, Less) => (y0..=y1).for_each(|y| field[(x0, y)] += 1),
            (Equal, Greater) => (y1..=y0).for_each(|y| field[(x0, y)] += 1),
            // Horizontal
            (Less, Equal) => (x0..=x1).for_each(|x| field[(x, y0)] += 1),
            (Greater, Equal) => (x1..=x0).for_each(|x| field[(x, y0)] += 1),
            _ => (),
        }
    }

    field.field.iter().filter(|c| c > &&1).count()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[Line]) -> usize {
    let size_limit = input.iter().flatten().flatten().max().unwrap() + 1;
    let mut field = CompressedField::new(vec![0_usize; size_limit * size_limit], size_limit);

    for line in input.iter() {
        let (x0, x1) = (line[0][0], line[1][0]);
        let (y0, y1) = (line[0][1], line[1][1]);

        match (x0.cmp(&x1), y0.cmp(&y1)) {
            // Vertical
            (Equal, Less) => (y0..=y1).for_each(|y| field[(x0, y)] += 1),
            (Equal, Greater) => (y1..=y0).for_each(|y| field[(x0, y)] += 1),
            // Horizontal
            (Less, Equal) => (x0..=x1).for_each(|x| field[(x, y0)] += 1),
            (Greater, Equal) => (x1..=x0).for_each(|x| field[(x, y0)] += 1),
            // Up Diagonals
            (Less, Less) => {
                let line_len = y1 - y0;
                (0..=line_len).for_each(|i| field[(x0 + i, y0 + i)] += 1);
            }
            (Greater, Less) => {
                let line_len = y1 - y0;
                (0..=line_len).for_each(|i| field[(x0 - i, y0 + i)] += 1);
            }
            // Down Diagonals
            (Less, Greater) => {
                let line_len = y0 - y1;
                (0..=line_len).for_each(|i| field[(x0 + i, y0 - i)] += 1);
            }
            (Greater, Greater) => {
                let line_len = y0 - y1;
                (0..=line_len).for_each(|i| field[(x0 - i, y0 - i)] += 1);
            }
            _ => (),
        }
    }

    field.field.iter().filter(|c| c > &&1).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn generator() {
        assert_eq!(
            parse_input(EXAMPLE_INPUT),
            vec![
                [[0, 9], [5, 9],],
                [[8, 0], [0, 8],],
                [[9, 4], [3, 4],],
                [[2, 2], [2, 1],],
                [[7, 0], [7, 4],],
                [[6, 4], [2, 0],],
                [[0, 9], [2, 9],],
                [[3, 4], [1, 4],],
                [[0, 0], [8, 8],],
                [[5, 5], [8, 2]]
            ]
        );
    }

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&parse_input(EXAMPLE_INPUT)), 5);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&parse_input(EXAMPLE_INPUT)), 12);
    }

    #[test]
    fn part1_myinput() {
        let input = crate::get_input_for_day(5);
        assert_eq!(solve_part1(&parse_input(&input)), 5169);
    }

    #[test]
    fn part2_myinput() {
        let input = crate::get_input_for_day(5);
        assert_eq!(solve_part2(&parse_input(&input)), 22083);
    }
}
