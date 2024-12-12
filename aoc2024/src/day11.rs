use nom::combinator::ParserIterator;

#[aoc_generator(day11)]
fn generate(input: &str) -> Vec<usize> {
    input.split(' ').map(|num| num.parse().unwrap()).collect()
}

const PART1_ITERATIONS: usize = 25;

fn stone_blink(stone: usize, mut iterations: usize) -> usize {
    // base case: we've reached the iteration limit
    if iterations == PART1_ITERATIONS {
        return 1;
    }
    iterations += 1;

    if stone == 0 {
        return stone_blink(1, iterations);
    }

    let digits = stone.ilog10() + 1;
    if digits % 2 == 0 {
        let modifier = 10usize.pow(digits / 2);
        let left = stone / modifier;
        let right = stone % modifier;
        return stone_blink(left, iterations) + stone_blink(right, iterations);
    }

    stone_blink(stone * 2024, iterations)
}

#[aoc(day11, part1)]
fn solve_part1(stones: &Vec<usize>) -> usize {
    stones.iter().map(|&stone| stone_blink(stone, 0)).sum()
}

#[aoc(day11, part2)]
fn solve_part2(_input: &Vec<usize>) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;
    use test_case::test_case;

    const SAMPLE_INPUT: &str = "125 17";

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), 55312);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part1(&generate(&crate::get_input(11))), 213625);
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
            assert_eq!(solve_part2(&generate(&crate::get_input(11))), todo!());
        }
    }
}
