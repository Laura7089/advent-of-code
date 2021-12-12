const OUTPUT_DIGITS: usize = 4;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DigitCode {
    segs: [bool; 7],
}

impl DigitCode {
    fn new() -> Self {
        Self { segs: [false; 7] }
    }
    fn from_str(input: &str) -> Self {
        let mut segs = [false; 7];
        input.bytes().for_each(|c| segs[c as usize - 97] = true);
        Self { segs }
    }

    fn len(&self) -> usize {
        self.segs.iter().filter(|e| **e).count()
    }

    fn is_subset(&self, other: &Self) -> bool {
        for i in 0..7 {
            if self.segs[i] && !other.segs[i] {
                return false;
            }
        }
        return true;
    }

    fn diff(&self, other: &Self) -> usize {
        let mut count = 0;
        for i in 0..7 {
            if self.segs[i] != other.segs[i] {
                count += 1;
            }
        }
        count
    }
}

#[derive(Debug, Copy, Clone)]
pub struct InputLine {
    digit_codes: [DigitCode; 10],
    output: [DigitCode; OUTPUT_DIGITS],
}

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Vec<InputLine> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(" | ");
            let digit_codes = split.next().expect("Bad formatting");
            let output_values = split.next().expect("Bad formatting");

            InputLine {
                digit_codes: digit_codes
                    .split(" ")
                    .map(DigitCode::from_str)
                    .collect::<Vec<_>>()
                    .try_into()
                    .expect("Wrong number of unique codes"),
                output: output_values
                    .split_whitespace()
                    .map(DigitCode::from_str)
                    .collect::<Vec<_>>()
                    .try_into()
                    .expect("Wrong number of output digits"),
            }
        })
        .collect()
}

#[aoc(day8, part1)]
fn solve_part1(input: &[InputLine]) -> usize {
    let input: Vec<InputLine> = input.to_vec();
    input
        .into_iter()
        .map(|i| i.output.into_iter())
        .flatten()
        .filter(|elem| {
            let length = elem.len();
            length == 2 || length == 4 || length == 3 || length == 7
        })
        .count()
}

#[aoc(day8, part2)]
fn solve_part2(input: &[InputLine]) -> usize {
    let input: Vec<InputLine> = input.to_vec();
    input
        .into_iter()
        .map(|line| {
            let mut digits = [DigitCode::new(); 10];
            // Unpack input struct
            let InputLine {
                digit_codes,
                output,
            } = line;
            let mut digit_codes: Vec<DigitCode> = digit_codes.to_vec();

            // We use a 3-step method to match the digits
            // We pop them as we match them
            // Each step depends on the matches from the last
            // After each step, we check that we got them all

            // Step 1: "Easy digits"
            for i in (0..digit_codes.len()).rev() {
                match digit_codes[i].len() {
                    2 => digits[1] = digit_codes.remove(i),
                    3 => digits[7] = digit_codes.remove(i),
                    4 => digits[4] = digit_codes.remove(i),
                    7 => digits[8] = digit_codes.remove(i),
                    _ => (),
                }
            }

            assert_eq!(digit_codes.len(), 6);

            // Step 2: Digits based on easy digits
            for i in (0..digit_codes.len()).rev() {
                match digit_codes[i].len() {
                    5 => {
                        // Whichever one is a superset of digits[1] is 3
                        if digits[1].is_subset(&digit_codes[i]) {
                            digits[3] = digit_codes.remove(i);
                            continue;
                        }
                    }
                    6 => {
                        // 1 diff with digits[1] = 6
                        if digits[1].diff(&digit_codes[i]) == 1 {
                            digits[6] = digit_codes.remove(i);
                            continue;
                        }
                        // Superset of digits[7] = 9
                        if digits[7].is_subset(&digit_codes[i]) {
                            digits[9] = digit_codes.remove(i);
                            continue;
                        }
                    }
                    _ => panic!(),
                }
            }

            assert_eq!(digit_codes.len(), 3);

            // Step 3: The stragglers
            for i in (0..digit_codes.len()).rev() {
                match digit_codes[i].len() {
                    // Either 2 or 5
                    5 => {
                        if digit_codes[i].is_subset(&digits[9]) {
                            // 5 is a subset of 9
                            digits[5] = digit_codes.remove(i);
                            continue;
                        } else {
                            // It can only be 2
                            digits[2] = digit_codes.remove(i);
                            continue;
                        }
                    }
                    // Only 6-length left is 0
                    6 => {
                        digits[0] = digit_codes.remove(i);
                        continue;
                    }
                    _ => panic!(),
                }
            }

            assert_eq!(digit_codes.len(), 0);

            // Calculate the output number
            output
                .into_iter()
                .enumerate()
                .map(|(pos, od)| {
                    // Poor way of matching into the digits array
                    for (i, c) in digits.iter().enumerate() {
                        if &od == c {
                            // Multiply into correct position
                            let mult = (10_usize).pow(3 - pos as u32);
                            return i * mult;
                        }
                    }
                    panic!("No matching digit found");
                })
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&parse_input(&EXAMPLE_INPUT)), 26);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&parse_input(&EXAMPLE_INPUT)), 5353);
    }

    #[test]
    fn part1_myinput() {
        let input = crate::get_input_for_day(8);
        assert_eq!(solve_part1(&parse_input(&input)), 247);
    }

    #[test]
    fn part2_myinput() {
        let input = crate::get_input_for_day(8);
        assert_eq!(solve_part2(&parse_input(&input)), unimplemented!());
    }
}
