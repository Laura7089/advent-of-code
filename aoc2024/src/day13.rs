const BUTTON_A_TOKENS: usize = 3;
const BUTTON_B_TOKENS: usize = 1;

type Button = (usize, usize);

struct Machine {
    button_a: Button,
    button_b: Button,
    prize: (usize, usize),
}

impl Machine {
    fn calculate_minimal_presses(&self) -> Option<(usize, usize)> {
        // strategy: try to make the prize by only pressing B, then progressively
        // remove Bs and fill with As until we get a valid answer

        if self.prize.0 % self.button_b.0 == 0 && self.prize.1 % self.button_b.1 == 0 {
            return Some((
                self.prize.0 / self.button_b.0,
                self.prize.1 / self.button_b.1,
            ));
        }

        let start_bs = std::cmp::max(
            (self.prize.0 / self.button_b.0) + 1,
            (self.prize.1 / self.button_b.1) + 1,
        );

        for num_bs in (0..start_bs).rev() {
            let b_xtotal = self.button_b.0 * num_bs;
            let b_ytotal = self.button_b.1 * num_bs;

            let Some(xrem) = self.prize.0.checked_sub(b_xtotal) else {
                continue;
            };
            let Some(yrem) = self.prize.1.checked_sub(b_ytotal) else {
                continue;
            };

            let num_as = xrem / self.button_a.0;

            if num_as * self.button_a.0 == xrem && num_as * self.button_a.1 == yrem {
                return Some((num_as, num_bs));
            }
        }

        None
    }
}

mod parse {
    use super::{Button, Machine};
    use nom::{
        bytes::complete::{is_a, tag, take_while1},
        character::complete::newline,
        combinator::map_res,
        multi::separated_list0,
        sequence::Tuple,
    };

    type IResult<'a, T> = nom::IResult<&'a str, T>;

    fn num(input: &str) -> IResult<usize> {
        map_res(take_while1(|c: char| c.is_ascii_digit()), |raw: &str| {
            raw.parse::<usize>()
        })(input)
    }

    fn button(input: &str) -> IResult<Button> {
        let mut parser = (
            tag("Button "),
            is_a("AB"),
            tag(": X+"),
            num,
            tag(", Y+"),
            num,
        );
        let (rem, (_, _, _, xdiff, _, ydiff)) = parser.parse(input)?;
        Ok((rem, (xdiff, ydiff)))
    }

    fn prize(input: &str) -> IResult<(usize, usize)> {
        let mut parser = (tag("Prize: X="), num, tag(", Y="), num);
        let (rem, (_, x, _, y)) = parser.parse(input)?;
        Ok((rem, (x, y)))
    }

    fn machine(input: &str) -> IResult<Machine> {
        let mut parser = (button, newline, button, newline, prize);
        let (rem, (button_a, _, button_b, _, prize)) = parser.parse(input)?;
        Ok((
            rem,
            Machine {
                button_a,
                button_b,
                prize,
            },
        ))
    }

    pub fn parse(input: &str) -> IResult<Vec<Machine>> {
        separated_list0(tag("\n\n"), machine)(input)
    }
}

#[aoc_generator(day13)]
fn generate(input: &str) -> Vec<Machine> {
    parse::parse(input).expect("parse error").1
}

#[aoc(day13, part1)]
fn solve_part1(input: &[Machine]) -> usize {
    input
        .iter()
        .filter_map(Machine::calculate_minimal_presses)
        .map(|(a_presses, b_presses)| (a_presses * BUTTON_A_TOKENS) + (b_presses * BUTTON_B_TOKENS))
        .sum()
}

#[aoc(day13, part2)]
fn solve_part2(_input: &[Machine]) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;

    const SAMPLE_INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), 480);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part1(&generate(&crate::get_input(13))), 36571);
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
            assert_eq!(solve_part2(&generate(&crate::get_input(13))), todo!());
        }
    }
}
