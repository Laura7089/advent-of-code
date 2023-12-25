use itertools::Itertools;

type Coord = (usize, usize);

#[aoc_generator(day11)]
fn generate(input: &str) -> Vec<Coord> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes()
                .enumerate()
                .filter(|(_, c)| *c == b'#')
                .map(move |(x, _)| (x, y))
        })
        .collect()
}

fn apply_offset(mut vals: Vec<&mut usize>, factor: usize) {
    vals.sort_unstable();

    // let mut blanks = Vec::new();
    let mut total_diff = 0;
    let mut new_vals = Vec::with_capacity(vals.len());
    new_vals.push(*vals[0]);
    for i in 0..(vals.len().checked_sub(1).expect("empty sequence")) {
        let diff = *vals[i + 1] - *vals[i];
        if diff > 1 {
            total_diff += (diff - 1) * (factor - 1);
        }
        new_vals.push(*vals[i + 1] + total_diff);
    }

    for i in 0..vals.len() {
        *vals[i] = new_vals[i];
    }
}

fn expand_and_dist(input: &[(usize, usize)], factor: usize) -> usize {
    let mut coords = input.to_vec();

    let (xs, ys): (Vec<_>, Vec<_>) = coords.iter_mut().map(|c| (&mut c.0, &mut c.1)).unzip();
    apply_offset(xs, factor);
    apply_offset(ys, factor);

    coords
        .into_iter()
        .combinations(2)
        .map(|comb| crate::manhattan_dist(comb[0], comb[1]))
        .sum()
}

#[aoc(day11, part1)]
fn solve_part1(input: &[(usize, usize)]) -> usize {
    expand_and_dist(input, 2)
}

#[aoc(day11, part2)]
fn solve_part2(input: &[(usize, usize)]) -> usize {
    expand_and_dist(input, 1_000_000)
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;

    const SAMPLE_INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), 374);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part1(&generate(&crate::get_input(11))), 9418609);
        }
    }

    mod part2 {
        use super::*;

        // Okay AoC, just don't provide a value for the example input in part 2

        #[test]
        fn mine() {
            assert_eq!(solve_part2(&generate(&crate::get_input(11))), 593821230983);
        }
    }
}
