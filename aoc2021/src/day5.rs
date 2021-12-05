type Line = [[usize; 2]; 2];

#[aoc_generator(day5)]
pub fn parse_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(" -> ");
            let start_point = {
                let mut raw = split.next().unwrap().split(",");
                [
                    raw.next().unwrap().parse().unwrap(),
                    raw.next().unwrap().parse().unwrap(),
                ]
            };
            let end_point = {
                let mut raw = split.next().unwrap().split(",");
                [
                    raw.next().unwrap().parse().unwrap(),
                    raw.next().unwrap().parse().unwrap(),
                ]
            };
            [start_point, end_point]
        })
        .collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[Line]) -> usize {
    let size_limit = input.iter().flatten().flatten().max().unwrap() + 1;
    let mut field = vec![vec![0_usize; size_limit]; size_limit];

    for line in input.iter() {
        if line[0][0] == line[1][0] {
            // Vertical line
            let x = line[0][0];
            let (y0, y1) = (line[0][1], line[1][1]);
            let range = if y0 < y1 { y0..=y1 } else { y1..=y0 };
            for y in range {
                field[x][y] += 1;
            }
        } else if line[0][1] == line[1][1] {
            // Horizontal line
            let y = line[0][1];
            let (x0, x1) = (line[0][0], line[1][0]);
            let range = if x0 < x1 { x0..=x1 } else { x1..=x0 };
            for x in range {
                field[x][y] += 1;
            }
        }
    }

    field.iter().flatten().filter(|c| c > &&1).count()
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
}
