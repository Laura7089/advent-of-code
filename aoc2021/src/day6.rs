#[aoc_generator(day6)]
fn parse_input(input: &str) -> Vec<usize> {
    input.split(",").map(|n| n.parse().unwrap()).collect()
}

#[aoc(day6, part1)]
fn solve_part1(input: &[usize]) -> usize {
    unimplemented!()
}

#[aoc(day6, part2)]
fn solve_part2(input: &[usize]) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "3,4,3,1,2";

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&parse_input(&EXAMPLE_INPUT)), 5934);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&parse_input(&EXAMPLE_INPUT)), unimplemented!());
    }
}
