#![allow(unreachable_code)]
#[macro_use]
extern crate aoc_runner_derive;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

#[cfg(test)]
fn get_input_for_day(day: usize) -> String {
    std::fs::read_to_string(format!("input/2021/day{}.txt", day))
        .unwrap()
        .trim()
        .to_string()
}

aoc_lib! {year = 2021}
