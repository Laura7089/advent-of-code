#[derive(Clone, Debug, PartialEq)]
pub enum Cave {
    Start,
    End,
    Large(String),
    Small(String),
}

impl Cave {
    pub fn from_str(id: &str) -> Self {
        match id {
            "start" => Self::Start,
            "end" => Self::End,
            i if i == i.to_uppercase() => Self::Large(i.to_string()),
            i if i == i.to_lowercase() => Self::Small(i.to_string()),
            i => panic!("Mixed casing in cave name: '{}'", i),
        }
    }
}

#[aoc_generator(day12)]
fn parse_input(input: &str) -> Vec<(Cave, Cave)> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split("-");
            (
                Cave::from_str(split.next().unwrap()),
                Cave::from_str(split.next().unwrap()),
            )
        })
        .collect()
}

#[aoc(day12, part1)]
fn solve_part1(_input: &[(Cave, Cave)]) -> usize {
    unimplemented!()
}

#[aoc(day12, part2)]
fn solve_part2(_input: &[(Cave, Cave)]) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&parse_input(&EXAMPLE_INPUT)), 226);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&parse_input(&EXAMPLE_INPUT)), unimplemented!());
    }

    #[test]
    fn part1_myinput() {
        let _input = crate::get_input_for_day(12);
        assert_eq!(solve_part1(&parse_input(&_input)), unimplemented!());
    }

    #[test]
    fn part2_myinput() {
        let _input = crate::get_input_for_day(12);
        assert_eq!(solve_part2(&parse_input(&_input)), unimplemented!());
    }
}
