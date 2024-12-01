#[macro_use]
extern crate aoc_runner_derive;

mod day01;

#[cfg(test)]
fn get_input(day: u32) -> String {
    std::fs::read_to_string(format!("./input/2024/day{day}.txt"))
        .unwrap()
        .trim()
        .to_owned()
}

aoc_lib! { year = 2024 }