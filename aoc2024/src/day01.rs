#[aoc_generator(day01)]
fn generate(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|l| {
            let mut split = l.split("   ");
            (
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

#[inline]
fn make_sorted_vecs(input: &[(usize, usize)]) -> (Vec<usize>, Vec<usize>) {
    let mut left = Vec::with_capacity(input.len());
    let mut right = Vec::with_capacity(input.len());

    for (l, r) in input.iter().copied() {
        left.push(l);
        right.push(r);
    }

    left.sort_unstable();
    right.sort_unstable();

    (left, right)
}

#[aoc(day01, part1)]
fn solve_part1(input: &[(usize, usize)]) -> usize {
    let (left, right) = make_sorted_vecs(input);

    left.into_iter()
        .zip(right.into_iter())
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

#[cfg(test)]
mod test {
    #![allow(unreachable_code)]
    use super::*;

    const SAMPLE_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), 11);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part1(&generate(&crate::get_input(01))), 3714264);
        }
    }
}
