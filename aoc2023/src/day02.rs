type Game = Vec<Sample>;
type Sample = [u32; 3];

mod parse {
    use nom::branch::alt;
    use nom::bytes::complete::{is_not, tag};
    use nom::character::complete::{newline, u32 as pu32};
    use nom::combinator::map;
    use nom::multi::separated_list1 as sepl;
    use nom::sequence::{preceded, separated_pair as sepp, tuple};

    use super::{Game, Sample};

    type Result<'a, T> = nom::IResult<&'a str, T>;

    pub fn games(input: &str) -> Result<Vec<Game>> {
        sepl(
            newline,
            preceded(tuple((is_not(":"), tag(": "))), sepl(tag("; "), cubes)),
        )(input)
    }

    fn cubes(input: &str) -> Result<Sample> {
        map(sepl(tag(", "), cube), |seq| {
            seq.into_iter()
                .fold([0; 3], |l, r| [l[0] + r[0], l[1] + r[1], l[2] + r[2]])
        })(input)
    }

    fn cube(input: &str) -> Result<Sample> {
        map(
            sepp(pu32, tag(" "), alt((tag("red"), tag("green"), tag("blue")))),
            |(amt, colour)| match colour {
                "red" => [amt, 0, 0],
                "green" => [0, amt, 0],
                "blue" => [0, 0, amt],
                _ => panic!("Bad colour: {colour}"),
            },
        )(input)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_cube() {
            assert_eq!(cube("3 blue"), Ok(("", [0, 0, 3])));
            assert_eq!(cube("28 red"), Ok(("", [28, 0, 0])));
        }

        #[test]
        fn test_cubes() {
            assert_eq!(cubes("3 blue, 1 red"), Ok(("", [1, 0, 3])));
        }
    }
}

#[aoc_generator(day02)]
fn generate(input: &str) -> Vec<Game> {
    parse::games(input).unwrap().1
}

fn game_maxes(game: &Game) -> [u32; 3] {
    game.iter().fold([0, 0, 0], |l, r| {
        [l[0].max(r[0]), l[1].max(r[1]), l[2].max(r[2])]
    })
}

#[aoc(day02, part1)]
fn solve_part1(input: &[Game]) -> usize {
    const TARGET: [u32; 3] = [12, 13, 14];

    input
        .iter()
        .enumerate()
        .filter_map(|(id, game)| {
            if game_maxes(game)
                .into_iter()
                .zip(TARGET.into_iter())
                .all(|(g, t)| g <= t)
            {
                Some(id + 1)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day02, part2)]
fn solve_part2(input: &[Game]) -> u32 {
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
