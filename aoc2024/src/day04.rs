// There's quite a lot we could do better here.
// As with the usual Advent of Code conundrum, if we specialise on the *specific*
// problem that it gives us, then we can almost certainly squeeze more performance out of it.
// For example, with part 2, it's definitely worth investigating the avenue that we see if
// we observe that the "middle" character is unique and also a more effective way to search for
// matches - there are 2 Ms and 2 Ss per "X-MAS", but only one A. Thus, we could search for As
// to narrow down our raycast search (which is currently naive).

#[aoc_generator(day04)]
// TODO: return a Vec<&[u8]> instead
// TODO: ndarray?
fn generate(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

const P1_GOAL: &[u8] = b"XMAS";

#[inline]
fn get_limits(modi: isize, dim: usize, goal: &[u8]) -> std::ops::Range<usize> {
    // invert the modifier because limits need to encroach opposite
    // the direction of movement
    let modi = modi * -1;
    let glen = goal.len() - 1;

    let n0 = glen * modi.clamp(0, 1) as usize;
    let n1m = glen as isize * modi.clamp(-1, 0);
    let n1 = dim.saturating_add_signed(n1m);

    n0..n1
}

#[inline]
fn try_read_2way(
    input: &[Vec<u8>],
    goal: &[u8],
    x: usize,
    y: usize,
    xmod: isize,
    ymod: isize,
) -> bool {
    let glen = goal.len() as isize;

    // search forwards
    'forwards: {
        for index in 0isize..glen {
            let y_ = y.saturating_add_signed(ymod * index);
            let x_ = x.saturating_add_signed(xmod * index);
            if input[y_][x_] != goal[index as usize] {
                break 'forwards;
            }
        }
        // all letters matched :)
        return true;
    }

    // search backwards

    // first, offset our starting point
    let y = y.saturating_add_signed((glen - 1) * ymod);
    let x = x.saturating_add_signed((glen - 1) * xmod);

    for index in 0isize..glen {
        // then, work back through the offset (ie. note the -1)
        let y_ = y.saturating_add_signed(ymod * index * -1);
        let x_ = x.saturating_add_signed(xmod * index * -1);
        if input[y_][x_] != goal[index as usize] {
            return false;
        }
    }
    true
}

fn count_aligned(ymod: isize, xmod: isize, input: &[Vec<u8>]) -> usize {
    let mut acc = 0;

    let yrange = get_limits(ymod, input.len(), P1_GOAL);
    let xrange = get_limits(xmod, input[0].len(), P1_GOAL);

    for y in yrange {
        // why is Range not Copy.....
        //              \/\/\/
        for x in xrange.clone() {
            // NOTE: this will break if the search term is palindromic
            if try_read_2way(input, P1_GOAL, x, y, xmod, ymod) {
                acc += 1;
            }
        }
    }
    acc
}

#[aoc(day04, part1)]
fn solve_part1(input: &[Vec<u8>]) -> usize {
    [
        // search up and down
        count_aligned(1, 0, input),
        // search right and left
        count_aligned(0, 1, input),
        // search up-left and down-right
        count_aligned(-1, 1, input),
        // search up-right and down-left
        count_aligned(-1, -1, input),
    ]
    .into_iter()
    .sum()
}

const P2_GOAL: &[u8] = b"MAS";

#[aoc(day04, part2)]
fn solve_part2(input: &[Vec<u8>]) -> usize {
    let (height, width) = (input.len(), input[0].len());
    let mut count = 0;

    for y in 0..(height - P2_GOAL.len() + 1) {
        for x in 0..(width - P2_GOAL.len() + 1) {
            // look up-right/down-left
            if !try_read_2way(input, P2_GOAL, x, y, 1, 1) {
                continue;
            }

            // offset x
            // this might behave unexpectedly if P2_GOAL is even in length
            let xm = x + P2_GOAL.len() - 1;
            // then look up-left/down-right
            if try_read_2way(input, P2_GOAL, xm, y, -1, 1) {
                count += 1;
            }
        }
    }

    count
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
            assert_eq!(solve_part2(&generate(SAMPLE_INPUT)), 9);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part2(&generate(&crate::get_input(04))), 1796);
        }
    }
}
