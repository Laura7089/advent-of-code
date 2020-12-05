pub struct Seat {
    pub row: u32,
    pub column: u32,
    pub id: u32,
}

impl Seat {
    pub fn from_specifier(spec: &str) -> Result<Self, std::num::ParseIntError> {
        let id = spec
            .chars()
            .enumerate()
            .map(|(i, c)| match c {
                'B' | 'R' => 2_u32.pow(9 - (i as u32)),
                'F' | 'L' => 0,
                _ => panic!("Bad character in input"),
            })
            .sum();

        Ok(Self {
            row: id >> 3,
            column: id & 7,
            id,
        })
    }
}

#[aoc_generator(day5)]
pub fn parse_input(input: &str) -> Vec<Seat> {
    input
        .lines()
        .map(|l| Seat::from_specifier(l).unwrap())
        .collect()
}

#[aoc(day5, part1)]
pub fn solve_input_part1(data: &[Seat]) -> u32 {
    data.iter().map(|s| s.id).max().unwrap()
}

#[aoc(day5, part2)]
pub fn solve_input_part2(data: &[Seat]) -> u32 {
    let mut ids: Vec<u32> = data.iter().map(|s| s.id).collect();
    ids.sort();

    for i in 0..ids.len() - 1 {
        if ids[i + 1] - ids[i] == 2 {
            return ids[i] + 1;
        }
    }
    panic!("No answer found for day 5, part 2");
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn my_input_part1() {
        assert_eq!(
            solve_input_part1(&parse_input(
                &std::fs::read_to_string("./input/2020/day5.txt").unwrap()
            )),
            994
        );
    }

    #[test]
    fn my_input_part2() {
        assert_eq!(
            solve_input_part2(&parse_input(
                &std::fs::read_to_string("./input/2020/day5.txt").unwrap()
            )),
            741
        );
    }
}
