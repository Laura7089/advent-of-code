#[inline(always)]
fn get_digit(byte: u8) -> Option<usize> {
    (byte >= 48 && byte < 59).then_some(byte as usize - 48)
}

#[aoc(day01, part1)]
fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let first = line
                .bytes()
                .find_map(get_digit)
                .ok_or_else(|| format!("Couldn't find a digit in the line: '{line}'"))
                .unwrap();

            let last = line.bytes().rev().find_map(get_digit).unwrap();

            (first * 10) + last
        })
        .sum()
}

const DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn get_digit_word(input: &str) -> Option<usize> {
    let first_byte = input.bytes().next()?;
    get_digit(first_byte).or_else(|| DIGITS.iter().position(|w| input.starts_with(w)))
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
                solve_part1(
                    "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
                ),
                142
            );
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part1(&crate::get_input(01)), 54630);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(
                solve_part2(
                    "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
                ),
                281
            );
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part2(&crate::get_input(01)), 54770);
        }
    }
}
