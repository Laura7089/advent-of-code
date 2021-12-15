#[inline(always)]
fn bit_mask(length: usize) -> usize {
    (1 << length) - 1
}

fn most_common(nums: &[usize], pos: usize) -> usize {
    // "Round up" halving
    let threshold = (nums.len() + 1) / 2;

    let count = nums
        .iter()
        // Filters out lines with 0 at pos
        .filter(|&&line| (line >> pos) & 1 == 1)
        .count();

    (count >= threshold) as usize
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> usize {
    let bit_length = input.lines().next().unwrap().len();
    let input_parsed: Vec<usize> = input
        .lines()
        .map(|l| usize::from_str_radix(l, 2).unwrap())
        .collect();

    let mut gamma_rate = 0;
    for pos in 0..bit_length {
        let inc = most_common(&input_parsed, pos) << pos;

        gamma_rate += inc;
    }

    gamma_rate * (gamma_rate ^ bit_mask(bit_length))
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> usize {
    let bit_length = input.lines().next().unwrap().len();

    let mut oxy_vec: Vec<usize> = input
        .lines()
        .map(|l| usize::from_str_radix(l, 2).unwrap())
        .collect();
    let mut co2_vec = oxy_vec.clone();

    let mut pos = bit_length;
    while oxy_vec.len() > 1 {
        pos -= 1;
        let mc = most_common(&oxy_vec, pos);
        oxy_vec = oxy_vec
            .into_iter()
            .filter(|n| (n >> pos) & 1 == mc)
            .collect();
    }
    let oxy_rating = oxy_vec[0];

    let mut pos = bit_length;
    while co2_vec.len() > 1 {
        pos -= 1;
        let mc = most_common(&co2_vec, pos);
        co2_vec = co2_vec
            .into_iter()
            .filter(|n| (n >> pos) & 1 != mc)
            .collect();
    }
    let co2_rating = co2_vec[0];

    oxy_rating * co2_rating
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
        assert_eq!(solve_part2(&input), 6775520);
    }
}
