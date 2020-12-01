const TOTAL_TARGET: u32 = 2020;

#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> Vec<u32> {
    crate::input::list_of_numbers(input)
}

#[aoc(day1, part1)]
fn solve_input_part1(input: &[u32]) -> u32 {
    let mut target;
    for (i, elem1) in input.iter().enumerate() {
        if elem1 > &TOTAL_TARGET {
            continue;
        }
        target = TOTAL_TARGET - elem1;
        for elem2 in input[i..].iter() {
            if elem2 == &target {
                return elem1 * elem2;
            }
        }
    }
    0
}

#[aoc(day1, part2)]
fn solve_input_part2(input: &[u32]) -> u32 {
    let mut target;
    for (i, elem1) in input.iter().enumerate() {
        for (o, elem2) in input[i..].iter().enumerate() {
            let sub = elem1 + elem2;
            if sub > TOTAL_TARGET {
                continue;
            }
            target = TOTAL_TARGET - sub;
            for elem3 in input[o..].iter() {
                if elem3 == &target {
                    return elem1 * elem2 * elem3;
                }
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(&[100,200,1920], 192_000)]
    #[test_case(&[1820,200,1920], 364_000)]
    #[test_case(&[1010,1010,1920], 1_020_100)]
    #[test_case(&[29,200,1991], 57_739)]
    #[test_case(&[28,200,1991], 0 ; "No valid combinations")]
    #[test_case(&[], 0 ; "Empty input")]
    fn part1(input: &[u32], expected: u32) {
        assert_eq!(solve_input_part1(input), expected);
    }

    #[test_case(&[200,1800,20,10,1241,23], 7_200_000)]
    #[test_case(&[200,579,20,10,1241,23], 143_707_800)]
    #[test_case(&[28,200,1991,1231], 0 ; "No valid combinations")]
    #[test_case(&[], 0 ; "Empty input")]
    fn part2(input: &[u32], expected: u32) {
        assert_eq!(solve_input_part2(input), expected);
    }
}
