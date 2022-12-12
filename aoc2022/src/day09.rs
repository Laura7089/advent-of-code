use std::collections::HashSet;

#[derive(Clone, PartialEq, Debug, Eq)]
enum Move {
    Ver(isize),
    Hor(isize),
}

#[derive(Clone, PartialEq, Debug, Hash, Eq)]
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
            (1, 2) | (2, 1) => {
                self.0 += 1;
                self.1 += 1;
            }
            (-1, 2) | (-2, 1) => {
                self.0 -= 1;
                self.1 += 1;
            }
            (1, -2) | (2, -1) => {
                self.0 += 1;
                self.1 -= 1;
            }
            (-1, -2) | (-2, -1) => {
                self.0 -= 1;
                self.1 -= 1;
            }

            _ => (),
        }
    }
}

impl Move {
    fn apply(&mut self, knot: &mut Knot) -> bool {
        println!("{self:?}");
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

#[aoc(day09, part1)]
fn solve_part1(input: &[Move]) -> usize {
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

    let mut head = Knot(0, 0);
    let mut tail = Knot(0, 0);

    for mov in input {
        let mut m = mov.clone();
        let mut cont = true;
        while cont {
            cont = !m.apply(&mut head);
            tail.follow(&head);
            println!("{head:?}, {tail:?}");
            tail_positions.insert(tail.clone());
        }
    }

    tail_positions.len()
}

#[aoc(day09, part2)]
fn solve_part2(input: &[Move]) -> usize {
    todo!()
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
    fn part2_example() {
        assert_eq!(solve_part2(&generate(SAMPLE_INPUT)), 36);
    }

    #[test]
    fn part2_mine() {
        assert_eq!(solve_part2(&generate(&crate::get_input(09))), todo!());
    }
}
