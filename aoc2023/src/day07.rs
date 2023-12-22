use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand([u8; 5]);

impl Hand {
    fn from_str(input: &str) -> Self {
        Self(std::array::from_fn(|i| match input.as_bytes()[i] {
                b'A' => 14,
                b'K' => 13,
                b'Q' => 12,
                b'J' => 11,
                b'T' => 10,
                i @ b'2'..=b'9' => i - b'0',
                _ => panic!("Bad hand char"),
            } - 2))
    }

    fn score(&self) -> usize {
        let mut counts = [0; 13];
        self.0.iter().for_each(|n| counts[*n as usize] += 1);

        match counts.iter().max().unwrap() {
            // 5 of a kind
            5 => 6,
            // 4 of a kind
            4 => 5,
            // full house
            3 if counts.iter().find(|n| **n == 2).is_some() => 4,
            // three of a kind
            3 => 3,
            // two pair
            2 if counts.iter().filter(|n| **n == 2).count() == 2 => 2,
            // one pair
            2 => 1,
            // high card
            _ => 0,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let score = self.score();
        let other_score = other.score();
        if score != other_score {
            score.cmp(&other_score)
        } else {
            for (l, r) in self.0.iter().zip(other.0.iter()) {
                if l != r {
                    return l.cmp(r);
                }
            }
            Ordering::Equal
        }
    }
}

#[aoc_generator(day07)]
fn generate(input: &str) -> Vec<(Hand, usize)> {
    input
        .lines()
        .map(|line| {
            let (hand_lit, bid_lit) = line.split_once(' ').expect("Bad hand format");
            (Hand::from_str(hand_lit), bid_lit.parse().unwrap())
        })
        .collect()
}

#[aoc(day07, part1)]
fn solve_part1(input: &[(Hand, usize)]) -> usize {
    let mut hands = input.to_vec();

    hands.sort_unstable_by(|(lh, _), (rh, _)| lh.cmp(rh));

    // hands.iter().for_each(|hand| println!("{hand:?}"));

    hands
        .into_iter()
        .enumerate()
        .map(|(i, (_, b))| (i + 1) * b)
        .sum()
}

#[aoc(day07, part2)]
fn solve_part2(_input: &[(Hand, usize)]) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;
    use test_case::test_case;

    const SAMPLE_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test_case("32T3K", "T55J5" => Ordering::Less)]
    #[test_case("T55J5", "KK677" => Ordering::Greater)]
    #[test_case("KK677", "KTJJT" => Ordering::Greater)]
    #[test_case("KTJJT", "QQQJA" => Ordering::Less)]
    fn test_cmp(left: &str, right: &str) -> Ordering {
        let l = Hand::from_str(left);
        let r = Hand::from_str(right);
        l.cmp(&r)
    }

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), 6440);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part1(&generate(&crate::get_input(07))), 251545216);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part2(&generate(SAMPLE_INPUT)), todo!());
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part2(&generate(&crate::get_input(07))), todo!());
        }
    }
}
