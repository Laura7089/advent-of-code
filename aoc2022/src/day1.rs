#[aoc(day1, part1)]
fn solve_part1(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|elf| elf.split('\n').map(|c| c.parse::<u32>().unwrap()).sum())
        .max()
        .unwrap()
}

#[aoc(day1, part2)]
fn solve_part2(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|elf| elf.split('\n').map(|c| c.parse::<u32>().unwrap()).sum())
        .fold([0, 0, 0], |cm, item| {
            match (item > cm[0], item > cm[1], item > cm[2]) {
                (false, _, _) => cm,
                (true, false, _) => [item, cm[1], cm[2]],
                (true, true, false) => [cm[1], item, cm[2]],
                (true, true, true) => [cm[1], cm[2], item],
            }
        })
        .iter()
        .sum()

    // Imperative solution
    //     let elves: Vec<u32> = input
    //         .split("\n\n")
    //         .map(|elf| elf.split('\n').map(|c| c.parse::<u32>().unwrap()).sum())
    //         .collect();

    //     let mut maxima = [0; 3];
    //     for elf in elves {
    //         match (elf > maxima[0], elf > maxima[1], elf > maxima[2]) {
    //             (false, _, _) => (),
    //             (true, false, _) => maxima[0] = elf,
    //             (true, true, false) => {
    //                 maxima[0] = maxima[1];
    //                 maxima[1] = elf;
    //             }
    //             (true, true, true) => {
    //                 maxima.rotate_left(1);
    //                 maxima[2] = elf;
    //             }
    //         };
    //     }

    //     maxima.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &'static str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(SAMPLE_INPUT), 24_000);
    }

    #[test]
    fn part1_mine() {
        assert_eq!(solve_part1(&crate::get_input(1)), 73_211);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(SAMPLE_INPUT), 45_000);
    }

    #[test]
    fn part2_mine() {
        assert_eq!(solve_part2(&crate::get_input(1)), 213_958);
    }
}
