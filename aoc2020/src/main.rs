use std::fs;

fn main() {
    println!(
        "Day 1 part 1: {}\nDay 1 part 2: {}\n",
        aoc2020::day1::solve_input_part1(&aoc2020::day1::parse_input(
            &fs::read_to_string("input/2020/day1.txt").expect("No data found!")
        )),
        aoc2020::day1::solve_input_part2(&aoc2020::day1::parse_input(
            &fs::read_to_string("input/2020/day1.txt").expect("No data found!")
        ))
    );

    println!(
        "Day 2 part 1: {}\nDay 2 part 2: {}\n",
        aoc2020::day2::solve_input_part1(&aoc2020::day2::parse_input(
            &fs::read_to_string("input/2020/day2.txt").expect("No data found!")
        )),
        aoc2020::day2::solve_input_part2(&aoc2020::day2::parse_input(
            &fs::read_to_string("input/2020/day2.txt").expect("No data found!")
        ))
    );

    println!(
        "Day 3 part 1: {}\nDay 3 part 2: {}\n",
        aoc2020::day3::solve_input_part1(&aoc2020::day3::get_slope(
            &fs::read_to_string("input/2020/day3.txt").expect("No data found!")
        )),
        aoc2020::day3::solve_input_part2(&aoc2020::day3::get_slope(
            &fs::read_to_string("input/2020/day3.txt").expect("No data found!")
        ))
    );
}
