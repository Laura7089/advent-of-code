#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

type Row = (Vec<SpringState>, Vec<usize>);

#[aoc_generator(day12)]
fn generate(input: &str) -> Vec<Row> {
    input
        .lines()
        .map(|line| {
            let (lit_raw, list_raw) = line.split_once(' ').expect("no space in line");

            (
                lit_raw
                    .bytes()
                    .map(|b| match b {
                        b'#' => SpringState::Damaged,
                        b'.' => SpringState::Operational,
                        b'?' => SpringState::Unknown,
                        _ => panic!("unknown spring char {}", b as char),
                    })
                    .collect(),
                list_raw.split(',').map(|n| n.parse().unwrap()).collect(),
            )
        })
        .collect()
}

#[aoc(day12, part1)]
fn solve_part1(_input: &[Row]) -> usize {
    todo!()
}

#[aoc(day12, part2)]
fn solve_part2(_input: &[Row]) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;

    const SAMPLE_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

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
