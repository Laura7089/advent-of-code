#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[usize]) -> usize {
    (1..input.len())
        .filter(|i| input[i - 1] < input[*i])
        .count()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[usize]) -> usize {
    (3..input.len())
        .filter(|i| input[i - 3] < input[*i])
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: [usize; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&EXAMPLE_INPUT), 7);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&EXAMPLE_INPUT), 5);
    }

    #[test]
    fn part1_myinput() {
        let input = crate::get_input_for_day(1);
        assert_eq!(solve_part1(&parse_input(&input)), 1696);
    }

    #[test]
    fn part2_myinput() {
        let input = crate::get_input_for_day(1);
        assert_eq!(solve_part2(&parse_input(&input)), 1737);
    }
}
