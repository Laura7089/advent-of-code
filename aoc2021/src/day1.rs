#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> Vec<u32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    let mut count = 0;
    for i in 1..input.len() {
        if input[i - 1] < input[i] {
            count += 1;
        }
    }
    count
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[u32]) -> u32 {
    let totals: Vec<u32> = (2..input.len())
        .map(|i| input[i - 2] + input[i - 1] + input[i])
        .collect();
    solve_part1(&totals)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: [u32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&EXAMPLE_INPUT), 7);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&EXAMPLE_INPUT), 5);
    }
}
