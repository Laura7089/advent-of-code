const SEARCH_AREA: [i32; 3] = [-1, 0, 1];

#[aoc_generator(day7)]
pub fn parse_input(input: &str) -> Vec<usize> {
    input.split(",").map(|n| n.parse().unwrap()).collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &[usize]) -> usize {
    let mut input: Vec<usize> = input.to_vec();
    // Best position is in the median when sorted
    input.sort();
    let best_pos = input[input.len() / 2];

    input
        .into_iter()
        .map(|pos| {
            if best_pos > pos {
                best_pos - pos
            } else {
                pos - best_pos
            }
        })
        .sum()
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &[usize]) -> usize {
    // Best position is within SEARCH_AREA + mean
    let mean = (input.iter().sum::<usize>() as f64 / input.len() as f64).ceil() as usize;

    SEARCH_AREA
        .iter()
        .map(|offset| {
            let spot = (mean as i32 + offset) as usize;
            input
                .iter()
                .map(|pos| {
                    let dist = if spot > *pos { spot - pos } else { pos - spot };
                    (dist * (dist + 1)) / 2
                })
                .sum()
        })
        .min()
        .unwrap()
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
        assert_eq!(solve_part2(&parse_input(EXAMPLE_INPUT)), 168);
    }

    #[test]
    fn part1_myinput() {
        let input = crate::get_input_for_day(7);
        assert_eq!(solve_part1(&parse_input(&input)), 336040);
    }

    #[test]
    fn part2_myinput() {
        let input = crate::get_input_for_day(7);
        assert_eq!(solve_part2(&parse_input(&input)), 94813675);
    }
}
