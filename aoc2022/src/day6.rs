use itertools::Itertools;

const PACKET_LEN: usize = 4;

#[aoc(day6, part1)]
fn solve_part1(input: &str) -> usize {
    let mut window = [' '; PACKET_LEN];
    input
        .chars()
        .take(PACKET_LEN)
        .enumerate()
        .for_each(|(i, c)| window[i] = c);
    println!("{:?}", window);

    let mut ptr = 0;
    for (i, c) in input.chars().enumerate().skip(PACKET_LEN) {
        window[ptr] = c;
        ptr = (ptr + 1) % PACKET_LEN;
        println!("{:?}", window);

        if window.iter().unique().count() == PACKET_LEN {
            return i + 1;
        }
    }
    panic!("No valid sequence found!");
}

#[aoc(day6, part2)]
fn solve_part2(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_INPUT: &'static [(&'static str, usize)] = &[
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
    ];

    #[test]
    fn part1_example() {
        for (input, expected) in SAMPLE_INPUT {
            println!("Trying '{input}'");
            assert_eq!(solve_part1(input), *expected);
        }
    }
}
