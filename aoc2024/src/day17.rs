#![allow(clippy::cast_lossless)]

#[derive(Clone, Debug)]
struct Computer {
    prog: Vec<u8>,
    pc: usize,
    output: Vec<u32>,
    rega: u32,
    regb: u32,
    regc: u32,
}

mod parse {
    use nom::{
        bytes::complete::{tag, take, take_while},
        combinator::{map, map_res},
        multi::separated_list0,
        sequence::{preceded, Tuple},
    };

    use super::Computer;
    type IResult<'a, T> = nom::IResult<&'a str, T>;

    fn program(input: &str) -> IResult<Vec<u8>> {
        let prefix = tag("Program: ");
        let single_u8 = map(take(1usize), |n: &str| n.as_bytes()[0] - b'0');
        preceded(prefix, separated_list0(tag(","), single_u8))(input)
    }

    fn num(input: &str) -> IResult<u32> {
        map_res(take_while(|c: char| c.is_ascii_digit()), |raw: &str| {
            raw.parse()
        })(input)
    }

    pub fn computer(input: &str) -> IResult<Computer> {
        let (rem, (register_a, _, register_b, _, register_c)) = (
            preceded(tag("Register A: "), num),
            tag("\n"),
            preceded(tag("Register B: "), num),
            tag("\n"),
            preceded(tag("Register C: "), num),
        )
            .parse(input)?;

        let (rem, program) = preceded(tag("\n\n"), program)(rem)?;
        Ok((
            rem,
            Computer {
                prog: program,
                rega: register_a,
                regb: register_b,
                regc: register_c,
                pc: 0,
                output: Vec::new(),
            },
        ))
    }
}

#[aoc_generator(day17)]
fn generate(input: &str) -> Computer {
    parse::computer(input).expect("parse error").1
}

impl Computer {
    fn combo_opr(&self, operand: u8) -> u32 {
        match operand {
            0..=3 => operand as u32,
            4 => self.rega,
            5 => self.regb,
            6 => self.regc,
            7 => panic!("reserved operand dereferenced"),
            _ => panic!("bad combo operand byte {operand}"),
        }
    }

    fn step(&mut self) -> bool {
        let Some(&opcode) = self.prog.get(self.pc) else {
            return true;
        };
        let Some(&operand) = self.prog.get(self.pc + 1) else {
            return true;
        };

        match opcode {
            // adv
            0 => {
                self.rega /= 2u32.pow(self.combo_opr(operand));
            }
            // bxl
            1 => self.regb ^= operand as u32,
            // bst
            2 => self.regb = self.combo_opr(operand) % 8,
            // jnz
            3 => {
                if self.rega != 0 {
                    self.pc = operand as usize;
                    return false;
                }
            }
            // bxc
            4 => self.regb ^= self.regc,
            // out
            5 => self.output.push(self.combo_opr(operand) % 8),
            // bdv
            6 => self.regb = self.rega / 2u32.pow(self.combo_opr(operand)),
            // cdv
            7 => self.regc = self.rega / 2u32.pow(self.combo_opr(operand)),
            other => panic!("bad opcode {other}"),
        }

        self.pc += 2;
        false
    }

    fn run_until_halt(&mut self) {
        while !self.step() {}
    }
}

#[aoc(day17, part1)]
fn solve_part1(input: &Computer) -> String {
    let mut comp = input.clone();
    comp.run_until_halt();
    comp.output
        .into_iter()
        .map(|n| format!("{n}"))
        .collect::<Vec<String>>()
        .join(",")
}

#[aoc(day17, part2)]
fn solve_part2(_input: &Computer) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;

    const SAMPLE_INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), "4,6,3,5,6,3,5,2,1,0");
        }

        #[test]
        fn mine() {
            assert_eq!(
                solve_part1(&generate(&crate::get_input(17))),
                "3,6,3,7,0,7,0,3,0"
            );
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
            assert_eq!(solve_part2(&generate(&crate::get_input(17))), todo!());
        }
    }
}
