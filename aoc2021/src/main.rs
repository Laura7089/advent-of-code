use aoc2021::*;
use std::env;
use std::fs;
use std::time::Instant;

type DayBox = Box<dyn FnOnce(&str) -> usize>;

fn main() {
    let input_dir = env::args().nth(1).unwrap_or("input/2021".to_string());

    let solves: Vec<[DayBox; 2]> = vec![
        [
            Box::new(|s| day1::solve_part1(&day1::parse_input(s))),
            Box::new(|s| day1::solve_part2(&day1::parse_input(s))),
        ],
        [
            Box::new(|s| day2::solve_part1(&day2::parse_input(s))),
            Box::new(|s| day2::solve_part2(&day2::parse_input(s))),
        ],
        [
            Box::new(|s| day3::solve_part1(s)),
            Box::new(|s| day3::solve_part2(s)),
        ],
        [
            Box::new(|s| day4::solve_part1(&day4::parse_input(s))),
            Box::new(|s| day4::solve_part2(&day4::parse_input(s))),
        ],
        [
            Box::new(|s| day5::solve_part1(&day5::parse_input(s))),
            Box::new(|s| day5::solve_part2(&day5::parse_input(s))),
        ],
        [
            Box::new(|s| day6::solve_part1(&day6::parse_input(s))),
            Box::new(|s| day6::solve_part2(&day6::parse_input(s))),
        ],
        [
            Box::new(|s| day7::solve_part1(&day7::parse_input(s))),
            Box::new(|s| day7::solve_part2(&day7::parse_input(s))),
        ],
        [
            Box::new(|s| day8::solve_part1(&day8::parse_input(s))),
            Box::new(|s| day8::solve_part2(&day8::parse_input(s))),
        ],
        [
            Box::new(|s| day9::solve_part1(&day9::parse_input(s))),
            Box::new(|s| day9::solve_part2(&day9::parse_input(s))),
        ],
        [
            Box::new(|s| day10::solve_part1(s)),
            Box::new(|s| day10::solve_part2(s)),
        ],
        [
            Box::new(|s| day11::solve_part1(&day11::parse_input(s))),
            Box::new(|s| day11::solve_part2(&day11::parse_input(s))),
        ],
    ];

    for (day, [part1, part2]) in solves.into_iter().enumerate() {
        let filename = format!("{}/day{}.txt", input_dir, day + 1);
        let mut input_string = fs::read_to_string(filename).expect("No data found!");

        // Remove trailing newlines
        if input_string.ends_with('\n') {
            input_string.pop();
            if input_string.ends_with('\r') {
                input_string.pop();
            }
        }

        let now = Instant::now();
        let part1_answer = part1(&input_string);
        let part1_elapsed = now.elapsed().as_micros();

        let now = Instant::now();
        let part2_answer = part2(&input_string);
        let part2_elapsed = now.elapsed().as_micros();

        println!(
            "Day {day} part 1: {}, in {}us\nDay {day} part 2: {}, in {}us\n",
            part1_answer,
            part1_elapsed,
            part2_answer,
            part2_elapsed,
            day = day + 1
        );
    }
}
