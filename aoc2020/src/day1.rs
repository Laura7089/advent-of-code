const TOTAL_TARGET: u32 = 2020;

#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

#[aoc(day1, part1)]
fn solve_input_part1(input: &[u32]) -> u32 {
    let mut target;
    for i in input {
        target = TOTAL_TARGET - i;
        for o in input {
            if o == &target {
                return i * o;
            }
        }
    }
    0
}

#[aoc(day1, part2)]
fn solve_input_part2(input: &[u32]) -> u32 {
    let mut target;
    for i in input {
        for o in input {
            target = TOTAL_TARGET - i - o;
            for p in input {
                if p == &target {
                    return i * o * p;
                }
            }
        }
    }
    0
}
