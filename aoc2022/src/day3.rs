use itertools::Itertools;

const ITEM_CARDINALITY: usize = 52;

struct Rucksack(([bool; ITEM_CARDINALITY], [bool; ITEM_CARDINALITY]));

impl Rucksack {
    fn new(input: &str) -> Self {
        let len = input.trim().bytes().len();
        assert!(len % 2 == 0, "Odd number of items in rucksack");

        let mut sack = Self(([false; ITEM_CARDINALITY], [false; ITEM_CARDINALITY]));
        for item in input.bytes().take(len / 2) {
            sack.0 .0[Self::offset(item.into())] = true;
        }
        for item in input.bytes().skip(len / 2) {
            sack.0 .1[Self::offset(item.into())] = true;
        }

        sack
    }

    fn all_items(&self) -> impl Iterator<Item = (usize, bool)> + '_ {
        self.0
             .0
            .iter()
            .zip(self.0 .1.iter())
            .enumerate()
            .map(|(i, (&left, &right))| (i, left || right))
    }

    fn offset(item: usize) -> usize {
        match item {
            65..=90 => item - 39,
            97..=122 => item - 97,
            _ => panic!("Bad item type: {item}"),
        }
    }
}

#[aoc_generator(day3)]
fn generate(input: &str) -> Vec<Rucksack> {
    input.lines().map(Rucksack::new).collect()
}

#[aoc(day3, part1)]
fn solve_part1(input: &[Rucksack]) -> usize {
    input
        .iter()
        .map(|sack| {
            sack.0
                 .0
                .into_iter()
                .zip(sack.0 .1)
                .enumerate()
                .find_map(|(i, (l, r))| if l && r { Some(i + 1) } else { None })
                .unwrap()
        })
        .sum()
}

#[aoc(day3, part2)]
fn solve_part2(input: &[Rucksack]) -> usize {
    input
        .chunks(3)
        .map(|group| {
            group[0]
                .all_items()
                .zip(group[1].all_items())
                .zip(group[2].all_items())
                .find_map(|(((i, item1), (_, item2)), (_, item3))| {
                    if item1 && item2 && item3 {
                        Some(i + 1)
                    } else {
                        None
                    }
                })
                .expect("No common item found")
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &'static str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), 157);
    }

    #[test]
    fn part1_mine() {
        assert_eq!(solve_part1(&generate(&crate::get_input(3))), 7568);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&generate(SAMPLE_INPUT)), 70);
    }

    #[test]
    fn part2_mine() {
        assert_eq!(solve_part2(&generate(&crate::get_input(3))), 2780);
    }
}
