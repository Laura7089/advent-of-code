#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> usize {
    // Assume all MyBitVecs are the same length
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

    let mut gamma_rate = 0_usize;
    for o in 0..length {
        if ones[o] > total_lines / 2 {
            gamma_rate |= 1 << o;
        }
    }
    // epsilon_rate = NOT gamma_rate AND (bitmask of `length` bits)
    let epsilon_rate = !gamma_rate & ((1 << length) - 1);
    gamma_rate * epsilon_rate
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
        assert_eq!(solve_part1(INPUT), 198);
    }
}
