#![warn(clippy::pedantic)]

#[macro_use]
extern crate aoc_runner_derive;

#[cfg(test)]
fn get_input(day: u32) -> String {
    std::fs::read_to_string(format!("./input/2024/day{day}.txt"))
        .unwrap()
        .trim()
        .to_owned()
}

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day07;
aoc_lib! { year = 2024 }
