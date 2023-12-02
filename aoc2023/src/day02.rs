use std::ops::Add;

#[derive(Clone, PartialEq, Debug)]
struct Game {
    id: usize,
    samples: Vec<Sample>,
}

#[derive(Debug, Clone, Default, PartialEq)]
struct Sample([Option<usize>; 3]);

fn add_opt_pair<T: Add<Output = T>>(lhs: Option<T>, rhs: Option<T>) -> Option<T> {
    match (lhs, rhs) {
        (Some(l), Some(r)) => Some(l + r),
        (Some(v), None) | (None, Some(v)) => Some(v),
        _ => None,
    }
}

impl Add for Sample {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self([
            add_opt_pair(self.0[0], rhs.0[0]),
            add_opt_pair(self.0[1], rhs.0[1]),
            add_opt_pair(self.0[2], rhs.0[2]),
        ])
    }
}

impl std::iter::Sum<Sample> for Sample {
    fn sum<I: Iterator<Item = Sample>>(iter: I) -> Self {
        iter.fold(Default::default(), |l, r| l + r)
    }
}

mod parse {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{newline, u32 as pu32};
    use nom::combinator::map;
    use nom::multi::separated_list1;
    use nom::sequence::{preceded, separated_pair};

    use super::{Game, Sample};

    type Result<'a, T> = nom::IResult<&'a str, T>;

    pub fn games(input: &str) -> Result<Vec<Game>> {
        separated_list1(newline, game)(input)
    }

    fn game(input: &str) -> Result<Game> {
        map(
            separated_pair(
                preceded(tag("Game "), pu32),
                tag(": "),
                separated_list1(tag("; "), cubes),
            ),
            |(id, samples)| Game {
                id: id as usize,
                samples,
            },
        )(input)
    }

    fn cubes(input: &str) -> Result<Sample> {
        let (rem, seq) = separated_list1(tag(", "), cube)(input)?;
        Ok((rem, seq.into_iter().sum()))
    }

    fn cube(input: &str) -> Result<Sample> {
        let (rem, (amt, colour)) =
            separated_pair(pu32, tag(" "), alt((tag("red"), tag("green"), tag("blue"))))(input)?;

        let mut sample: Sample = Default::default();
        match colour {
            "red" => sample.0[0].insert(amt as usize),
            "green" => sample.0[1].insert(amt as usize),
            "blue" => sample.0[2].insert(amt as usize),
            _ => panic!("Bad colour: {colour}"),
        };

        Ok((rem, sample))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_cube() {
            assert_eq!(cube("3 blue"), Ok(("", Sample([None, None, Some(3),]))));
            assert_eq!(cube("28 red"), Ok(("", Sample([Some(28), None, None]))));
        }

        #[test]
        fn test_cubes() {
            assert_eq!(
                cubes("3 blue, 1 red"),
                Ok(("", Sample([Some(1), None, Some(3),])))
            );
        }
    }
}

#[aoc_generator(day02)]
fn generate(input: &str) -> Vec<Game> {
    parse::games(input).unwrap().1
}

fn game_maxes(game: &Game) -> [usize; 3] {
    game.samples
        .iter()
        // Turn `None`s into `0`s
        .map(|s| s.0.map(|v| v.unwrap_or(0)))
        .fold([0, 0, 0], |l, r| {
            [l[0].max(r[0]), l[1].max(r[1]), l[2].max(r[2])]
        })
}

#[aoc(day02, part1)]
fn solve_part1(input: &[Game]) -> usize {
    const TARGET: [usize; 3] = [12, 13, 14];

    input
        .iter()
        .filter_map(|game| {
            let complies = game_maxes(game)
                .into_iter()
                .zip(TARGET.clone().into_iter())
                .all(|(g, t)| g <= t);
            if complies {
                Some(game.id)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day02, part2)]
fn solve_part2(input: &[Game]) -> usize {
    input
        .iter()
        .map(|game| game_maxes(game).into_iter().fold(1, |l, r| l * r))
        .sum()
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;

    const SAMPLE_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), 8);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part1(&generate(&crate::get_input(02))), 2268);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part2(&generate(SAMPLE_INPUT)), 2286);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part2(&generate(&crate::get_input(02))), 63542);
        }
    }
}
