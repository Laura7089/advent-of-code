#[derive(Debug, Clone)]
pub struct InputLine {
    digit_codes: [String; 10],
    output: [String; 4],
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
                    .split_whitespace()
                    .map(str::to_string)
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
                output: output_values
                    .split_whitespace()
                    .map(str::to_string)
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            }
        })
        .collect()
}

#[aoc(day8, part1)]
fn solve_part1(input: &[InputLine]) -> usize {
    let input: Vec<InputLine> = dbg!(input.to_vec());
    input
        .into_iter()
        .map(|i| i.output.into_iter())
        .flatten()
        .filter(|elem| {
            let length = dbg!(elem.len());
            length == 2 || length == 4 || length == 3 || length == 7
        })
        .count()
}

#[aoc(day8, part2)]
fn solve_part2(_input: &[InputLine]) -> usize {
    unimplemented!()
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
        assert_eq!(solve_part2(&parse_input(&EXAMPLE_INPUT)), unimplemented!());
    }

    #[test]
    fn part1_myinput() {
        let input = crate::get_input_for_day(8);
        assert_eq!(solve_part1(&parse_input(&input)), unimplemented!());
    }

    #[test]
    fn part2_myinput() {
        let input = crate::get_input_for_day(8);
        assert_eq!(solve_part2(&parse_input(&input)), unimplemented!());
    }
}
