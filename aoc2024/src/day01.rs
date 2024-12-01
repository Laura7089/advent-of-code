mod parse {
    use nom::bytes::complete::take_while;
    use nom::character::complete::{newline, space1};
    use nom::combinator::map_res;
    use nom::multi::separated_list0;
    use nom::sequence::separated_pair;

    type Input<'a> = &'a str;
    type IResult<'a, T> = nom::IResult<Input<'a>, T>;

    fn num(input: Input) -> IResult<usize> {
        map_res(take_while(|c: char| c.is_digit(10)), |raw| {
            usize::from_str_radix(raw, 10)
        })(input)
    }

    fn line(input: Input) -> IResult<(usize, usize)> {
        separated_pair(num, space1, num)(input)
    }

    pub fn whole_input(input: Input) -> IResult<Vec<(usize, usize)>> {
        separated_list0(newline, line)(input)
    }
}

#[aoc_generator(day01)]
fn generate(input: &str) -> Vec<(usize, usize)> {
    parse::whole_input(input).unwrap().1
}

#[inline]
fn make_sorted_vecs(input: &[(usize, usize)]) -> (Vec<usize>, Vec<usize>) {
    let mut left = Vec::with_capacity(input.len());
    let mut right = Vec::with_capacity(input.len());

    for (l, r) in input.iter().copied() {
        left.push(l);
        right.push(r);
    }

    left.sort_unstable();
    right.sort_unstable();

    (left, right)
}

#[aoc(day01, part1)]
fn solve_part1(input: &[(usize, usize)]) -> usize {
    let (left, right) = make_sorted_vecs(input);

    left.into_iter()
        .zip(right.into_iter())
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

type Counts = Vec<(usize, usize)>;

// assumes the list is sorted!
fn get_counts(list: &[usize]) -> Counts {
    let mut current = None;
    let mut cur_count = 0;

    let mut counts = Vec::new();

    for num in list {
        if Some(num) == current {
            cur_count += 1;
        } else {
            if let Some(&current) = current {
                counts.push((current, cur_count));
            }

            current = Some(num);
            cur_count = 1;
        }
    }

    if let Some(&current) = current {
        counts.push((current, cur_count));
    }

    counts
}

#[aoc(day01, part2)]
fn solve_part2(input: &[(usize, usize)]) -> usize {
    let (left, right) = make_sorted_vecs(input);
    let left_counts = get_counts(&left);
    let right_counts = get_counts(&right);

    let mut right_slice = &right_counts[..];
    let mut total = 0;

    for (num, lc) in left_counts {
        if let Some((i, (_, rc))) = right_slice.iter().enumerate().find(|(_, (n, _))| num == *n) {
            total += num * lc * rc;
            right_slice = &right_slice[i..];
        }
    }

    total
}

#[cfg(test)]
mod test {
    #![allow(unreachable_code)]
    use super::*;

    const SAMPLE_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), 11);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part1(&generate(&crate::get_input(01))), 3714264);
        }
    }

    #[test]
    fn test_get_counts() {
        let mut left = vec![3, 4, 2, 1, 3, 3];

        // must sort so that get_counts works
        left.sort_unstable();

        assert_eq!(get_counts(&left), vec![(1, 1), (2, 1), (3, 3), (4, 1)]);
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part2(&generate(SAMPLE_INPUT)), 31);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part2(&generate(&crate::get_input(01))), 18805872);
        }
    }
}
