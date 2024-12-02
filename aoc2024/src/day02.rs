use std::{cmp::Ordering, ops::RangeInclusive};

#[aoc_generator(day02)]
fn generate(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| l.split(' ').map(|num| num.parse().unwrap()).collect())
        .collect()
}

const DESIRED_DIFF: RangeInclusive<usize> = 1..=3;

/// Find the first pair that causes a safety violation, and return their indices.
fn find_unsafe_index(report: &[usize]) -> Option<(usize, usize)> {
    // get the ordering between the first two elements as a baseline
    let order = match report[0].cmp(&report[1]) {
        // if the first two elements are equal, we've got a problem
        Ordering::Equal => return Some((0, 1)),
        o => o,
    };

    let mut left = report[0];
    for (ri, right) in report.into_iter().enumerate().skip(1) {
        if left.cmp(right) != order || !DESIRED_DIFF.contains(&left.abs_diff(*right)) {
            return Some((ri - 1, ri));
        }
        left = *right;
    }

    None
}

#[aoc(day02, part1)]
fn solve_part1(input: &[Vec<usize>]) -> usize {
    input
        .into_iter()
        .filter(|rep| find_unsafe_index(rep).is_none())
        .count()
}

fn clone_to_buf_without<T: Clone>(buf: &mut Vec<T>, report: &[T], index: usize) {
    buf.clear();
    buf.extend_from_slice(report);
    buf.remove(index);
}

#[aoc(day02, part2)]
fn solve_part2(input: &[Vec<usize>]) -> usize {
    // reusable buffer
    let mut buf = Vec::new();

    input
        .into_iter()
        .filter(|rep| {
            let Some((left, right)) = find_unsafe_index(rep) else {
                // safe without removal
                return true;
            };

            // try removing the "problem indices"
            // TODO: iterator magic?
            clone_to_buf_without(&mut buf, rep, left);
            if find_unsafe_index(&buf).is_none() {
                return true;
            }
            clone_to_buf_without(&mut buf, rep, right);
            find_unsafe_index(&buf).is_none()
        })
        .count()
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;
    use test_case::test_case;

    const SAMPLE_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test_case(&vec![7, 6, 4, 2, 1] => None)]
    #[test_case(&vec![1, 3, 2, 4, 5] => Some((1, 2)))]
    #[test_case(&vec![8, 6, 4, 4, 1] => Some((2, 3)))]
    #[test_case(&vec![1, 3, 6, 7, 9] => None)]
    fn test_find_unsafe_index(report: &Vec<usize>) -> Option<(usize, usize)> {
        find_unsafe_index(report)
    }

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), 2);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part1(&generate(&crate::get_input(02))), 287);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part2(&generate(SAMPLE_INPUT)), 4);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part2(&generate(&crate::get_input(02))), todo!());
        }
    }
}
