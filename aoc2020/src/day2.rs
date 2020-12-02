#[derive(Debug, PartialEq)]
pub struct PasswordWithPolicy {
    pub nums: (usize, usize),
    pub letter: u8,
    pub password: Vec<u8>,
}

#[aoc_generator(day2)]
pub fn parse_input(input: &str) -> Vec<PasswordWithPolicy> {
    input
        .lines()
        .map(|line| {
            let mut line_iter = line.split(' ');
            let mut nums = line_iter.next().expect("Empty line!").split('-');

            PasswordWithPolicy {
                nums: (
                    nums.next()
                        .expect("Bad number formatting")
                        .parse()
                        .expect("Bad number formatting"),
                    nums.next()
                        .expect("Bad number formatting")
                        .parse()
                        .expect("Bad number formatting"),
                ),
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
pub fn solve_input_part1(input: &[PasswordWithPolicy]) -> usize {
    input.iter().filter(valid_password_part1).count()
}

fn valid_password_part1(password: &&PasswordWithPolicy) -> bool {
    let occurences = password
        .password
        .iter()
        .filter(|b| b == &&password.letter)
        .take(password.nums.1 + 1)
        .count();
    occurences >= password.nums.0 && occurences <= password.nums.1
}

#[aoc(day2, part2)]
pub fn solve_input_part2(input: &[PasswordWithPolicy]) -> usize {
    input.iter().filter(valid_password_part2).count()
}

fn valid_password_part2(password: &&PasswordWithPolicy) -> bool {
    (password.password[password.nums.0 - 1] == password.letter)
        ^ (password.password[password.nums.1 - 1] == password.letter)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("1-3 a: abcde", (1, 3), 'a', "abcde")]
    #[test_case("1-3 b: cdefg", (1, 3), 'b', "cdefg")]
    #[test_case("2-9 c: ccccccccc", (2, 9), 'c', "ccccccccc")]
    fn parser(input: &str, nums: (usize, usize), letter: char, password: &str) {
        assert_eq!(
            parse_input(input)[0],
            PasswordWithPolicy {
                nums,
                letter: letter as u8,
                password: Vec::from(password.as_bytes())
            }
        );
    }

    #[test_case((1, 3), 'a', "abcde", true)]
    #[test_case((1, 3), 'b', "cdefg", false)]
    #[test_case((2, 9), 'c', "ccccccccc", true)]
    fn part1(nums: (usize, usize), letter: char, password: &str, expected: bool) {
        assert_eq!(
            valid_password_part1(&&PasswordWithPolicy {
                nums,
                letter: letter as u8,
                password: Vec::from(password.as_bytes())
            }),
            expected
        );
    }
}
