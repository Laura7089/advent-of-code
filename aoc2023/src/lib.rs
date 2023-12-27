#![warn(clippy::pedantic)]
#![allow(clippy::inline_always)]

#[macro_use]
extern crate aoc_runner_derive;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day07;
mod day08;
mod day09;
mod day11;
mod day12;
mod day14;

aoc_lib! { year = 2023 }

// TODO: this is absolutely horrendous and I shouldn't have to do it
#[allow(dead_code)]
type PIterStr<'a, F> = nom::combinator::ParserIterator<&'a str, nom::error::Error<&'a str>, F>;

#[inline]
fn manhattan_dist((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> usize {
    x1.abs_diff(x2) + y1.abs_diff(y2)
}

#[cfg(test)]
fn get_input(day: u32) -> String {
    std::fs::read_to_string(format!("./input/2023/day{day}.txt"))
        .unwrap()
        .trim()
        .to_owned()
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    #[test_case((4, 0), (9, 10) => 15)]
    #[test_case((1, 6), (5, 11) => 9)]
    #[test_case((0, 2), (12, 7) => 17)]
    #[test_case((0, 11), (5, 11) => 5)]
    fn test_man_dist(l: (usize, usize), r: (usize, usize)) -> usize {
        super::manhattan_dist(l, r)
    }
}
