#[macro_use]
extern crate aoc_runner_derive;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;

#[cfg(test)]
fn get_input_for_day(day: usize) -> String {
    std::fs::read_to_string(format!("input/2021/day{}.txt", day)).unwrap()
}

aoc_lib! {year = 2021}
