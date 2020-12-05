use std::env;
use std::fs;

fn main() {
    let input_dir = env::args().nth(1).unwrap_or("input/2020".to_string());

    println!(
        "Day 1 part 1: {}\nDay 1 part 2: {}\n",
        aoc2020::day1::solve_input_part1(&aoc2020::day1::parse_input(
            &fs::read_to_string(format!("{}/day1.txt", input_dir)).expect("No data found!")
        )),
        aoc2020::day1::solve_input_part2(&aoc2020::day1::parse_input(
            &fs::read_to_string(format!("{}/day1.txt", input_dir)).expect("No data found!")
        ))
    );

    println!(
        "Day 2 part 1: {}\nDay 2 part 2: {}\n",
        aoc2020::day2::solve_input_part1(&aoc2020::day2::parse_input(
            &fs::read_to_string(format!("{}/day2.txt", input_dir)).expect("No data found!")
        )),
        aoc2020::day2::solve_input_part2(&aoc2020::day2::parse_input(
            &fs::read_to_string(format!("{}/day2.txt", input_dir)).expect("No data found!")
        ))
    );

    println!(
        "Day 3 part 1: {}\nDay 3 part 2: {}\n",
        aoc2020::day3::solve_input_part1(&aoc2020::day3::get_slope(
            &fs::read_to_string(format!("{}/day3.txt", input_dir)).expect("No data found!")
        )),
        aoc2020::day3::solve_input_part2(&aoc2020::day3::get_slope(
            &fs::read_to_string(format!("{}/day3.txt", input_dir)).expect("No data found!")
        ))
    );

    println!(
        "Day 4 part 1: {}\nDay 4 part 2: {}\n",
        aoc2020::day4::solve_input_part1(&aoc2020::day4::parse_input(
            &fs::read_to_string(format!("{}/day4.txt", input_dir)).expect("No data found!")
        )),
        aoc2020::day4::solve_input_part2(&aoc2020::day4::parse_input(
            &fs::read_to_string(format!("{}/day4.txt", input_dir)).expect("No data found!")
        ))
    );

    println!(
        "Day 5 part 1: {}\nDay 5 part 2: {}\n",
        aoc2020::day5::solve_input_part1(&aoc2020::day5::parse_input(
            &fs::read_to_string(format!("{}/day5.txt", input_dir)).expect("No data found!")
        )),
        aoc2020::day5::solve_input_part2(&aoc2020::day5::parse_input(
            &fs::read_to_string(format!("{}/day5.txt", input_dir)).expect("No data found!")
        ))
    );
}
