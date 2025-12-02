use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day01)]
fn generate(input: &str) -> Vec<i16> {
    input
        .lines()
        .map(|line| {
            let sign = match line.as_bytes()[0] {
                b'R' => 1,
                b'L' => -1,
                other => panic!("unexpected byte in input: {other}"),
            };

            i16::from_str_radix(&line[1..], 10).expect("couldn't parse integer") * sign
        })
        .collect()
}

const DIAL_SIZE: i16 = 100;
const DIAL_START: i16 = 50;

#[aoc(day01, part1)]
fn part1(input: &[i16]) -> usize {
    input
        .iter()
        .scan(DIAL_START, |dial, &twist| {
            *dial = (*dial + twist) % DIAL_SIZE;
            Some(*dial)
        })
        .filter(|&dial| dial == 0)
        .count()
}

#[aoc(day01, part2)]
fn part2(input: &[i16]) -> usize {
    input
        .iter()
        .scan(DIAL_START, |dial, &twist| {
            let unclamped = *dial + twist;
            *dial = unclamped.rem_euclid(DIAL_SIZE);
            Some(unclamped.div_euclid(DIAL_SIZE).abs() as usize)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const SAMPLE_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn part1_sample() {
        let input = generate(SAMPLE_INPUT);
        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn part2_sample() {
        let input = generate(SAMPLE_INPUT);
        assert_eq!(part2(&input), 6);
    }

    #[test_case("L300" => 3)]
    #[test_case("L300\nR100" => 4)]
    #[test_case("L49" => 0)]
    #[test_case("R50" => 1)]
    #[test_case("R49" => 0)]
    fn part2_sanity(input: &str) -> usize {
        part2(&generate(input))
    }
}
