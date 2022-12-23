use nom::Finish;
type Move = (u8, u8, u8);
type Stacks = Vec<Vec<char>>;

mod parse {
    use super::{Move, Stacks};
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{anychar, char, digit1, newline as le, space0},
        combinator::{map, opt, value},
        multi::separated_list1 as seplist,
        sequence::{delimited, preceded, separated_pair, terminated, tuple},
        IResult,
    };

    fn stack_item(input: &str) -> IResult<&str, Option<char>> {
        let correct_num = map(delimited(char('['), anychar, char(']')), Some);
        let blank = value(None, tag("   "));
        alt((correct_num, blank))(input)
    }

    fn stack_row(input: &str) -> IResult<&str, Vec<Option<char>>> {
        terminated(seplist(char(' '), stack_item), space0)(input)
    }

    fn label_row(input: &str) -> IResult<&str, Vec<&str>> {
        seplist(char(' '), delimited(char(' '), digit1, opt(char(' '))))(input)
    }

    fn stacks(input: &str) -> IResult<&str, Stacks> {
        let (input, rows) = terminated(seplist(le, stack_row), le)(input)?;

        let mut stacks = vec![Vec::with_capacity(rows.len()); rows.last().unwrap().len()];

        for row in rows.into_iter().rev() {
            for (stack, item) in row.into_iter().enumerate().filter(|x| x.1.is_some()) {
                stacks[stack].push(item.unwrap());
            }
        }

        Ok((label_row(input)?.0, stacks))
    }

    fn move_single(input: &str) -> IResult<&str, Move> {
        use nom::character::complete::u8;
        tuple((
            preceded(tag("move "), u8),
            map(preceded(tag(" from "), u8), |x| x - 1),
            map(preceded(tag(" to "), u8), |x| x - 1),
        ))(input)
    }

    pub fn both(input: &str) -> IResult<&str, (Stacks, Vec<Move>)> {
        separated_pair(stacks, tag("\n\n"), seplist(le, move_single))(input)
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;

        #[test_case("   " => ("", None))]
        #[test_case("[a]" => ("", Some('a')))]
        #[test_case("r  " => panics)]
        fn stack_item(input: &str) -> (&str, Option<char>) {
            super::stack_item(input).unwrap()
        }

        #[test_case("    [a] [b]" => ("", vec![None, Some('a'), Some('b')]))]
        #[test_case("     " => ("", vec![None]))]
        #[test_case("    [a] [b]\n" => ("\n", vec![None, Some('a'), Some('b')]))]
        fn stack_row(input: &str) -> (&str, Vec<Option<char>>) {
            super::stack_row(input).unwrap()
        }

        #[test_case(" 1   2   3   4" => ("", vec!["1", "2", "3", "4"]))]
        #[test_case("  1   2   3   4" => panics)]
        fn label_row(input: &str) -> (&str, Vec<&str>) {
            super::label_row(input).unwrap()
        }

        #[test_case(
            "    [D]\n[N] [C]\n[Z] [M] [P]\n 1   2   3"
            => ("", vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']])
            ; "example input"
        )]
        #[test_case("[D]\n[C]\n 1" => ("", vec![vec!['C', 'D']]); "small")]
        fn stacks(input: &str) -> (&str, Vec<Vec<char>>) {
            super::stacks(input).unwrap()
        }

        #[test_case("move 10 from 2 to 3" => ("", (10, 1, 2)))]
        #[test_case("move 21 from 8 to 1" => ("", (21, 7, 0)))]
        #[test_case("move 10a from 2 to 3" => panics; "erroneous char")]
        fn move_single(input: &str) -> (&str, super::Move) {
            super::move_single(input).unwrap()
        }
    }
}

#[aoc_generator(day5)]
fn generate(input: &str) -> (Stacks, Vec<Move>) {
    parse::both(input).unwrap().1
}

#[aoc(day5, part1)]
fn solve_part1((stacks, sequence): &(Stacks, Vec<Move>)) -> String {
    let mut stacks = stacks.clone();

    for &(num, src, dest) in sequence.iter() {
        for _ in 0..num {
            let to_move = stacks[src as usize].pop().unwrap();
            stacks[dest as usize].push(to_move);
        }
    }

    stacks.into_iter().map(|s| *s.last().unwrap()).collect()
}

#[aoc(day5, part2)]
fn solve_part2((stacks, sequence): &(Stacks, Vec<Move>)) -> String {
    let mut stacks = stacks.clone();

    for &(num, src, dest) in sequence.iter() {
        let num = num as usize;
        let src_len = stacks[src as usize].len();
        let to_move = stacks[src as usize].split_at(src_len - num).1.to_owned();
        stacks[dest as usize].extend_from_slice(&to_move);
        stacks[src as usize].truncate(src_len - num);
    }

    stacks.into_iter().map(|s| *s.last().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part1_example() {
        assert_eq!(&solve_part1(&generate(SAMPLE_INPUT)), "CMZ");
    }

    #[test]
    fn part1_mine() {
        assert_eq!(&solve_part1(&generate(&crate::get_input(5))), "BZLVHBWQF");
    }

    #[test]
    fn part2_example() {
        assert_eq!(&solve_part2(&generate(SAMPLE_INPUT)), "MCD");
    }

    #[test]
    fn part2_mine() {
        assert_eq!(&solve_part2(&generate(&crate::get_input(5))), "TDGJQTZSL");
    }
}
