const ID_CHAR_OFFSET: usize = 'a' as usize;
const ID_CHAR_LENGTH: usize = 26;

#[aoc_generator(day6)]
pub fn parse_input(input: &str) -> Vec<Vec<Vec<u8>>> {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|person| person.bytes().collect())
                .collect()
        })
        .collect()
}

#[aoc(day6, part1)]
pub fn solve_input_part1(input: &[Vec<Vec<u8>>]) -> usize {
    input
        .iter()
        .map(|group| {
            let mut map = [false; ID_CHAR_LENGTH];
            for c in group.iter().flatten() {
                map[*c as usize - ID_CHAR_OFFSET] = true;
            }
            map.iter().filter(|q| **q).count()
        })
        .sum()
}

#[aoc(day6, part2)]
pub fn solve_input_part2(input: &[Vec<Vec<u8>>]) -> usize {
    input
        .iter()
        .map(|group| {
            let mut group_map = [true; ID_CHAR_LENGTH];
            for person in group.iter() {
                let mut person_map = [false; ID_CHAR_LENGTH];
                // Get all the questions the person answered "yes" to
                for c in person.iter() {
                    person_map[*c as usize - ID_CHAR_OFFSET] = true;
                }
                // Insert them into the group pool of "yes" questions
                for i in 0..26 {
                    group_map[i] &= person_map[i];
                }
            }
            group_map.iter().filter(|q| **q).count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("abcx\nabcy\nabcz", vec![vec!["abcx", "abcy", "abcz"]]; "site example")]
    fn test_parse_input(input: &str, expected: Vec<Vec<&str>>) {
        let expected: Vec<Vec<Vec<u8>>> = expected
            .into_iter()
            .map(|g| g.iter().map(|p| p.bytes().collect()).collect())
            .collect();
        assert_eq!(parse_input(input), expected);
    }

    #[test_case("abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb", 11)]
    fn test_part1(input: &str, expected: usize) {
        assert_eq!(solve_input_part1(&parse_input(input)), expected);
    }

    #[test_case("abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb", 6)]
    fn test_part2(input: &str, expected: usize) {
        assert_eq!(solve_input_part2(&parse_input(input)), expected);
    }


    #[test]
    fn my_input_part1() {
        assert_eq!(
            solve_input_part1(&parse_input(
                &std::fs::read_to_string("./input/2020/day6.txt").unwrap()
            )),
            6782
        );
    }

    #[test]
    fn my_input_part2() {
        assert_eq!(
            solve_input_part2(&parse_input(
                &std::fs::read_to_string("./input/2020/day6.txt").unwrap()
            )),
            3596
        );
    }
}
