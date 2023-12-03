struct PartMap(Vec<Vec<usize>>);

impl PartMap {
    fn new(input: &str, pred: impl Fn(char) -> bool) -> Self {
        Self(
            input
                .lines()
                .map(|line| {
                    line.chars()
                        .enumerate()
                        .filter_map(|(x, c)| if pred(c) { Some(x) } else { None })
                        .collect()
                })
                .collect(),
        )
    }

    fn search_rect1(&self, y: usize, x1: usize, len: usize) -> bool {
        let right_lim = x1 + len + 1;

        // Bottom row
        if y != 0 {
            for x in x1.saturating_sub(1)..right_lim {
                if self.0[y - 1].contains(&x) {
                    return true;
                }
            }
        }

        // Middle row
        if self.0[y].contains(&x1.saturating_sub(1)) {
            return true;
        }
        if self.0[y].contains(&(right_lim - 1)) {
            return true;
        }

        // Top row
        if y != self.0.len() - 1 {
            for x in x1.saturating_sub(1)..right_lim {
                if self.0[y + 1].contains(&x) {
                    return true;
                }
            }
        }

        false
    }

    fn search_rect(&self, y: usize, x1: usize, len: usize) -> Vec<(usize, usize)> {
        todo!()
    }
}

#[inline(always)]
fn part1_pred(c: char) -> bool {
    c != '.' && !c.is_digit(10)
}

fn find_number_sequence(input: &str, skip: usize) -> Option<(&str, usize, usize)> {
    let mut num_seq = input
        .chars()
        .enumerate()
        .skip(skip)
        .skip_while(|v| !v.1.is_digit(10));

    let (start, _) = num_seq.next()?;
    let len = num_seq.take_while(|v| v.1.is_digit(10)).count() + 1;
    Some((&input[start..(start + len)], start, len))
}

#[aoc(day03, part1)]
fn solve_part1(input: &str) -> usize {
    let symbols_locs = PartMap::new(input, part1_pred);

    let mut total = 0;
    for (y, line) in input.lines().enumerate() {
        let mut reached = 0;

        // Process numbers on the line one by one
        while let Some((seq, start, len)) = find_number_sequence(line, reached) {
            reached = start + len + 1;
            if symbols_locs.search_rect1(y, start, len) {
                total += seq.parse::<usize>().unwrap();
            }
        }
    }

    total
}

#[inline(always)]
fn part2_pred(c: char) -> bool {
    c == '*'
}

#[aoc(day03, part2)]
fn solve_part2(input: &str) -> usize {
    let pot_gears = PartMap::new(input, part2_pred);
    let mut gear_map: Vec<Vec<(usize, usize)>> = pot_gears
        .0
        .iter()
        .map(|xs| xs.iter().map(|&x| (x, 0)).collect())
        .collect();

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
        let locs = PartMap::new(SAMPLE_INPUT, part1_pred);

        assert_eq!(
            locs.0,
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
        let locs = PartMap::new(SAMPLE_INPUT, part1_pred);

        assert!(locs.search_rect1(0, 0, 3));
        assert!(locs.search_rect1(2, 6, 3));
        assert!(locs.search_rect1(5, 0, 3));
        assert!(locs.search_rect1(6, 2, 3));
        assert!(locs.search_rect1(7, 6, 3));
        assert!(!locs.search_rect1(5, 7, 2));

        // Found earlier
        let test_case = "....\n.12*\n....";
        assert!(PartMap::new(test_case, part1_pred).search_rect1(1, 1, 2));
    }

    #[test]
    fn test_contains_adj_rect_diags() {
        let test_case = b"....\n.12.\n....";
        for x in [0, 3] {
            for y in [0, 2] {
                let mut current_case = test_case.to_vec();
                current_case[(y * 5) + x] = b'*';
                let current_case = String::from_utf8(current_case).unwrap();
                assert!(PartMap::new(&current_case, part1_pred).search_rect1(1, 1, 2));
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
