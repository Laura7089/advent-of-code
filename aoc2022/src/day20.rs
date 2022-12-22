use crate::helpers::{index_mod, wrapping_index};

#[aoc_generator(day20)]
fn generate(input: &str) -> Vec<(usize, isize)> {
    input
        .lines()
        .map(|l| l.parse().unwrap())
        .enumerate()
        .collect()
}

const GROVE_INDICES: &[usize] = &[1_000, 2_000, 3_000];

#[aoc(day20, part1)]
fn solve_part1(input: &[(usize, isize)]) -> isize {
    let mut input = input.to_owned();

    for j in 0..input.len() {
        let (i, n) = input[j];
        let newi = index_mod(i, n, input.len());
        input[j].0 = newi;

        let modifier = if n < 0 { 1 } else { -1 }; // Elements moving forwards or backwards?
        if i as isize + n >= 0 {
            // The element hasn't circled round
            let lower = newi.min(i);
            let upper = newi.max(i);
            for o in lower..upper {
                input[o].0 = index_mod(input[o].0, modifier, input.len());
            }
        } else {
            // The element has circled round
            for o in i..input.len() {
                input[o].0 = index_mod(input[o].0, modifier, input.len());
            }
            for o in 0..newi {
                input[o].0 = index_mod(input[o].0, modifier, input.len());
            }
        }
    }

    // TODO: inefficient as hell
    input.sort_by(|(i, _), (j, _)| i.cmp(j));
    println!("{input:?}");

    GROVE_INDICES
        .iter()
        .map(|i| wrapping_index(&mut input, *i, 0).1)
        .sum()
}

#[aoc(day20, part2)]
fn solve_part2(input: &[(usize, isize)]) -> isize {
    todo!()
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;

    const SAMPLE_INPUT: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), 3);
    }

    #[test]
    fn part1_mine() {
        assert_eq!(solve_part1(&generate(&crate::get_input(20))), todo!());
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&generate(SAMPLE_INPUT)), todo!());
    }

    #[test]
    fn part2_mine() {
        assert_eq!(solve_part2(&generate(&crate::get_input(20))), todo!());
    }
}
