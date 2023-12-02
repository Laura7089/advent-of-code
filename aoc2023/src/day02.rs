use std::ops::Add;

#[derive(Clone, PartialEq, Debug)]
struct Game {
    id: usize,
    samples: Vec<Sample>,
}

#[derive(Debug, Clone, Default, PartialEq)]
struct Sample {
    red: Option<usize>,
    green: Option<usize>,
    blue: Option<usize>,
}

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
        Self {
            red: add_opt_pair(self.red, rhs.red),
            green: add_opt_pair(self.green, rhs.green),
            blue: add_opt_pair(self.blue, rhs.blue),
        }
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
            "red" => sample.red.insert(amt as usize),
            "green" => sample.green.insert(amt as usize),
            "blue" => sample.blue.insert(amt as usize),
            _ => panic!("Bad colour: {colour}"),
        };

        Ok((rem, sample))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_cube() {
            assert_eq!(
                cube("3 blue"),
                Ok((
                    "",
                    Sample {
                        blue: Some(3),
                        ..Default::default()
                    }
                ))
            );
            assert_eq!(
                cube("28 red"),
                Ok((
                    "",
                    Sample {
                        red: Some(28),
                        ..Default::default()
                    }
                ))
            );
        }

        #[test]
        fn test_cubes() {
            assert_eq!(
                cubes("3 blue, 1 red"),
                Ok((
                    "",
                    Sample {
                        blue: Some(3),
                        red: Some(1),
                        green: None
                    }
                ))
            );
        }
    }
}

#[aoc_generator(day02)]
fn generate(input: &str) -> Vec<Game> {
    parse::games(input).unwrap().1
}

#[aoc(day02, part1)]
fn solve_part1(input: &[Game]) -> usize {
    const TARGET: [usize; 3] = [12, 13, 14];

    input
        .iter()
        .filter_map(|game| {
            let ts: Sample = game.samples.iter().cloned().sum();
            let r = ts.red.unwrap_or(0);
            let g = ts.green.unwrap_or(0);
            let b = ts.blue.unwrap_or(0);
            if r <= TARGET[0] && g <= TARGET[1] && b <= TARGET[2] {
                Some(game.id)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day02, part2)]
fn solve_part2(_input: &[Game]) -> usize {
    todo!()
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
            assert_eq!(solve_part1(&generate(&crate::get_input(02))), todo!());
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part2(&generate(SAMPLE_INPUT)), todo!());
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part2(&generate(&crate::get_input(02))), todo!());
        }
    }
}
