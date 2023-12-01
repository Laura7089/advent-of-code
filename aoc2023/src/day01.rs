#[aoc_generator(day01)]
fn generate(input: &str) -> String {
    input.to_owned()
}

#[inline(always)]
fn is_digit(&byte: &u8) -> bool {
    byte >= 48 && byte < 59
}

#[aoc(day01, part1)]
fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let first = line
                .bytes()
                .find(is_digit)
                .ok_or_else(|| format!("Couldn't find a digit in the line: '{line}'"))
                .unwrap() as usize
                - 48;

            let last = line.bytes().rev().find(is_digit).unwrap() as usize - 48;

            (first * 10) + last
        })
        .sum()
}

const DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn get_digit_word(input: &str) -> Option<usize> {
    match input.as_bytes().get(0) {
        None => None,
        Some(b) if is_digit(b) => Some(*b as usize - 48),
        _ => DIGITS.iter().position(|w| input.starts_with(w)),
    }
}

#[aoc(day01, part2)]
fn solve_part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let first = (0..line.len())
                .find_map(|i| get_digit_word(&line[i..]))
                .ok_or_else(|| format!("Couldn't find a digit in the line: '{line}'"))
                .unwrap();

            let last = (0..line.len())
                .rev()
                .find_map(|i| get_digit_word(&line[i..]))
                .unwrap();

            (first * 10) + last
        })
        .sum()
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(
                solve_part1(&generate(
                    "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
                )),
                142
            );
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part1(&generate(&crate::get_input(01))), 54630);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(
                solve_part2(&generate(
                    "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
                )),
                281
            );
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part2(&generate(&crate::get_input(01))), 54770);
        }
    }
}
