use aoc2021::*;
use std::env;
use std::fs;

fn main() {
    let input_dir = env::args().nth(1).unwrap_or("input/2021".to_string());

    let solves: [(Box<dyn FnOnce(&str) -> u32>, Box<dyn FnOnce(&str) -> u32>); 2] = [
        (
            Box::new(|s| day1::solve_part1(&day1::parse_input(s))),
            Box::new(|s| day1::solve_part2(&day1::parse_input(s))),
        ),
        (
            Box::new(|s| day2::solve_part1(&day2::parse_input(s))),
            Box::new(|s| day2::solve_part2(&day2::parse_input(s))),
        ),
    ];

    for (day, (part1, part2)) in solves.into_iter().enumerate() {
        let input_string = &fs::read_to_string(format!("{}/day{}.txt", input_dir, day + 1))
            .expect("No data found!");
        println!(
            "Day {} part 1: {}\nDay {} part 2: {}\n",
            day + 1,
            part1(&input_string),
            day + 1,
            part2(&input_string)
        );
    }
}
