use std::cmp::Ordering;
use std::collections::BTreeSet;

mod parse {
    use super::{PageOrderFragment, PageUpdate};
    use nom::{
        bytes::complete::{tag, take_while},
        character::complete::newline,
        combinator::map_res,
        multi::separated_list1,
        sequence::{separated_pair, tuple},
    };

    type IResult<'a, T> = nom::IResult<&'a str, T>;

    fn num(input: &str) -> IResult<usize> {
        map_res(take_while(|c: char| c.is_ascii_digit()), |raw: &str| {
            raw.parse()
        })(input)
    }

    fn order_fragment(input: &str) -> IResult<PageOrderFragment> {
        separated_pair(num, tag("|"), num)(input)
    }

    fn page_update(input: &str) -> IResult<PageUpdate> {
        map_res(separated_list1(tag(","), num), |list| {
            if list.len() % 2 == 0 {
                Err("even-length page update encountered")
            } else {
                Ok(list)
            }
        })(input)
    }

    pub fn whole_input(input: &str) -> IResult<(Vec<PageOrderFragment>, Vec<PageUpdate>)> {
        let fragments = separated_list1(newline, order_fragment);
        let updates = separated_list1(newline, page_update);
        let sep = tuple((newline, newline));

        separated_pair(fragments, sep, updates)(input)
    }
}

type PageOrderFragment = (usize, usize);
type PageUpdate = Vec<usize>;

#[aoc_generator(day05)]
fn generate(input: &str) -> (Vec<PageOrderFragment>, Vec<PageUpdate>) {
    parse::whole_input(input).expect("parse error").1
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
