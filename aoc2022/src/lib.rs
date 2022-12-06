#[macro_use]
extern crate aoc_runner_derive;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

aoc_lib! { year = 2022 }

#[cfg(test)]
fn get_input(day: u32) -> String {
    std::fs::read_to_string(format!("./input/2022/day{day}.txt"))
        .unwrap()
        .trim()
        .to_owned()
}
