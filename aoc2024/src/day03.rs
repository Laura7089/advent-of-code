#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Instruction {
    Mul(usize, usize),
    Do,
    Dont,
}

mod parse {
    use super::Instruction;

    use winnow::{
        combinator::{alt, repeat_till},
        prelude::*,
        token::{any, take_while},
        Result,
    };

    fn number(input: &mut &str) -> Result<usize> {
        take_while(1..=3, '0'..='9').parse_to().parse_next(input)
    }

    fn mul_instr(input: &mut &str) -> Result<Instruction> {
        ("mul(", number, ',', number, ')')
            .map(|(_, l, _, r, _)| Instruction::Mul(l, r))
            .parse_next(input)
    }

    fn instr(input: &mut &str) -> Result<Instruction> {
        alt((
            "do()".value(Instruction::Do),
            "don't()".value(Instruction::Dont),
            mul_instr,
        ))
        .parse_next(input)
    }

    pub fn rubbish_then_instr(input: &mut &str) -> Result<Instruction> {
        let out: ((), Instruction) = repeat_till(0.., any, instr).parse_next(input)?;
        Ok(out.1)
    }
}

#[aoc(day03, part1)]
fn solve_part1(input: &str) -> usize {
    winnow::combinator::iterator(input, parse::rubbish_then_instr)
        .filter_map(|ins| match ins {
            Instruction::Mul(l, r) => Some(l * r),
            _ => None,
        })
        .sum()
}

#[aoc(day03, part2)]
fn solve_part2(input: &str) -> usize {
    let mut enabled = true;
    let mut total = 0;

    let mut iter = winnow::combinator::iterator(input, parse::rubbish_then_instr);
    for ins in &mut iter {
        match ins {
            Instruction::Mul(l, r) if enabled => total += l * r,
            Instruction::Do if !enabled => enabled = true,
            Instruction::Dont if enabled => enabled = false,
            _ => continue,
        }
    }

    total
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;

    const SAMPLE_INPUT_PART1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part1(SAMPLE_INPUT_PART1), 161);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part1(&crate::get_input(03)), 169021493);
        }
    }

    const SAMPLE_INPUT_PART2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part2(SAMPLE_INPUT_PART2), 48);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part2(&crate::get_input(03)), 111762583);
        }
    }
}
