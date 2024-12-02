use std::{cmp::Ordering, ops::RangeInclusive};

#[aoc_generator(day02)]
fn generate(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| l.split(' ').map(|num| num.parse().unwrap()).collect())
        .collect()
}

const DESIRED_DIFF: RangeInclusive<usize> = 1..=3;

fn find_unsafe_index(report: &[usize]) -> Option<usize> {
    let desired_order = match report[0].cmp(&report[1]) {
        Ordering::Equal => return Some(1),
        o => o,
    };

    let mut last = report[0];

    for (i, num) in report.into_iter().enumerate().skip(1) {
        if last.cmp(num) != desired_order {
            return Some(i);
        }
        if !DESIRED_DIFF.contains(&last.abs_diff(*num)) {
            return Some(i);
        }
        last = *num;
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

#[aoc(day02, part2)]
fn solve_part2(input: &[Vec<usize>]) -> usize {
    input
        .into_iter()
        .filter(|rep| {
            let Some(problem_index) = find_unsafe_index(rep) else {
                return true;
            };

            let mut rep = rep.to_vec();
            rep.remove(problem_index);
            find_unsafe_index(&rep).is_none()
        })
        .count()
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;

    const SAMPLE_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

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
