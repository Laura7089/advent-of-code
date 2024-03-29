use ndarray::prelude::*;
#[cfg(feature = "rayon")]
use rayon::prelude::*;

type Space = u8;
const SLIDING: u8 = b'O';
const STATIC: u8 = b'#';

#[aoc_generator(day14)]
fn generate(input: &str) -> Array2<Space> {
    let line_len = input.find('\n').unwrap_or(input.len());
    let num_lines = input.len() / line_len;

    Array1::from_iter(input.bytes().filter(|c| c != &b'\n'))
        .into_shape((line_len, num_lines))
        .unwrap()
}

// Ugly function signature because the addition isn't always necessary so we take a mut
// reference and do nothing with it if we don't need to add
#[inline(always)]
fn sum_sliders(total: &mut usize, len: usize, last_static: usize, num_sliders: usize) {
    if num_sliders == 0 {
        return;
    }
    let high = len - last_static;
    let low = high - num_sliders + 1;
    *total += (low..=high).sum::<usize>();
}

#[aoc(day14, part1)]
fn solve_part1(input: &Array2<Space>) -> usize {
    let iter = input.axis_iter(Axis(1));

    #[cfg(feature = "rayon")]
    let iter = iter.into_par_iter();

    iter.map(|col| {
        // Iterate from the "north"
        let (sliders, last_static, mut total) =
                // For reasons beyond my mortal understanding,
                // It's faster to iterate over indices than enumerated elements
                (0..col.len()).fold((0, 0, 0), |(s, ls, mut sum), ptr| {
                    match col[ptr] {
                        STATIC => {
                            sum_sliders(&mut sum, col.len(), ls, s);
                            // Reset counters
                            (0, ptr + 1, sum)
                        }
                        // Count sliding rocks
                        SLIDING => (s + 1, ls, sum),
                        // Do nothing
                        _ => (s, ls, sum),
                    }
                });
        sum_sliders(&mut total, col.len(), last_static, sliders);
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
