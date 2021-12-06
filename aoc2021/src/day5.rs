use std::cmp::Ordering;

type Line = [[usize; 2]; 2];

#[aoc_generator(day5)]
pub fn parse_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(" -> ");
            let mut line = [[0_usize; 2]; 2];
            for i in 0..2 {
                let mut raw = split.next().unwrap().split(",");
                for o in 0..2 {
                    line[i][o] = raw.next().unwrap().parse().unwrap();
                }
            }
            line
        })
        .collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[Line]) -> usize {
    let size_limit = input.iter().flatten().flatten().max().unwrap() + 1;
    let mut field = vec![vec![0_usize; size_limit]; size_limit];

    for line in input.iter() {
        let (x0, x1) = (line[0][0], line[1][0]);
        let (y0, y1) = (line[0][1], line[1][1]);

        match (x0.cmp(&x1), y0.cmp(&y1)) {
            // Vertical
            (Ordering::Equal, Ordering::Less) => (y0..=y1).for_each(|y| field[x0][y] += 1),
            (Ordering::Equal, Ordering::Greater) => (y1..=y0).for_each(|y| field[x0][y] += 1),
            // Horizontal
            (Ordering::Less, Ordering::Equal) => (x0..=x1).for_each(|x| field[x][y0] += 1),
            (Ordering::Greater, Ordering::Equal) => (x1..=x0).for_each(|x| field[x][y0] += 1),
            _ => (),
        }
    }

    field.iter().flatten().filter(|c| c > &&1).count()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[Line]) -> usize {
    0
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
    fn test_parse_input() {
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
    fn test_solve_part1_example() {
        assert_eq!(solve_part1(&parse_input(EXAMPLE_INPUT)), 5);
    }

    #[test]
    fn test_solve_part2_example() {
        assert_eq!(solve_part2(&parse_input(EXAMPLE_INPUT)), 12);
    }
}
