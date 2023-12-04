#![warn(clippy::pedantic)]
#![allow(clippy::inline_always)]

#[macro_use]
extern crate aoc_runner_derive;

mod day01;
mod day02;
mod day03;
mod day04;

aoc_lib! { year = 2023 }

#[cfg(test)]
fn get_input(day: u32) -> String {
    std::fs::read_to_string(format!("./input/2023/day{day}.txt"))
        .unwrap()
        .trim()
        .to_owned()
}
