#[derive(Debug, PartialEq)]
pub struct PasswordWithPolicy {
    pub letter: u8,
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
                letter: line_iter
                    .next()
                    .expect("Missing letter and password")
                    .split(':')
                    .next()
                    .expect("Bad letter formatting")
                    .as_bytes()[0],

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
    let occurences = password
        .password
        .iter()
        .filter(|b| b == &&password.letter)
        .take(password.num_second as usize + 1)
        .count() as u32;
    occurences >= password.num_first && occurences <= password.num_second
}

#[aoc(day2, part2)]
pub fn solve_input_part2(input: &[PasswordWithPolicy]) -> u32 {
    input.iter().filter(valid_password_part2).count() as u32
}

fn valid_password_part2(password: &&PasswordWithPolicy) -> bool {
    (password.password[password.num_first as usize - 1] == password.letter)
        ^ (password.password[password.num_second as usize - 1] == password.letter)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("1-3 a: abcde", (1, 3), 'a', "abcde" )]
    #[test_case("1-3 b: cdefg", (1, 3), 'b', "cdefg" )]
    #[test_case("2-9 c: ccccccccc", (2, 9), 'c', "ccccccccc" )]
    fn parser(input: &str, (num_first, num_second): (u32, u32), letter: char, password: &str) {
        assert_eq!(
            parse_input(input)[0],
            PasswordWithPolicy {
                num_first,
                num_second,
                letter: letter as u8,
                password: Vec::from(password.as_bytes())
            }
        );
    }

    #[test_case((1, 3), 'a', "abcde", true)]
    #[test_case(( 1, 3), 'b', "cdefg", false)]
    #[test_case(( 2, 9), 'c', "ccccccccc", true)]
    fn part1((num_first, num_second): (u32, u32), letter: char, password: &str, expected: bool) {
        assert_eq!(
            valid_password_part1(&&PasswordWithPolicy {
                num_first,
                num_second,
                letter: letter as u8,
                password: Vec::from(password.as_bytes())
            }),
            expected
        );
    }
}
