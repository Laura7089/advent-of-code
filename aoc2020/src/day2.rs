#[derive(Debug, PartialEq)]
pub struct PasswordWithPolicy {
    pub nums: (usize, usize),
    pub letter: char,
    pub password: String,
}

#[aoc_generator(day2)]
pub fn parse_input(input: &str) -> Vec<PasswordWithPolicy> {
    input
        .lines()
        .map(|line| {
            let mut min = 0;
            let mut worker: String = String::with_capacity(3);

            for (i, c) in line.chars().enumerate() {
                match c {
                    '-' => {
                        min = worker.parse().unwrap();
                        worker.clear();
                    }
                    ' ' => {
                        return PasswordWithPolicy {
                            nums: (min, worker.parse().unwrap()),
                            letter: line.chars().nth(i + 1).unwrap(),
                            password: line[i + 4..].to_string(),
                        };
                    }
                    _ => worker.push(c),
                }
            }

            panic!("Bad line formatting");
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
        .chars()
        .filter(|b| b == &password.letter)
        .take(password.nums.1 + 1)
        .count();
    occurences >= password.nums.0 && occurences <= password.nums.1
}

#[aoc(day2, part2)]
pub fn solve_input_part2(input: &[PasswordWithPolicy]) -> usize {
    input.iter().filter(valid_password_part2).count()
}

fn valid_password_part2(password: &&PasswordWithPolicy) -> bool {
    (password.password.as_bytes()[password.nums.0 - 1] as char == password.letter)
        ^ (password.password.as_bytes()[password.nums.1 - 1] as char == password.letter)
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
                letter: letter,
                password: password.to_string()
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
                letter: letter,
                password: password.to_string(),
            }),
            expected
        );
    }
}
