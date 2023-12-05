fn sep_nums(raw: &str) -> impl Iterator<Item = usize> + '_ {
    raw.split_ascii_whitespace()
        .map(|v| v.parse().expect("Bad integer literal"))
}

fn nmatches_raw<const N: usize>(card: &str, ours_buf: &mut [usize; N]) -> usize {
    let (winners, ours) = card
        .split_once(": ")
        .expect("Bad card format")
        .1
        .split_once(" | ")
        .expect("Bad card format");

    let mut ours = sep_nums(ours);
    *ours_buf = std::array::from_fn(|_| ours.next().expect("Too few numbers"));

    sep_nums(winners).filter(|n| ours_buf.contains(n)).count()
}

fn part1<const N: usize>(input: &str) -> usize {
    let mut buf = [0; N];
    input
        .lines()
        .filter_map(|card| nmatches_raw(card, &mut buf).checked_sub(1).map(|n| 1 << n))
        .sum()
}

fn part2<const N: usize>(input: &str) -> usize {
    let mut buf = [0; N];
    let matches: Vec<_> = input
        .lines()
        .map(|card| nmatches_raw(card, &mut buf))
        .collect();
    let mut copies = vec![1; matches.len()];

    for (i, n) in matches.iter().enumerate() {
        let our_copies = copies[i];
        copies[(i + 1)..=(i + n)]
            .iter_mut()
            .for_each(|c| *c += our_copies);
    }

    copies.into_iter().sum()
}

#[aoc(day04, part1)]
fn solve_part1(input: &str) -> usize {
    part1::<25>(input)
}

#[aoc(day04, part2)]
fn solve_part2(input: &str) -> usize {
    part2::<25>(input)
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;

    const SAMPLE_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(part1::<8>(SAMPLE_INPUT), 13);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part1(&crate::get_input(04)), 25010);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(part2::<8>(SAMPLE_INPUT), 30);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part2(&crate::get_input(04)), 9924412);
        }
    }
}
