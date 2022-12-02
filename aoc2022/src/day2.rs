#[derive(Copy, Clone, PartialEq, Debug)]
enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}
use Play::*;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Outcome {
    Win = 6,
    Draw = 3,
    Loss = 0,
}
use Outcome::*;

impl Outcome {
    fn new(input: char) -> Self {
        match input {
            'X' => Loss,
            'Y' => Draw,
            'Z' => Win,
            _ => panic!("{input} isn't a valid play"),
        }
    }
}

impl Play {
    fn new(input: char) -> Self {
        match input {
            'A' | 'X' => Rock,
            'B' | 'Y' => Paper,
            'C' | 'Z' => Scissors,
            _ => panic!("{input} isn't a valid play"),
        }
    }

    fn resolve(&self, other: &Self) -> Outcome {
        match (self, other) {
            (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => Win,
            (x, y) if x == y => Draw,
            _ => Loss,
        }
    }

    fn find_desired(&self, desired: Outcome) -> u32 {
        match desired {
            Win => (*self as u32 % 3) + 1,
            Loss => ((*self as u32 + 1) % 3) + 1,
            Draw => *self as u32,
        }
    }
}

#[aoc(day2, part1)]
fn solve_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let mut l = l.chars();
            let left = Play::new(l.next().unwrap());
            l.next();
            (left, Play::new(l.next().unwrap()))
        })
        // Note the swapped order
        .map(|(theirs, ours)| ours.resolve(&theirs) as u32 + ours as u32)
        .sum()
}

#[aoc(day2, part2)]
fn solve_part2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let mut l = l.chars();
            let left = Play::new(l.next().unwrap());
            l.next();
            (left, Outcome::new(l.next().unwrap()))
        })
        .map(|(theirs, want)| want as u32 + theirs.find_desired(want))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &'static str = "A Y
B X
C Z";

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(SAMPLE_INPUT), 15);
    }

    #[test]
    fn part1_mine() {
        assert_eq!(solve_part1(&crate::get_input(2)), 10816);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(SAMPLE_INPUT), 12);
    }

    #[test]
    fn part2_mine() {
        assert_eq!(solve_part2(&crate::get_input(2)), 11657);
    }
}
