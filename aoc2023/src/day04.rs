type Card = (Vec<usize>, Vec<usize>);

mod parse {
    use nom::{
        bytes::complete::{is_not, take},
        character::complete::{newline, space1},
        combinator::{iterator, opt},
        sequence::{preceded, separated_pair, terminated, tuple},
    };

    type Result<'a, T> = nom::IResult<&'a str, T>;

    // TODO: BADBADnaughtyBADBADBADprogrammersBADgetnaughtyBADBADBADsignaturesBAD
    pub fn cards<'a>(
        input: &'a str,
    ) -> crate::PIterStr<impl FnMut(&'a str) -> Result<'a, super::Card>> {
        iterator(input, terminated(card, opt(newline)))
    }

    fn card(input: &str) -> Result<super::Card> {
        preceded(
            tuple((is_not(":"), take(1usize), space1)),
            separated_pair(num_list, tuple((take(2usize), space1)), num_list),
        )(input)
    }

    #[allow(clippy::unnecessary_wraps)]
    fn num_list(input: &str) -> Result<Vec<usize>> {
        let mut nums = Vec::with_capacity(25);
        let mut ptr = 0;

        for num in input
            .lines()
            .flat_map(|l| l.split(" | "))
            .take(1)
            .flat_map(|l| l.split(' '))
        {
            ptr += num.len() + 1;
            if let Ok(n) = num.parse() {
                nums.push(n);
            }
        }
        ptr = ptr.saturating_sub(1);

        Ok((&input[ptr..], nums))

        // All that for a drop of blood?
        // separated_list1(space1, map(nchar::u32, |v| v as usize))(input)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use test_case::test_case;

        #[test_case(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            (vec![41, 48, 83, 86, 17], vec![83, 86, 6, 31, 17, 9, 48, 53])
        )]
        #[test_case(
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            (vec![1, 21, 53, 59, 44], vec![69, 82, 63, 72, 16, 21, 14, 1])
        )]
        fn test_card(raw: &str, res: crate::day04::Card) {
            assert_eq!(card(raw), Ok(("", res)));
        }
    }
}

fn nmatches((winners, ours): &Card) -> usize {
    winners.iter().filter(|n| ours.contains(n)).count()
}

#[aoc(day04, part1)]
fn solve_part1(input: &str) -> usize {
    parse::cards(input)
        .filter_map(|c| nmatches(&c).checked_sub(1).map(|n| 1 << n))
        .sum()
}

#[aoc(day04, part2)]
fn solve_part2(input: &str) -> usize {
    let cards: Vec<_> = parse::cards(input).collect();
    let mut copies = vec![1; cards.len()];

    for (i, card) in cards.iter().enumerate() {
        let our_copies = copies[i];
        copies[(i + 1)..=(i + nmatches(card))]
            .iter_mut()
            .for_each(|c| *c += our_copies);
    }

    copies.into_iter().sum()
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;

    const SAMPLE_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part1(SAMPLE_INPUT), 13);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part1(&crate::get_input(04)), 25010);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part2(SAMPLE_INPUT), 30);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part2(&crate::get_input(04)), 9924412);
        }
    }
}
