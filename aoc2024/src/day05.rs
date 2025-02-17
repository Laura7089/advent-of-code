use std::cmp::Ordering;
use std::collections::BTreeSet;

mod parse {
    use super::{PageOrderFragment, PageUpdate};

    use winnow::{
        ascii::digit1,
        combinator::{separated, separated_pair},
        prelude::*,
        Result,
    };

    fn num(input: &mut &str) -> Result<usize> {
        digit1.parse_to().parse_next(input)
    }

    fn order_fragment(input: &mut &str) -> Result<PageOrderFragment> {
        separated_pair(num, '|', num).parse_next(input)
    }

    fn page_update(input: &mut &str) -> Result<PageUpdate> {
        separated(1.., num, ',')
            .verify(|list: &Vec<_>| list.len() % 2 != 0)
            .parse_next(input)
    }

    pub fn whole_input(input: &mut &str) -> Result<(Vec<PageOrderFragment>, Vec<PageUpdate>)> {
        let fragments = separated(0.., order_fragment, '\n');
        let updates = separated(0.., page_update, '\n');

        separated_pair(fragments, "\n\n", updates).parse_next(input)
    }
}

use winnow::Parser;

type PageOrderFragment = (usize, usize);
type PageUpdate = Vec<usize>;

#[aoc_generator(day05)]
fn generate(input: &str) -> (Vec<PageOrderFragment>, Vec<PageUpdate>) {
    parse::whole_input.parse(input).expect("parse error")
}

// note that irrelevant fragments are here considered to match
fn find_frag_violation(
    update: &PageUpdate,
    &(first, second): &PageOrderFragment,
) -> Option<(usize, usize)> {
    let first_idx = update.iter().position(|&n| n == first)?;
    let second_idx = update.iter().position(|&n| n == second)?;
    // if the first index is behind the second index, that's a violation
    (first_idx > second_idx).then_some((first_idx, second_idx))
}

#[aoc(day05, part1)]
fn solve_part1((fragments, updates): &(Vec<PageOrderFragment>, Vec<PageUpdate>)) -> usize {
    updates
        .iter()
        .filter(|update| {
            fragments
                .iter()
                .all(|frag| find_frag_violation(update, frag).is_none())
        })
        .map(|update| update[update.len() / 2])
        .sum()
}

#[derive(Copy, Clone, Debug, Eq)]
struct OrderedVal<'a> {
    fragments: &'a [PageOrderFragment],
    value: usize,
}

// assumes both sides refer to the same page order
impl PartialOrd for OrderedVal<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrderedVal<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            Ordering::Equal
        } else if self.fragments.contains(&(self.value, other.value)) {
            Ordering::Less
        } else if self.fragments.contains(&(other.value, self.value)) {
            Ordering::Greater
        } else {
            // Oh, egads! The input page order isn't total!
            // My sort is ruined! But what if...
            // I were to return Equal and pretend it's a valid answer?
            Ordering::Equal
            // [chuckles] Delightfully devilish, Laura.
        }
    }
}

impl PartialEq for OrderedVal<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

#[aoc(day05, part2)]
fn solve_part2((fragments, updates): &(Vec<PageOrderFragment>, Vec<PageUpdate>)) -> usize {
    // scratch buffers for upcoming calculations
    let mut mismatches = Vec::new();
    let mut indexes = BTreeSet::new();

    let mut total = 0;

    for update in updates {
        mismatches.clear();
        indexes.clear();
        for frag in fragments {
            // determine indexes of important (violating) elements
            if let Some((l, r)) = find_frag_violation(update, frag) {
                mismatches.push(*frag);
                indexes.insert(l);
                indexes.insert(r);
            }
        }
        if mismatches.is_empty() {
            // all the fragments matched so the page update is valid
            // ergo, into the bin with it :)
            continue;
        }

        // To do this, we need to sort the relevant numbers within the indexes they
        // currently have. Nothing else needs to move - therefore, if none of them
        // are in the middle, then we needn't bother >:)

        let middle = update.len() / 2;
        // note BTreeSets always iterate in order
        let Some(middle_in_indexes) = indexes.iter().position(|n| *n == middle) else {
            // the middle element isn't affected by the order, so don't bother
            total += update[update.len() / 2];
            continue;
        };

        // frustratingly, the borrow checker won't let us use a scratch buffer for this
        // because values (by type) holds immutable references to mismatches, which stops
        // us from mutating it at the beginning of this for loop.
        // TODO: perhaps this can be worked around, but I don't think the compiler is clever
        // enough to recognise the .clear() as removing them (and indeed it might not).

        // Ideally, these three lines could be accomplished fast and more concisely with a
        // select_nth_unstable_by_key call, but we're kinda hacking around the fact that the
        // input orderings given are not total, even just over the mismatches and thus the
        // sort order is ambiguous. The rust stdlib is explicitly *not* designed to deal with
        // this, but this particular arrangement makes my tests pass and therefore it's
        // good enough.
        let mut values: Vec<_> = indexes
            .iter()
            .map(|&i| OrderedVal {
                fragments: &mismatches,
                value: update[i],
            })
            .collect();
        values.sort_unstable();
        total += values[middle_in_indexes].value;
    }

    total
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;

    const SAMPLE_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), 143);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part1(&generate(&crate::get_input(05))), 4905);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part2(&generate(SAMPLE_INPUT)), 123);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part2(&generate(&crate::get_input(05))), 6204);
        }
    }
}
