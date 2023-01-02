use std::collections::HashSet;

#[derive(Clone, PartialEq, Debug, Eq)]
enum Move {
    Ver(isize),
    Hor(isize),
}

#[derive(Copy, Clone, PartialEq, Debug, Hash, Eq, Default)]
struct Knot {
    x: isize,
    y: isize,
}

impl Knot {
    fn follow(&mut self, other: &Self) {
        match (other.x - self.x, other.y - self.y) {
            // Single-dimension movements
            (x, 0) if x.abs() >= 2 => self.x += x.signum(),
            (0, y) if y.abs() >= 2 => self.y += y.signum(),

            // Multi-dimension movements
            (x, y) if (x.abs() + y.abs()) >= 3 => {
                self.x += x.signum();
                self.y += y.signum();
            }

            _ => (),
        }
    }
}

impl Move {
    fn apply(&mut self, knot: &mut Knot) -> bool {
        match self {
            Self::Ver(dy) => {
                knot.y += dy.signum();
                *dy += -dy.signum();
                dy == &0
            }
            Self::Hor(dx) => {
                knot.x += dx.signum();
                *dx += -dx.signum();
                dx == &0
            }
        }
    }

    fn magnitude(&self) -> usize {
        match self {
            Move::Hor(x) => x.unsigned_abs(),
            Move::Ver(y) => y.unsigned_abs(),
        }
    }
}

#[aoc_generator(day9)]
fn generate(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|line| {
            let value = line.split(' ').nth(1).unwrap().parse().unwrap();
            match line.split(' ').next().unwrap() {
                "R" => Move::Hor(value),
                "U" => Move::Ver(value),
                "L" => Move::Hor(-value),
                "D" => Move::Ver(-value),
                d => panic!("Direction {d} not recognised"),
            }
        })
        .collect()
}

fn run_knot_snake<const LENGTH: usize>(input: &[Move]) -> usize {
    let input = input.to_owned();
    let max_moves = input.iter().map(Move::magnitude).sum();

    // Hashing is SLOOOOOOW >:(
    // - Vectors + uniqueness comparisons are quadratic complexity, but
    //   integer equality checks are definitely a lower constant time
    // - A matrix of bools is another option but it would be enormous and we'd
    //   spend most of our time reading `false`s
    // - Perhaps some kind of nested vector with internal index offsets would
    //   be better (I'm sure there's a crate) but god I cannot be fucked
    let mut passed: HashSet<Knot> = HashSet::with_capacity(max_moves);

    let mut knots = [Knot::default(); LENGTH];

    for mut mov in input {
        let mut cont = true;
        while cont {
            cont = !mov.apply(&mut knots[0]);
            for i in 1..LENGTH {
                let this_head = knots[i - 1];
                knots[i].follow(&this_head);
            }
            passed.insert(*knots.last().unwrap());
        }
    }

    passed.len()
}

#[aoc(day9, part1)]
fn solve_part1(input: &[Move]) -> usize {
    run_knot_snake::<2>(input)
}

#[aoc(day9, part2)]
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
