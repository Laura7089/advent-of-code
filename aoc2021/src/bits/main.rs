use std::env::args;
use std::error::Error;
use std::fs;
use std::time::Instant;

use aoc2021::bits;

fn main() -> Result<(), Box<dyn Error>> {
    let input_file_path = args().nth(1).expect("No input file path provided");
    let input = fs::read_to_string(input_file_path)?;

    let now = Instant::now();
    let bit_packets = bits::parse(&bits::get_bits(input.trim())).0;
    let parse_time = now.elapsed().as_micros();

    let now = Instant::now();
    let evaluated = bit_packets.eval();
    let evaluated_time = now.elapsed().as_nanos();

    let now = Instant::now();
    let total_version = aoc2021::day16::solve_part1(&bit_packets);
    let version_time = now.elapsed().as_nanos();

    println!(
        "Took {}us to parse.
The input evaluated to {} in {}ns.
The total version was {}, calculated in {}ns.",
        parse_time, evaluated, evaluated_time, total_version, version_time
    );
    Ok(())
}
