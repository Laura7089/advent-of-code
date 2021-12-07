#[aoc_generator(day7)]
pub fn parse_input(input: &str) -> Vec<usize> {
    input.split(",").map(|n| n.parse().unwrap()).collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &[usize]) -> usize {
    unimplemented!()
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &[usize]) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&parse_input(EXAMPLE_INPUT)), 37);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&parse_input(EXAMPLE_INPUT)), unimplemented!());
    }
}
