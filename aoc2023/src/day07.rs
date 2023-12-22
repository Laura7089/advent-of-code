use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    cards: [u8; 5],
    counts: [usize; 13],
}
const JOKER: u8 = 11;

impl Hand {
    fn from_str(input: &str) -> Self {
        let cards = std::array::from_fn(|i| match input.as_bytes()[i] {
            b'A' => 14,
            b'K' => 13,
            b'Q' => 12,
            b'J' => JOKER,
            b'T' => 10,
            i @ b'2'..=b'9' => i - b'0',
            _ => panic!("Bad hand char"),
        });

        let mut counts = [0; 13];
        cards.iter().for_each(|n| counts[*n as usize - 2] += 1);

        Self { cards, counts }
    }

    #[inline(always)]
    fn score(&self, jokers: bool) -> usize {
        let mut cloned = self.clone();
        let mut max = *self.counts.iter().max().unwrap();
        if jokers {
            max += std::mem::replace(&mut cloned.counts[JOKER as usize - 2], 0);
        }

        match max {
            // 5 of a kind
            5 => 6,
            // 4 of a kind
            4 => 5,
            // full house
            3 if cloned.counts.iter().any(|n| *n == 2) => 4,
            // three of a kind
            3 => 3,
            // two pair
            2 if cloned.counts.iter().filter(|n| **n == 2).count() == 2 => 2,
            // one pair
            2 => 1,
            // high card
            _ => 0,
        }
    }

    // Implement cmp without trait so we can have the joker impl too
    fn cmp(&self, other: &Self) -> Ordering {
        let score = self.score(false);
        let other_score = other.score(false);
        if score == other_score {
            for (l, r) in self.cards.iter().zip(other.cards.iter()) {
                if l != r {
                    return l.cmp(r);
                }
            }
            Ordering::Equal
        } else {
            score.cmp(&other_score)
        }
    }

    fn cmp_joker(&self, other: &Self) -> Ordering {
        let score = self.score(true);
        let other_score = other.score(true);
        if score == other_score {
            for (mut l, mut r) in self.cards.iter().zip(other.cards.iter()) {
                if l == &JOKER {
                    l = &0;
                }
                if r == &JOKER {
                    r = &0;
                }
                if l != r {
                    return l.cmp(r);
                }
            }
            Ordering::Equal
        } else {
            score.cmp(&other_score)
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

fn total_winnings(input: &[(Hand, usize)], cmp: impl Fn(&Hand, &Hand) -> Ordering) -> usize {
    let mut hands = input.to_vec();
    hands.sort_unstable_by(|(lh, _), (rh, _)| cmp(lh, rh));

    hands
        .into_iter()
        .enumerate()
        .map(|(i, (_, b))| (i + 1) * b)
        .sum()
}

#[aoc(day07, part1)]
fn solve_part1(input: &[(Hand, usize)]) -> usize {
    total_winnings(input, Hand::cmp)
}

#[aoc(day07, part2)]
fn solve_part2(input: &[(Hand, usize)]) -> usize {
    total_winnings(input, Hand::cmp_joker)
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

    #[test_case("32T3K", "T55J5" => Ordering::Less)]
    #[test_case("T55J5", "KK677" => Ordering::Greater)]
    #[test_case("KK677", "KTJJT" => Ordering::Less)]
    #[test_case("KTJJT", "QQQJA" => Ordering::Greater)]
    #[test_case("32T3K", "KK677" => Ordering::Less)]
    #[test_case("QQQJA", "T55J5" => Ordering::Greater)]
    fn test_cmp_jkr(left: &str, right: &str) -> Ordering {
        let l = Hand::from_str(left);
        let r = Hand::from_str(right);
        l.cmp_joker(&r)
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
            assert_eq!(solve_part2(&generate(SAMPLE_INPUT)), 5905);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part2(&generate(&crate::get_input(07))), todo!());
        }
    }
}
