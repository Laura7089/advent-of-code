#[derive(Debug, PartialEq)]
pub struct PasswordWithPolicy {
    pub letter: Vec<u8>,
    pub password: Vec<u8>,
    pub num_first: u32,
    pub num_second: u32,
}

#[aoc_generator(day2)]
pub fn parse_input(input: &str) -> Vec<PasswordWithPolicy> {
    input
        .lines()
        .map(|line| {
            let mut line_iter = line.split(' ');
            let mut nums = line_iter.next().expect("Empty line!").split('-');

            PasswordWithPolicy {
                num_first: nums
                    .next()
                    .expect("Bad number formatting")
                    .parse()
                    .expect("Bad number formatting"),
                num_second: nums
                    .next()
                    .expect("Bad number formatting")
                    .parse()
                    .expect("Bad number formatting"),
                letter: Vec::from(
                    line_iter
                        .next()
                        .expect("Missing letter and password")
                        .split(':')
                        .next()
                        .expect("Bad letter formatting")
                        .as_bytes(),
                ),
                password: Vec::from(line_iter.next().expect("Missing password").as_bytes()),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_input_part1(input: &[PasswordWithPolicy]) -> u32 {
    input.iter().filter(valid_password_part1).count() as u32
}

fn valid_password_part1(password: &&PasswordWithPolicy) -> bool {
    // TODO: Make compatible with multi-byte "letters"
    let mut letters_in_password = 0u32;
    for letter in password.password.iter() {
        letters_in_password += (letter == &password.letter[0]) as u32;
        if letters_in_password > password.num_second {
            return false;
        }
    }
    letters_in_password >= password.num_first
}

#[aoc(day2, part2)]
pub fn solve_input_part2(input: &[PasswordWithPolicy]) -> u32 {
    input.iter().filter(valid_password_part2).count() as u32
}

fn valid_password_part2(password: &&PasswordWithPolicy) -> bool {
    // TODO: Make compatible with multi-byte "letters"
    let search_byte = password.letter[0];
    (password.password[password.num_first as usize - 1] == search_byte)
        ^ (password.password[password.num_second as usize - 1] == search_byte)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("1-3 a: abcde".to_string(), PasswordWithPolicy{ num_first: 1, num_second: 3, letter: "a".to_string(), password: "abcde".to_string() })]
    #[test_case("1-3 b: cdefg".to_string(), PasswordWithPolicy{ num_first: 1, num_second: 3, letter: "b".to_string(), password: "cdefg".to_string() })]
    #[test_case("2-9 c: ccccccccc".to_string(), PasswordWithPolicy{ num_first: 2, num_second: 9, letter: "c".to_string(), password: "ccccccccc".to_string() })]
    fn parser(input: String, expected: PasswordWithPolicy) {
        assert_eq!(parse_input(&input)[0], expected);
    }

    #[test_case(PasswordWithPolicy{ num_first: 1, num_second: 3, letter: "a".to_string(), password: "abcde".to_string() }, true)]
    #[test_case(PasswordWithPolicy{ num_first: 1, num_second: 3, letter: "b".to_string(), password: "cdefg".to_string() }, false)]
    #[test_case(PasswordWithPolicy{ num_first: 2, num_second: 9, letter: "c".to_string(), password: "ccccccccc".to_string() }, true)]
    fn part1(input: PasswordWithPolicy, expected: bool) {
        assert_eq!(valid_password_part1(&&input), expected);
    }
}
