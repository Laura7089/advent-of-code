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

fn contains_adj_rect(container: &PartMap, y: usize, x1: usize, len: usize) -> bool {
    let right_lim = x1 + len + 1;

    // Bottom row
    if y != 0 {
        for x in x1.saturating_sub(1)..right_lim {
            if container[y - 1].contains(&x) {
                return true;
            }
        }
    }

    // Middle row
    if container[y].contains(&x1.saturating_sub(1)) {
        return true;
    }
    if container[y].contains(&(right_lim - 1)) {
        return true;
    }

    // Top row
    if y != container.len() - 1 {
        for x in x1.saturating_sub(1)..right_lim {
            if container[y + 1].contains(&x) {
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

            let start = match num_seq.peek() {
                Some(&(x, _)) => x,
                // We're at the end of the line
                None => break,
            };
            let len = num_seq.count();

            let is_part = contains_adj_rect(&symbols_locs, y, start, len);

            let end = start + len;
            reached = end + 1;
            if is_part {
                // Only parse when we need to
                part_numbers.push(line[start..end].parse().unwrap());
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

    #[test]
    fn test_contains_adj_rect() {
        let locs = get_symbols_locs(SAMPLE_INPUT);

        assert!(contains_adj_rect(&locs, 0, 0, 3));
        assert!(contains_adj_rect(&locs, 2, 6, 3));
        assert!(contains_adj_rect(&locs, 5, 0, 3));
        assert!(contains_adj_rect(&locs, 6, 2, 3));
        assert!(contains_adj_rect(&locs, 7, 6, 3));
        assert!(!contains_adj_rect(&locs, 5, 7, 2));

        // Found earlier
        let test_case = "....\n.12*\n....";
        assert!(contains_adj_rect(&get_symbols_locs(test_case), 1, 1, 2));
    }

    #[test]
    fn test_contains_adj_rect_diags() {
        let test_case = b"....\n.12.\n....";
        for x in [0, 3] {
            for y in [0, 2] {
                let mut current_case = test_case.to_vec();
                current_case[(y * 5) + x] = b'*';
                let current_case = String::from_utf8(current_case).unwrap();
                println!("Testing:\n{current_case}");
                assert!(contains_adj_rect(&get_symbols_locs(&current_case), 1, 1, 2));
            }
        }
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
