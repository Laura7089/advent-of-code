pub struct BitsCounted {
    most_common: Vec<char>,
    input_raw: String,
}

#[aoc_generator(day3)]
pub fn parse_input(input: &str) -> BitsCounted {
    let length = input.lines().next().unwrap().len();
    let total_lines = input.lines().count();
    let mut ones: Vec<usize> = vec![0; length];

    for line in input.lines() {
        for (i, digit) in line.chars().rev().enumerate() {
            if digit == '1' {
                ones[i] += 1;
            }
        }
    }

    let most_common_string = ones
        .iter()
        .map(|count| if count > &(total_lines / 2) { '1' } else { '0' })
        .collect();

    BitsCounted {
        most_common: most_common_string,
        input_raw: input.to_string(),
    }
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &BitsCounted) -> usize {
    let length = &input.most_common.len();
    // let gamma_rate = usize::from_str_radix(input.most_common.into_iter().collect(), 2).unwrap();
    let gamma_rate: usize = input
        .most_common
        .iter()
        .enumerate()
        .map(|(i, d)| (*d as usize - 30) << i)
        .sum();
    let epsilon_rate = !gamma_rate & ((1 << length) - 1);
    gamma_rate * epsilon_rate
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &BitsCounted) -> usize {
    let mut oxy_vec: Vec<String> = input.input_raw.lines().map(str::to_string).collect();
    let mut co2_vec = oxy_vec.clone();

    for digit in 0..input.most_common.len() {
        oxy_vec = dbg!(oxy_vec
            .into_iter()
            .filter(|n| n.chars().nth(digit).unwrap() == dbg!(input.most_common[digit]))
            .collect());
        if oxy_vec.len() == 1 {
            break;
        }
    }

    for digit in 0..input.most_common.len() {
        co2_vec = co2_vec
            .into_iter()
            .filter(|n| n.chars().nth(digit).unwrap() != input.most_common[digit])
            .collect();
        if co2_vec.len() == 1 {
            break;
        }
    }

    usize::from_str_radix(&oxy_vec[0], 2).unwrap() * usize::from_str_radix(&co2_vec[0], 2).unwrap()
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
    fn test_part1_example() {
        assert_eq!(solve_part1(&parse_input(&INPUT)), 198);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(&parse_input(&INPUT)), 230);
    }
}
