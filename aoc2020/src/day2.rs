#[derive(Debug, PartialEq)]
pub struct PasswordWithPolicy {
    pub letter: String,
    pub password: String,
    pub letter_min: u32,
    pub letter_max: u32,
}

#[aoc_generator(day2)]
pub fn parse_input(input: &str) -> Vec<PasswordWithPolicy> {
    input
        .lines()
        .map(|line| {
            let mut line_iter = line.split(' ');
            let mut nums = line_iter.next().expect("Empty line!").split('-');

            PasswordWithPolicy {
                letter_min: nums
                    .next()
                    .expect("Bad number formatting")
                    .parse()
                    .expect("Bad number formatting"),
                letter_max: nums
                    .next()
                    .expect("Bad number formatting")
                    .parse()
                    .expect("Bad number formatting"),
                letter: line_iter
                    .next()
                    .expect("Missing letter and password")
                    .split(':')
                    .next()
                    .expect("Bad letter formatting")
                    .to_string(),
                password: line_iter.next().expect("Missing password").to_string(),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_input_part1(input: &[PasswordWithPolicy]) -> u32 {
    let mut total_valid = 0;
    for password in input.iter() {
        // TODO: Make compatible with multi-byte "letters"
        let search_byte = password.letter.as_bytes()[0];
        let password_bytes = password.password.as_bytes();
        let mut letters_in_password = 0u32;
        for letter in password_bytes.iter() {
            letters_in_password += (letter == &search_byte) as u32;
            if letters_in_password > password.letter_max {
                continue;
            }
        }
        total_valid += (letters_in_password >= password.letter_min) as u32;
    }
    total_valid
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("1-3 a: abcde".to_string(), PasswordWithPolicy{ letter_min: 1, letter_max: 3, letter: "a".to_string(), password: "abcde".to_string() })]
    #[test_case("1-3 b: cdefg".to_string(), PasswordWithPolicy{ letter_min: 1, letter_max: 3, letter: "b".to_string(), password: "cdefg".to_string() })]
    #[test_case("2-9 c: ccccccccc".to_string(), PasswordWithPolicy{ letter_min: 2, letter_max: 9, letter: "c".to_string(), password: "ccccccccc".to_string() })]
    fn parser(input: String, expected: PasswordWithPolicy) {
        assert_eq!(parse_input(&input)[0], expected);
    }

    #[test_case(&[PasswordWithPolicy{ letter_min: 1, letter_max: 3, letter: "a".to_string(), password: "abcde".to_string() }], 1)]
    #[test_case(&[PasswordWithPolicy{ letter_min: 1, letter_max: 3, letter: "b".to_string(), password: "cdefg".to_string() }], 0)]
    #[test_case(&[PasswordWithPolicy{ letter_min: 2, letter_max: 9, letter: "c".to_string(), password: "ccccccccc".to_string() }], 1)]
    fn part1(input: &[PasswordWithPolicy], expected: u32) {
        assert_eq!(solve_input_part1(input), expected);
    }
}
