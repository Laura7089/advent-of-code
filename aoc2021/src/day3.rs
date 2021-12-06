#[inline]
fn bit_mask(length: usize) -> usize {
    (1 << length) - 1
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> usize {
    let input: Vec<String> = input.lines().map(str::to_string).collect();
    let bit_length = input[0].len();

    let input_parsed: Vec<usize> = input.into_iter().map(|l| l.parse().unwrap()).collect();
    let threshold = input_parsed.len() >> 1;
    println!("Threshold: {}", threshold);

    let mut gamma_rate = 0;
    for pos in 0..bit_length {
        // Count the 1s in each position (note this iterates from the "right")
        let count = input_parsed
            .iter()
            // Filters out lines with 0 at pos
            .filter(|&&line| (line >> pos) & 1 == 1)
            .count();
        println!("Counted {} 1s in position {}", count, pos);

        let inc = ((count > threshold) as usize) << pos;
        println!("Adding {:011b} to gamma rate", inc);

        gamma_rate += inc;
    }

    gamma_rate * (gamma_rate ^ bit_mask(bit_length))
}

#[aoc(day3, part2)]
pub fn solve_part2(_input: &str) -> usize {
    // let mut oxy_vec: Vec<String> = input.input_raw.lines().map(str::to_string).collect();
    // let mut co2_vec = oxy_vec.clone();

    // for digit in 0..input.most_common.len() {
    //     oxy_vec = dbg!(oxy_vec
    //         .into_iter()
    //         .filter(|n| n.chars().nth(digit).unwrap() == dbg!(input.most_common[digit]))
    //         .collect());
    //     if oxy_vec.len() == 1 {
    //         break;
    //     }
    // }

    // for digit in 0..input.most_common.len() {
    //     co2_vec = co2_vec
    //         .into_iter()
    //         .filter(|n| n.chars().nth(digit).unwrap() != input.most_common[digit])
    //         .collect();
    //     if co2_vec.len() == 1 {
    //         break;
    //     }
    // }

    // usize::from_str_radix(&oxy_vec[0], 2).unwrap() * usize::from_str_radix(&co2_vec[0], 2).unwrap()
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&INPUT), 198);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&INPUT), 230);
    }

    #[test]
    fn part1_myinput() {
        let input = crate::get_input_for_day(3);
        assert_eq!(solve_part1(&input), 2261546);
    }

    #[test]
    fn part2_myinput() {
        let input = crate::get_input_for_day(3);
        assert_eq!(solve_part2(&input), unimplemented!());
    }
}
