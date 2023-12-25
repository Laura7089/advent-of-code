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

fn get_offsets(seq: &[usize]) -> Vec<Coord> {
    let mut blanks = Vec::new();
    let mut total_diff = 0;
    for i in 0..(seq.len().checked_sub(1).expect("empty sequence")) {
        let diff = seq[i + 1] - seq[i];
        if diff <= 1 {
            continue;
        }
        total_diff += diff - 1;
        blanks.push((seq[i] + 1, total_diff));
    }

    blanks
}

#[aoc(day11, part1)]
fn solve_part1(input: &[(usize, usize)]) -> usize {
    let mut coords = input.to_vec();

    let (mut xs, mut ys): (Vec<_>, Vec<_>) = coords.iter().copied().unzip();
    xs.sort_unstable();
    ys.sort_unstable();

    let x_blanks = get_offsets(&xs);
    let y_blanks = get_offsets(&ys);

    for (x, y) in &mut coords {
        if let Some((_, off)) = x_blanks.iter().filter(|(t, _)| *x >= *t).last() {
            *x += off;
        }
        if let Some((_, off)) = y_blanks.iter().filter(|(t, _)| *y >= *t).last() {
            *y += off;
        }
    }

    coords
        .into_iter()
        .combinations(2)
        .map(|comb| crate::manhattan_dist(comb[0], comb[1]))
        .sum()
}

#[aoc(day11, part2)]
fn solve_part2(_input: &[(usize, usize)]) -> usize {
    todo!()
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
