#![warn(clippy::pedantic)]
#![allow(clippy::inline_always)]

#[macro_use]
extern crate aoc_runner_derive;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day14;

aoc_lib! { year = 2023 }

// TODO: this is absolutely horrendous and I shouldn't have to do it
#[allow(dead_code)]
type PIterStr<'a, F> = nom::combinator::ParserIterator<&'a str, nom::error::Error<&'a str>, F>;

#[cfg(test)]
fn get_input(day: u32) -> String {
    std::fs::read_to_string(format!("./input/2023/day{day}.txt"))
        .unwrap()
        .trim()
        .to_owned()
}
