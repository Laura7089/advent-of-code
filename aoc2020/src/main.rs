use std::fs;

fn main() {
    println!(
        "Day 1 part 1: {}",
        aoc2020::day1::solve_input_part1(&aoc2020::day1::parse_input(
            &fs::read_to_string("input/2020/day1.txt").expect("No data found!")
        ))
    );
    println!(
        "Day 1 part 2: {}",
        aoc2020::day1::solve_input_part2(&aoc2020::day1::parse_input(
            &fs::read_to_string("input/2020/day1.txt").expect("No data found!")
        ))
    );
}
