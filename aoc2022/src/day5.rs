mod parse {
    use nom::{
        branch::alt,
        bytes::complete::{tag, take},
        character::complete::line_ending,
        combinator::map,
        multi::separated_list1,
        sequence::delimited,
        IResult,
    };

    pub fn stacks(input: &str) -> IResult<&str, Vec<Vec<u8>>> {
        let stack_row = separated_list1(tag(" "), stack_item);
        let (i, rows) = separated_list1(line_ending, stack_row)(input)?;

        let max = rows.last().expect("Parser didn't return any rows").len();
        let mut stacks = vec![Vec::with_capacity(rows.len()); max];

        for row in rows.into_iter().rev() {
            for (stack, item) in row.into_iter().enumerate().filter(|(_, r)| r.is_some()) {
                stacks[stack].push(item.unwrap());
            }
        }

        Ok((i, stacks))
    }

    fn stack_item(input: &str) -> IResult<&str, Option<u8>> {
        let correct_num = map(delimited(tag("["), take(1usize), tag("]")), |d: &str| {
            Some(d.as_bytes()[0])
        });
        let blank = map(tag("   "), |_| None);
        alt((correct_num, blank))(input)
    }
}

#[aoc_generator(day5)]
fn generate(input: &str) -> (Vec<Vec<u8>>, Vec<(usize, (usize, usize))>) {
    let mut input = input.split("\n\n");
    let stacks = input.next().unwrap();
    let seq = input.next().unwrap();

    (
        parse::stacks(stacks).unwrap().1,
        seq.lines()
            .map(|line| {
                let mut line = line.split(" ").skip(1);
                let target = line.next().unwrap().parse().unwrap();
                let src: usize = line.nth(1).unwrap().parse().unwrap();
                let dest: usize = line.nth(1).unwrap().parse().unwrap();
                (target, (src - 1, dest - 1))
            })
            .collect(),
    )
}

#[aoc(day5, part1)]
fn solve_part1((stacks, sequence): &(Vec<Vec<u8>>, Vec<(usize, (usize, usize))>)) -> String {
    let mut stacks = stacks.clone();

    for &(num, (src, dest)) in sequence.into_iter() {
        for _ in 0..num {
            let to_move = stacks[src].pop().unwrap();
            stacks[dest].push(to_move);
        }
    }

    String::from_utf8(stacks.into_iter().map(|s| *s.last().unwrap()).collect()).unwrap()
}

#[aoc(day5, part2)]
fn solve_part2((stacks, sequence): &(Vec<Vec<u8>>, Vec<(usize, (usize, usize))>)) -> String {
    let mut stacks = stacks.clone();

    for &(num, (src, dest)) in sequence.into_iter() {
        let src_len = stacks[src].len();
        let to_move = stacks[src].split_at(src_len - num).1.to_owned();
        stacks[dest].extend_from_slice(&to_move);
        stacks[src].truncate(src_len - num)
    }

    String::from_utf8(stacks.into_iter().map(|s| *s.last().unwrap()).collect()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &'static str = "    [D]
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
