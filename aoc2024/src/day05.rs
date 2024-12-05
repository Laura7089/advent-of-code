use std::collections::BTreeSet;

mod parse {
    use super::{PageOrdering, PageUpdate};
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

    fn single_ordering(input: &str) -> IResult<PageOrdering> {
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

    pub fn whole_input(input: &str) -> IResult<(Vec<PageOrdering>, Vec<PageUpdate>)> {
        let orderings = separated_list1(newline, single_ordering);
        let updates = separated_list1(newline, page_update);
        let sep = tuple((newline, newline));

        separated_pair(orderings, sep, updates)(input)
    }
}

type PageOrdering = (usize, usize);
type PageUpdate = Vec<usize>;

#[inline]
fn find_index<T: PartialEq>(slice: &[T], item: &T) -> Option<usize> {
    Some(slice.iter().enumerate().find(|(_i, n)| *n == item)?.0)
}

#[aoc_generator(day05)]
fn generate(input: &str) -> (Vec<PageOrdering>, Vec<PageUpdate>) {
    parse::whole_input(input).expect("parse error").1
}

// note that irrelevant orderings are here considered to match
fn ordering_matches_update(update: &PageUpdate, &(first, second): &PageOrdering) -> bool {
    let Some(first_idx) = find_index(update, &first) else {
        return true;
    };
    let Some(second_idx) = find_index(update, &second) else {
        return true;
    };

    if first_idx > second_idx {
        return false;
    }

    true
}

#[inline]
fn is_update_valid(update: &PageUpdate, orderings: &[PageOrdering]) -> bool {
    orderings
        .iter()
        .all(|ordering| ordering_matches_update(update, ordering))
}

#[inline]
fn middle(update: &PageUpdate) -> usize {
    update[update.len() / 2]
}

#[aoc(day05, part1)]
fn solve_part1((orderings, updates): &(Vec<PageOrdering>, Vec<PageUpdate>)) -> usize {
    updates
        .iter()
        .filter_map(|update| is_update_valid(update, orderings).then(|| middle(update)))
        .sum()
}

#[derive(Ord, Eq)]
struct ValWithOrdering<'a> {
    ordering: &'a [PageOrdering],
    value: usize,
}

impl<'a> PartialOrd for ValWithOrdering<'a> {
    // assumes both sides refer to the same page ordering
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.ordering.contains(&(self.value, other.value)) {
            Some(std::cmp::Ordering::Less)
        } else if self.ordering.contains(&(other.value, self.value)) {
            Some(std::cmp::Ordering::Greater)
        } else {
            // hopefully this is unreachable otherwise we're knackered
            None
        }
    }
}

impl<'a> PartialEq for ValWithOrdering<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

#[aoc(day05, part2)]
fn solve_part2((orderings, updates): &(Vec<PageOrdering>, Vec<PageUpdate>)) -> usize {
    let mut mismatch_buf = Vec::new();
    let mut update_buf = Vec::new();

    let mut total = 0;

    for update in updates {
        mismatch_buf.clear();

        for ordering in orderings {
            if !ordering_matches_update(update, ordering) {
                mismatch_buf.push(*ordering);
            }
        }
        if mismatch_buf.len() == 0 {
            // all the orderings matched
            continue;
        }

        update_buf.clear();
        update_buf.extend_from_slice(update);

        // To do this, we need to sort the relevant numbers within the indexes they
        // currently have. Nothing else needs to move - therefore, if none of them
        // are in the middle, then we needn't bother >:)
        // We need to make assumptions of the imput:
        // - orderings are together *total*
        // - updates contain no duplicate pages

        // determine indexes of important elements
        // note BTreeSets always iterate in order
        let mut indexes: BTreeSet<_> = mismatch_buf
            .iter()
            // flatten the orderings into a list of numbers for now
            .flat_map(|(l, r)| [l, r].into_iter())
            .map(|n| find_index(update, n).unwrap())
            .collect();

        if !indexes.contains(&(update_buf.len() / 2)) {
            // the middle element isn't affected by the ordering, so don't bother
            total += middle(&update_buf);
            continue;
        }

        let mut values: Vec<_> = indexes
            .iter()
            .map(|&i| ValWithOrdering {
                ordering: &mismatch_buf,
                value: update_buf[i],
            })
            .collect();
        values.sort_unstable();

        // TODO: replace with select_nth_unstable
        for (i, ValWithOrdering { value: new_val, .. }) in indexes.into_iter().zip(values) {
            update_buf[i] = new_val;
        }

        total += middle(&update_buf);
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
