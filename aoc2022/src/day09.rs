use std::collections::HashSet;

#[derive(Clone, PartialEq, Debug, Eq)]
enum Move {
    Ver(isize),
    Hor(isize),
}

#[derive(Copy, Clone, PartialEq, Debug, Hash, Eq)]
struct Knot(isize, isize);

impl Knot {
    fn follow(&mut self, other: &Self) {
        match (other.0 - self.0, other.1 - self.1) {
            // Single-dimension movements
            (2, 0) => self.0 += 1,
            (-2, 0) => self.0 -= 1,
            (0, 2) => self.1 += 1,
            (0, -2) => self.1 -= 1,

            // Multi-dimension movements
            (1, 2) | (2, 1) | (2, 2) => {
                self.0 += 1;
                self.1 += 1;
            }
            (-1, 2) | (-2, 1) | (-2, 2) => {
                self.0 -= 1;
                self.1 += 1;
            }
            (1, -2) | (2, -1) | (2, -2) => {
                self.0 += 1;
                self.1 -= 1;
            }
            (-1, -2) | (-2, -1) | (-2, -2) => {
                self.0 -= 1;
                self.1 -= 1;
            }

            _ => (),
        }
    }
}

impl Move {
    fn apply(&mut self, knot: &mut Knot) -> bool {
        match self {
            Self::Ver(dy) => {
                knot.1 += dy.signum();
                *dy += dy.signum() * -1;
                dy == &0
            }
            Self::Hor(dx) => {
                knot.0 += dx.signum();
                *dx += dx.signum() * -1;
                dx == &0
            }
        }
    }
}

#[aoc_generator(day09)]
fn generate(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(' ');
            let dir = split.next().unwrap();
            let value = split.next().unwrap().parse().unwrap();
            match dir {
                "R" => Move::Hor(value),
                "U" => Move::Ver(value),
                "L" => Move::Hor(-1 * value),
                "D" => Move::Ver(-1 * value),
                _ => panic!("Direction {dir} not recognised"),
            }
        })
        .collect()
}

fn run_knot_snake<const LENGTH: usize>(input: &[Move]) -> usize {
    // TODO: hashing is SLOOOOOOW
    // Allocate enough for 50% of total move units to produce a unique coord
    let mut tail_positions: HashSet<Knot> = HashSet::with_capacity(
        input
            .iter()
            .map(|m| match m {
                Move::Hor(x) => x.abs() as usize,
                Move::Ver(y) => y.abs() as usize,
            })
            .sum::<usize>()
            / 2,
    );

    let mut knots = [Knot(0, 0); LENGTH];

    for mov in input {
        let mut m = mov.clone();
        let mut cont = true;
        while cont {
            cont = !m.apply(&mut knots[0]);
            for i in 1..LENGTH {
                let this_head = knots[i - 1];
                knots[i].follow(&this_head);
            }
            tail_positions.insert(knots[LENGTH - 1].clone());
        }
    }

    tail_positions.len()
}

#[aoc(day09, part1)]
fn solve_part1(input: &[Move]) -> usize {
    run_knot_snake::<2>(input)
}

#[aoc(day09, part2)]
fn solve_part2(input: &[Move]) -> usize {
    run_knot_snake::<10>(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), 13);
    }

    #[test]
    fn part1_mine() {
        assert_eq!(solve_part1(&generate(&crate::get_input(09))), 5960);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(solve_part2(&generate(SAMPLE_INPUT)), 1);
    }

    #[test]
    fn part2_example2() {
        assert_eq!(
            solve_part2(&generate(
                "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
            )),
            36
        );
    }

    #[test]
    fn part2_mine() {
        assert_eq!(solve_part2(&generate(&crate::get_input(09))), 2327);
    }
}
