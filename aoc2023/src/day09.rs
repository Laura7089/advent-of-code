#[aoc_generator(day09)]
fn generate(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|l| l.split(' ').map(|v| v.parse().unwrap()).collect())
        .collect()
}

fn get_diffs(seq: &[isize]) -> Vec<isize> {
    let mut diffs = vec![0; seq.len() - 1];
    for i in 0..(seq.len() - 1) {
        diffs[i] = seq[i + 1] - seq[i];
    }
    diffs
}

#[allow(clippy::ptr_arg)]
fn all_diffs(seq: &Vec<isize>) -> Vec<Vec<isize>> {
    let mut diffs = vec![seq.clone()];
    let mut cur = 0;
    while !diffs[cur].iter().all(|x| *x == 0) {
        diffs.push(get_diffs(&diffs[cur]));
        cur += 1;
    }
    diffs
}

#[aoc(day09, part1)]
fn solve_part1(input: &[Vec<isize>]) -> isize {
    input
        .iter()
        .map(all_diffs)
        .map(|mut diffs| {
            for i in (1..diffs.len()).rev() {
                let ([.., current], [prev, ..]) = diffs.split_at_mut(i) else {
                    panic!("diffs is too small");
                };
                current.push(prev.last().unwrap() + current.last().unwrap());
            }

            *diffs[0].last().unwrap()
        })
        .sum()
}

#[aoc(day09, part2)]
fn solve_part2(input: &[Vec<isize>]) -> isize {
    input
        .iter()
        .map(all_diffs)
        .map(|mut diffs| {
            for i in (1..diffs.len()).rev() {
                let ([.., current], [prev, ..]) = diffs.split_at_mut(i) else {
                    panic!("diffs is too small");
                };
                current.insert(0, current[0] - prev[0]);
            }

            diffs[0][0]
        })
        .sum()
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;

    const SAMPLE_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), 114);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part1(&generate(&crate::get_input(09))), 1934898178);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part2(&generate(SAMPLE_INPUT)), 2);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part2(&generate(&crate::get_input(09))), 1129);
        }
    }
}
