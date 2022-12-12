type Elf = (u32, u32);

fn elf_range(input: &str) -> Elf {
    let mut pair = input.split('-').map(|x| x.parse().unwrap());
    (pair.next().unwrap(), pair.next().unwrap())
}

#[aoc_generator(day4)]
fn generate(input: &str) -> Vec<(Elf, Elf)> {
    input
        .lines()
        .map(|l| {
            let mut elves = l.split(',').map(elf_range);
            (elves.next().unwrap(), elves.next().unwrap())
        })
        .collect()
}

#[aoc(day4, part1)]
fn solve_part1(input: &[(Elf, Elf)]) -> usize {
    input
        .iter()
        .filter(|(l, r)| {
            let l_in_r = l.0 >= r.0 && l.1 <= r.1;
            let r_in_l = r.0 >= l.0 && r.1 <= l.1;
            l_in_r || r_in_l
        })
        .count()
}

#[aoc(day4, part2)]
fn solve_part2(input: &[(Elf, Elf)]) -> usize {
    input
        .iter()
        .filter(|(l, r)| {
            let left = (l.0)..=(l.1);
            let right = (r.0)..=(r.1);

            left.contains(&r.0)
                || left.contains(&r.1)
                || right.contains(&l.0)
                || right.contains(&l.1)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), 2);
    }

    #[test]
    fn part1_mine() {
        assert_eq!(solve_part1(&generate(&crate::get_input(4))), 475);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&generate(SAMPLE_INPUT)), 4);
    }

    #[test]
    fn part2_mine() {
        assert_eq!(solve_part2(&generate(&crate::get_input(4))), 825);
    }
}
