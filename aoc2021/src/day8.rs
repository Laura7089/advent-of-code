const OUTPUT_DIGITS: usize = 4;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct DigitCode {
    pub segs: [bool; 7],
}

impl DigitCode {
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
        true
    }

    fn diff(&self, other: &Self) -> usize {
        (0..7).filter(|&i| self.segs[i] != other.segs[i]).count()
    }
}

pub type InputLine = ([DigitCode; 10], [DigitCode; OUTPUT_DIGITS]);

#[aoc_generator(day8)]
pub fn parse_input(input: &str) -> Vec<InputLine> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(" | ");
            (
                split
                    .next()
                    .expect("Bad formatting")
                    .split(' ')
                    .map(DigitCode::from_str)
                    .collect::<Vec<_>>()
                    .try_into()
                    .expect("Wrong number of unique codes"),
                split
                    .next()
                    .expect("Bad formatting")
                    .split_whitespace()
                    .map(DigitCode::from_str)
                    .collect::<Vec<_>>()
                    .try_into()
                    .expect("Wrong number of output digits"),
            )
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &[InputLine]) -> usize {
    let input: Vec<InputLine> = input.to_vec();
    input
        .into_iter()
        .flat_map(|i| i.1.into_iter())
        .filter(|elem| {
            let length = elem.len();
            length == 2 || length == 4 || length == 3 || length == 7
        })
        .count()
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &[InputLine]) -> usize {
    let mut total = 0;

    for &line in input {
        let mut digits = [DigitCode::default(); 10];
        // Unpack input struct and copy into a shrinkable Vec
        let (digit_codes, output) = line;
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
                    if digits[1].is_subset(&digit_codes[i]) {
                        // 3 is a superset of 1
                        digits[3] = digit_codes.remove(i);
                    }
                }
                6 => {
                    if digits[1].diff(&digit_codes[i]) == 6 {
                        // 6 diff with digits[1] = 6
                        digits[6] = digit_codes.remove(i);
                    } else if digits[4].is_subset(&digit_codes[i]) {
                        // 4 is a subset of 9
                        digits[9] = digit_codes.remove(i);
                    } else if digits[4].segs[digit_codes[i]
                        .segs
                        .iter()
                        .enumerate()
                        .find(|(_, d)| d == &&false)
                        .unwrap()
                        .0]
                    {
                        // The only missing seg in 0 is there in 4
                        digits[0] = digit_codes.remove(i);
                    }
                }
                _ => panic!(),
            }
        }
        assert_eq!(digit_codes.len(), 2);

        // Step 3: The stragglers
        if digit_codes[0].is_subset(&digits[9]) {
            // 5 is a subset of 9
            digits[5] = digit_codes.remove(0);
            digits[2] = digit_codes.remove(0);
        } else {
            // It can only be 2
            digits[2] = digit_codes.remove(0);
            digits[5] = digit_codes.remove(0);
        }
        assert_eq!(digit_codes.len(), 0);

        total += output
            .into_iter()
            .enumerate()
            .map(
                // Match the digits against our `digits` array, place them and sum them
                |(pos, od)| match digits.iter().position(|c| &od == c) {
                    Some(i) => i * (10_usize).pow(3 - pos as u32),
                    None => panic!("No matching digit found"),
                },
            )
            .sum::<usize>()
    }

    total
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
        assert_eq!(solve_part2(&parse_input(&EXAMPLE_INPUT)), 61229);
    }

    #[test]
    fn part1_myinput() {
        let input = crate::get_input_for_day(8);
        assert_eq!(solve_part1(&parse_input(&input)), 247);
    }

    #[test]
    fn part2_myinput() {
        let input = crate::get_input_for_day(8);
        assert_eq!(solve_part2(&parse_input(&input)), 933305);
    }

    #[test]
    fn deduction_logic() {
        let one = DigitCode {
            segs: [false, false, true, false, false, true, false],
        };
        let three = DigitCode {
            segs: [true, false, true, true, false, true, true],
        };
        let four = DigitCode {
            segs: [false, true, true, true, false, true, false],
        };
        let five = DigitCode {
            segs: [true, true, false, true, false, true, true],
        };
        let six = DigitCode {
            segs: [true, true, false, true, true, true, true],
        };
        let seven = DigitCode {
            segs: [true, false, true, false, false, true, false],
        };
        let nine = DigitCode {
            segs: [true, true, true, true, false, true, true],
        };

        assert!(one.is_subset(&three));
        assert_eq!(six.diff(&one), 6);
        assert!(seven.is_subset(&nine));
        assert!(five.is_subset(&nine));
        assert!(four.is_subset(&nine));
    }
}
