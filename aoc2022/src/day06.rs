use itertools::Itertools;

fn find_unique<const SIZE: usize>(input: &str) -> usize {
    for i in 0..(input.len() - SIZE) {
        if input.as_bytes()[i..][..SIZE].iter().unique().count() == SIZE {
            return i + SIZE;
        }
    }
    panic!("No valid sequence found!");
}

#[aoc(day6, part1)]
fn solve_part1(input: &str) -> usize {
    find_unique::<4>(input)
}

#[aoc(day6, part2)]
fn solve_part2(input: &str) -> usize {
    find_unique::<14>(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_input;
    const SAMPLE_INPUT: &[(&str, usize, usize)] = &[
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6, 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26),
    ];

    #[test]
    fn part1_example() {
        for (input, expected, _) in SAMPLE_INPUT {
            println!("Trying '{input}'");
            assert_eq!(solve_part1(input), *expected);
        }
    }

    #[test]
    fn part1_mine() {
        assert_eq!(solve_part1(&get_input(6)), 1760);
    }

    #[test]
    fn part2_example() {
        for (input, _, expected) in SAMPLE_INPUT {
            println!("Trying '{input}'");
            assert_eq!(solve_part2(input), *expected);
        }
    }
}
