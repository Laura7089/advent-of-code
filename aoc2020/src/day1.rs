const TARGET: u32 = 2020;

#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

#[aoc(day1, part1)]
fn solve_input(input: &[u32]) -> u32 {
    for i in input {
        for o in input {
            if i + o == TARGET {
                return i + o;
            }
        }
    }
    0
}
