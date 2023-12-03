type Coord = (usize, usize);

fn get_symbols_locs(input: &str) -> Vec<Coord> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c != '.' && !c.is_digit(10) {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn contains_adj(container: &[Coord], (px, py): Coord) -> bool {
    const MODS: &[isize] = &[-1, 0, 1];

    for &ym in MODS.iter() {
        for &xm in MODS.iter() {
            let changed_point = (px.saturating_add_signed(xm), py.saturating_add_signed(ym));
            if container.contains(&changed_point) {
                return true;
            }
        }
    }

    false
}

#[aoc(day03, part1)]
fn solve_part1(input: &str) -> usize {
    let symbols_locs = get_symbols_locs(input);

    let mut part_numbers: Vec<usize> = Vec::with_capacity(symbols_locs.len() * 2);

    for (y, line) in input.lines().enumerate() {
        let mut reached = 0;
        loop {
            let mut num_seq = line
                .chars()
                .enumerate()
                .skip(reached)
                .skip_while(|(_, c)| !c.is_digit(10))
                .take_while(|(_, c)| c.is_digit(10))
                .peekable();

            let mut is_part = false;
            let num_start = match num_seq.peek() {
                Some(&(x, _)) => x,
                None => break,
            };
            let mut num_end = num_start;

            for (x, _) in num_seq {
                if !is_part && contains_adj(&symbols_locs, (x, y)) {
                    is_part = true;
                }
                num_end = x + 1;
            }

            let num_raw = &line[num_start..num_end];
            reached = num_end + 1;
            if is_part {
                part_numbers.push(num_raw.parse().unwrap());
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

        assert_eq!(locs, vec![(3, 1), (6, 3), (3, 4), (5, 5), (3, 8), (5, 8)]);
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
            assert_eq!(solve_part2(SAMPLE_INPUT), todo!());
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part2(&crate::get_input(03)), todo!());
        }
    }
}
