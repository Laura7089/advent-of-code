#[derive(Debug, Copy, Clone)]
enum Operand {
    Old,
    Literal(isize),
}

impl From<&str> for Operand {
    fn from(input: &str) -> Self {
        match input.trim() {
            "old" => Self::Old,
            x => Self::Literal(x.parse().unwrap()),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Add(Operand),
    Sub(Operand),
    Mul(Operand),
    Div(Operand),
}

impl From<&str> for Operation {
    fn from(input: &str) -> Self {
        let operand = input.split(' ').nth(1).unwrap().into();
        match input.split(' ').next().unwrap() {
            "*" => Self::Mul(operand),
            "/" => Self::Div(operand),
            "+" => Self::Add(operand),
            "-" => Self::Sub(operand),
            other => panic!("Unsupported operation literal {other}"),
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    start: Vec<usize>,
    op: Operation,
    test_mod: usize,
    on_success: usize,
    on_fail: usize,
}

impl From<&str> for Monkey {
    fn from(input: &str) -> Self {
        let mut lines = input.lines().skip(1);

        Monkey {
            start: lines.next().unwrap()[18..]
                .split(", ")
                .map(|i| i.parse().unwrap())
                .collect(),
            op: lines.next().unwrap()[23..].into(),
            test_mod: lines.next().unwrap()[21..].parse().unwrap(),
            on_success: lines.next().unwrap()[29..].parse().unwrap(),
            on_fail: lines.next().unwrap()[30..].parse().unwrap(),
        }
    }
}

#[aoc_generator(day11)]
fn generate(input: &str) -> Vec<Monkey> {
    input.split("\n\n").map(From::from).collect()
}

#[aoc(day11, part1)]
fn solve_part1(input: &[Monkey]) -> usize {
    todo!()
}

#[aoc(day11, part2)]
fn solve_part2(input: &[Monkey]) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), 10605);
    }

    #[test]
    fn part1_mine() {
        assert_eq!(solve_part1(&generate(&crate::get_input(11))), todo!());
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&generate(SAMPLE_INPUT)), todo!());
    }

    #[test]
    fn part2_mine() {
        assert_eq!(solve_part2(&generate(&crate::get_input(11))), todo!());
    }
}
