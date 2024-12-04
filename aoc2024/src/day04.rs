#[aoc_generator(day04)]
// TODO: return a Vec<&[u8]> instead
// TODO: ndarray?
fn generate(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

const GOAL: &[u8] = b"XMAS";

#[inline]
fn get_lower_and_modifier(modi: isize) -> (usize, isize) {
    // invert the modifier because limits need to encroach opposite
    // the direction of movement
    let modi = modi * -1;

    let n0 = (GOAL.len() - 1) * modi.clamp(0, 1) as usize;
    let n1m = (GOAL.len() - 1) as isize * modi.clamp(-1, 0);

    (n0, n1m)
}

fn count_direction(ymod: isize, xmod: isize, input: &[Vec<u8>]) -> usize {
    let mut acc = 0;

    let (y0, y1m) = get_lower_and_modifier(ymod);
    let y1 = input.len().saturating_add_signed(y1m);
    let (x0, x1m) = get_lower_and_modifier(xmod);
    let x1 = input[0].len().saturating_add_signed(x1m);

    for y in y0..y1 {
        'search: for x in x0..x1 {
            for index in 0isize..(GOAL.len() as isize) {
                let y_ = y.saturating_add_signed(ymod * index);
                let x_ = x.saturating_add_signed(xmod * index);
                if input[y_][x_] != GOAL[index as usize] {
                    continue 'search;
                }
            }
            acc += 1;
        }
    }
    acc
}

#[aoc(day04, part1)]
fn solve_part1(input: &[Vec<u8>]) -> usize {
    [
        // search down
        count_direction(-1, 0, input),
        // search left
        count_direction(0, -1, input),
        // search up
        count_direction(1, 0, input),
        // search right
        count_direction(0, 1, input),
        // search down-right
        count_direction(-1, 1, input),
        // search down-left
        count_direction(-1, -1, input),
        // search up-left
        count_direction(1, -1, input),
        // search up-right
        count_direction(1, 1, input),
    ]
    .into_iter()
    .sum()
}

#[aoc(day04, part2)]
fn solve_part2(input: &[Vec<u8>]) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;

    const SAMPLE_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), 18);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part1(&generate(&crate::get_input(04))), 2378);
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
            assert_eq!(solve_part2(&generate(&crate::get_input(04))), todo!());
        }
    }
}
