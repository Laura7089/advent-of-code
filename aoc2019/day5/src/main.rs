mod intcode_processor;

use std::fs;

fn main() -> Result<(), String> {
    let mut raw = fs::read_to_string("data.txt").unwrap();
    // Remove newline
    raw.pop();
    let data: Vec<i32> = raw.split(",").map(|x| x.parse().unwrap()).collect();
    let mut computer = intcode_processor::IntcodeComputer::new(data, std::io::stdin());
    computer.step();
    Ok(())
}
