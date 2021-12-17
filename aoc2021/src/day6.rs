const CYCLE_TIME: usize = 7;

type Shoal = [usize; CYCLE_TIME + 2];

#[inline(always)]
fn fish_sim(shoal: &mut Shoal, days: usize) {
    for day in 1..=days {
        // Fish that reproduced today
        let repro_index = (day - 1) % CYCLE_TIME;
        let reproductions = shoal[repro_index];

        // "Decrement" all fish >= CYCLE_TIME (babies)
        // Merge them into the main-cycle fish
        shoal[repro_index] += shoal[CYCLE_TIME];
        shoal[CYCLE_TIME] = shoal[CYCLE_TIME + 1];

        // Add baby fish :)
        shoal[CYCLE_TIME + 1] = reproductions;
    }
}

#[aoc_generator(day6)]
pub fn parse_input(input: &str) -> Shoal {
    let mut shoal = [0; CYCLE_TIME + 2];
    for fish in input.split(',') {
        shoal[fish.parse::<usize>().unwrap()] += 1;
    }
    shoal
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &Shoal) -> usize {
    let mut shoal = *input;

    fish_sim(&mut shoal, 80);
    shoal.into_iter().sum()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &Shoal) -> usize {
    let mut shoal = *input;

    fish_sim(&mut shoal, 256);
    shoal.into_iter().sum()
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
        assert_eq!(solve_part2(&parse_input(&EXAMPLE_INPUT)), 26984457539);
    }

    #[test]
    fn part1_myinput() {
        let input = crate::get_input_for_day(6);
        assert_eq!(solve_part1(&parse_input(&input)), 389726);
    }

    #[test]
    fn part2_myinput() {
        let input = crate::get_input_for_day(6);
        assert_eq!(solve_part2(&parse_input(&input)), 1743335992042);
    }
}
