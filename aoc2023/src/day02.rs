use std::cmp::max;

type Game = Vec<Sample>;
type Sample = [u32; 3];

mod parse {
    use nom::branch::alt;
    use nom::bytes::complete::{is_not, tag, take};
    use nom::character::complete::{newline, u32 as pu32};
    use nom::combinator::map;
    use nom::multi::separated_list1 as sepl;
    use nom::sequence::{preceded, separated_pair as sepp, tuple};

    use super::{Game, Sample};

    type Result<'a, T> = nom::IResult<&'a str, T>;

    pub fn games(input: &str) -> Result<Vec<Game>> {
        sepl(
            newline,
            preceded(
                // We could just skip until ": " here, but instead we can
                // leverage that we know we can skip certain numbers of characters
                // without checking them
                tuple((take(6u8), is_not(" "), take(1u8))),
                sepl(tag("; "), cubes),
            ),
        )(input)
    }

    fn cubes(input: &str) -> Result<Sample> {
        map(sepl(tag(", "), cube), |seq| {
            seq.into_iter()
                // Horrendous, wish there was a better way to do elementwise array combination
                .fold([0; 3], |l, r| [l[0] + r[0], l[1] + r[1], l[2] + r[2]])
        })(input)
    }

    fn cube(input: &str) -> Result<Sample> {
        map(
            sepp(
                pu32,
                take(1u8),
                // using the `take` method here gave a performance regression :(
                alt((tag("red"), tag("green"), tag("blue"))),
            ),
            |(amt, colour): (_, &str)| match colour.as_bytes()[0] {
                b'r' => [amt, 0, 0],
                b'g' => [0, amt, 0],
                b'b' => [0, 0, amt],
                // Cheeky: this is basically a compile-time assertion that the input is
                // *definitely* correct (trust me bro), but it doesn't cause UB so...
                _ => unreachable!("Bad colour in input"),
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
        // also horrendous
        [max(l[0], r[0]), max(l[1], r[1]), max(l[2], r[2])]
    })
}

#[aoc(day02, part1)]
fn solve_part1(input: &[Game]) -> usize {
    const TARGET: [u32; 3] = [12, 13, 14];

    input
        .iter()
        .enumerate()
        .filter_map(|(id, game)| {
            let [r, g, b] = game_maxes(game);
            let [tr, tg, tb] = TARGET;
            (r <= tr && g <= tg && b <= tb).then_some(id + 1)
        })
        .sum()
}

#[aoc(day02, part2)]
fn solve_part2(input: &[Game]) -> u32 {
    input
        .iter()
        .map(|game| {
            let [r, g, b] = game_maxes(game);
            r * g * b
        })
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
