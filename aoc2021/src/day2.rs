#[derive(PartialEq, Debug)]
pub enum Command {
    Forward(usize),
    Attitude(i32),
}

#[aoc_generator(day2)]
pub fn parse_input(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|l| {
            let mut split = l.split(" ");
            let command = split.next().unwrap();
            let num = split.next().unwrap().parse().unwrap();
            // Match against first letter only to avoid unneeded comparisons
            match command.chars().next().unwrap() {
                'f' => Command::Forward(num as usize),
                'u' => Command::Attitude(num * -1),
                'd' => Command::Attitude(num),
                _ => panic!("Invalid input line '{}'", l),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Command]) -> usize {
    // Rust's type inference will give (i32, usize)
    let (depth, hori) = input.iter().fold((0, 0), |(d, h), c| match c {
        Command::Forward(x) => (d, h + x),
        Command::Attitude(x) => (d + x, h),
    });
    depth as usize * hori
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Command]) -> usize {
    // Rust's type inference will give (i32, usize, i32)
    let (depth, hori, _aim) = input.iter().fold((0, 0, 0), |(d, h, a), c| match c {
        Command::Forward(x) => (d + (a * *x as i32) as usize, h + x, a),
        Command::Attitude(x) => (d, h, a + x),
    });
    depth as usize * hori
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    const EXAMPLE_PARSED: [Command; 6] = [
        Command::Forward(5),
        Command::Attitude(5),
        Command::Forward(8),
        Command::Attitude(-3),
        Command::Attitude(8),
        Command::Forward(2),
    ];

    #[test]
    fn generator() {
        assert_eq!(&parse_input(EXAMPLE_INPUT), &EXAMPLE_PARSED);
    }

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&EXAMPLE_PARSED), 150);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&EXAMPLE_PARSED), 900);
    }

    #[test]
    fn part1_myinput() {
        let input = crate::get_input_for_day(2);
        assert_eq!(solve_part1(&parse_input(&input)), 1648020);
    }

    #[test]
    fn part2_myinput() {
        let input = crate::get_input_for_day(2);
        assert_eq!(solve_part2(&parse_input(&input)), 1759818555);
    }
}
