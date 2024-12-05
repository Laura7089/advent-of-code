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

#[aoc(day05, part1)]
fn solve_part1((orderings, updates): &(Vec<PageOrdering>, Vec<PageUpdate>)) -> usize {
    updates
        .iter()
        .filter_map(|update| {
            for &(first, second) in orderings {
                let Some(first_idx) = find_index(update, &first) else {
                    continue;
                };
                let Some(second_idx) = find_index(update, &second) else {
                    continue;
                };

                if first_idx > second_idx {
                    return None;
                }
            }

            Some(update[update.len() / 2])
        })
        .sum()
}

#[aoc(day05, part2)]
fn solve_part2(input: &(Vec<PageOrdering>, Vec<PageUpdate>)) -> usize {
    todo!()
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
            assert_eq!(solve_part2(&generate(SAMPLE_INPUT)), todo!());
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part2(&generate(&crate::get_input(05))), todo!());
        }
    }
}
