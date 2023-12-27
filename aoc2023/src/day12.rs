#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

type Row = (Vec<SpringState>, Vec<usize>);

fn parse_lit(raw: &str) -> Vec<SpringState> {
    raw.bytes()
        .map(|b| match b {
            b'#' => SpringState::Damaged,
            b'.' => SpringState::Operational,
            b'?' => SpringState::Unknown,
            _ => panic!("unknown spring char {}", b as char),
        })
        .collect()
}

#[aoc_generator(day12)]
fn generate(input: &str) -> Vec<Row> {
    input
        .lines()
        .map(|line| {
            let (lit_raw, list_raw) = line.split_once(' ').expect("no space in line");

            (
                parse_lit(lit_raw),
                list_raw.split(',').map(|n| n.parse().unwrap()).collect(),
            )
        })
        .collect()
}

#[derive(Clone)]
struct DamageIter<'a> {
    plot: &'a [SpringState],
    off: usize,
    damage_len: usize,
}

impl<'a> DamageIter<'a> {
    fn new(plot: &'a [SpringState], damage_len: usize) -> Self {
        Self {
            plot,
            damage_len,
            off: 0,
        }
    }
}

impl<'a> Iterator for DamageIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        for (i, _) in self
            .plot
            .iter()
            .enumerate()
            .skip(self.off)
            .filter(|(_, s)| s != &&SpringState::Operational)
        {
            // Ensure (possible) gap on left
            if i != 0 && self.plot[i - 1] == SpringState::Damaged {
                continue;
            }
            let end = i + self.damage_len;
            // Ensure (possible) gap on right
            if self.plot.get(end + 1) == Some(&SpringState::Damaged) {
                continue;
            }
            // Ensure possible fit
            if !self.plot[i..end]
                .iter()
                .all(|s| matches!(s, SpringState::Damaged | SpringState::Unknown))
            {
                continue;
            }

            self.off += i;
            return Some(self.off);
        }
        None
    }
}

#[aoc(day12, part1)]
fn solve_part1(input: &[Row]) -> usize {
    input
        .iter()
        .map(|(plot, list)| {
            let mut iter = DamageIter::new(plot, *list.first().unwrap());
            todo!() as usize
        })
        .sum()
}

#[aoc(day12, part2)]
fn solve_part2(_input: &[Row]) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;
    use test_case::test_case;

    const SAMPLE_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test_case("???.###", 1 => vec![0, 1, 2])]
    #[test_case("???.###", 3 => vec![0])]
    fn test_damage_iter(seq: &str, len: usize) -> Vec<usize> {
        DamageIter::new(&parse_lit(seq), len).collect()
    }

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), 21);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part1(&generate(&crate::get_input(12))), todo!());
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
            assert_eq!(solve_part2(&generate(&crate::get_input(12))), todo!());
        }
    }
}
