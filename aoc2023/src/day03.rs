type Coord = (usize, usize);
type PartMap = Vec<Vec<usize>>;

fn get_symbols_locs(input: &str) -> PartMap {
    input
        .lines()
        .map(|line| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| {
                    if c != '.' && !c.is_digit(10) {
                        Some(x)
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect()
}

fn contains_adj(container: &PartMap, (px, py): Coord) -> bool {
    // Top row
    if py != 0 {
        for x in [px.saturating_sub(1), px, px + 1] {
            if container[py - 1].contains(&x) {
                return true;
            }
        }
    }

    // Middle row
    for x in [px.saturating_sub(1), px + 1] {
        if container[py].contains(&x) {
            return true;
        }
    }

    // Top row
    if py != container.len() - 1 {
        for x in [px.saturating_sub(1), px, px + 1] {
            if container[py + 1].contains(&x) {
                return true;
            }
        }
    }

    false
}

#[aoc(day03, part1)]
fn solve_part1(input: &str) -> usize {
    let symbols_locs = get_symbols_locs(input);

    let mut part_numbers = Vec::with_capacity(symbols_locs.len() * 2);

    for (y, line) in input.lines().enumerate() {
        let mut reached = 0;

        // Process numbers on the line one by one
        loop {
            // Find the next number sequence
            let mut num_seq = line
                .chars()
                .enumerate()
                .skip(reached)
                .skip_while(|(_, c)| !c.is_digit(10))
                .take_while(|(_, c)| c.is_digit(10))
                .peekable();

            let num_start = match num_seq.peek() {
                Some(&(x, _)) => x,
                // We're at the end of the line
                None => break,
            };
            let mut num_end = num_start;

            let mut is_part = false;
            for (x, _) in num_seq {
                if !is_part && contains_adj(&symbols_locs, (x, y)) {
                    is_part = true;
                }
                num_end += 1;
            }

            reached = num_end + 1;
            if is_part {
                // Only parse when we need to
                part_numbers.push(line[num_start..num_end].parse().unwrap());
            }
        }
    }

    part_numbers.iter().sum()
}

#[aoc(day03, part2)]
fn solve_part2(_input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;

    const SAMPLE_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_get_symbols_locs() {
        let locs = get_symbols_locs(SAMPLE_INPUT);

        assert_eq!(
            locs,
            vec![
                vec![],
                vec![3],
                vec![],
                vec![6],
                vec![3],
                vec![5],
                vec![],
                vec![],
                vec![3, 5],
                vec![],
            ]
        );
    }

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part1(SAMPLE_INPUT), 4361);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part1(&crate::get_input(03)), 537832);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part2(SAMPLE_INPUT), 467835);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part2(&crate::get_input(03)), todo!());
        }
    }
}
