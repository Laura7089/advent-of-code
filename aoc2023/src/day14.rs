use ndarray::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Ord, Eq)]
enum Space {
    Static,
    Moveable,
    Empty,
}

#[aoc_generator(day14)]
fn generate(input: &str) -> Array2<Space> {
    let line_len = input.find('\n').expect("No newlines in input");
    let num_lines = input.len() / line_len;

    Array1::from_iter(input.chars().filter(|c| c != &'\n').map(|c| match c {
        'O' => Space::Moveable,
        '#' => Space::Static,
        _ => Space::Empty,
    }))
    .into_shape((line_len, num_lines))
    .unwrap()
}

#[aoc(day14, part1)]
fn solve_part1(input: &Array2<Space>) -> usize {
    input
        .columns()
        .into_iter()
        .map(|col| {
            let mut total = 0;

            let mut num_moveable = 0;
            let mut last_static = 0;
            // Iterate from the "north"
            for ptr in 0..(col.len()) {
                match col[ptr] {
                    Space::Static => {
                        if num_moveable != 0 {
                            let max_sd = col.len() - last_static;
                            let min_sd = max_sd - num_moveable + 1;
                            total += (min_sd..=max_sd).sum::<usize>();
                        }
                        num_moveable = 0;
                        last_static = ptr + 1;
                    }
                    Space::Moveable => num_moveable += 1,
                    Space::Empty => {}
                }
            }

            if num_moveable != 0 {
                let max_sd = col.len() - last_static;
                let min_sd = max_sd - num_moveable + 1;
                total += (min_sd..=max_sd).sum::<usize>();
            }

            total
        })
        .sum()
}

#[aoc(day14, part2)]
fn solve_part2(_input: &Array2<Space>) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;

    const SAMPLE_INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), 136);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part1(&generate(&crate::get_input(14))), 107142);
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
            assert_eq!(solve_part2(&generate(&crate::get_input(14))), todo!());
        }
    }
}
