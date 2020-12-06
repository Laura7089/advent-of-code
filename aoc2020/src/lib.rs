#[macro_use]
extern crate aoc_runner_derive;
extern crate aoc_runner;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

pub fn list_of_numbers(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

aoc_lib! { year = 2020 }
