use std::{array, cmp::max};

type Sample = [u32; 3];

fn parse_game(game: &str) -> impl Iterator<Item = Sample> + '_ {
    game.split_once(": ")
        .expect("Bad game format")
        .1
        .split("; ")
        .map(|samp_raw| {
            let mut amts = [0; 3];
            for colour in samp_raw.split(", ") {
                let (amt_raw, colour_lit) = colour.split_once(" ").expect("Bad game format");
                let amt: u32 = amt_raw.parse().expect("Bad integer literal");
                amts[match colour_lit.as_bytes()[0] {
                    b'r' => 0,
                    b'g' => 1,
                    b'b' => 2,
                    _ => panic!("Bad colour specifier"),
                }] = amt;
            }
            amts
        })
}

fn game_maxes(game: impl Iterator<Item = Sample>) -> [u32; 3] {
    game.fold([0, 0, 0], |l, r| array::from_fn(|i| max(l[i], r[i])))
}

#[aoc(day02, part1)]
fn solve_part1(input: &str) -> usize {
    const TARGET: [u32; 3] = [12, 13, 14];

    input
        .lines()
        .map(parse_game)
        .enumerate()
        .filter_map(|(id, mut game)| {
            let [r, g, b] = game_maxes(&mut game);
            let [tr, tg, tb] = TARGET;
            (r <= tr && g <= tg && b <= tb).then_some(id + 1)
        })
        .sum()
}

#[aoc(day02, part2)]
fn solve_part2(input: &str) -> u32 {
    input
        .lines()
        .map(parse_game)
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
            assert_eq!(solve_part1(SAMPLE_INPUT), 8);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part1(&crate::get_input(02)), 2268);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part2(SAMPLE_INPUT), 2286);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part2(&crate::get_input(02)), 63542);
        }
    }
}
