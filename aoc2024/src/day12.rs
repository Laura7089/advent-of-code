type Garden = crate::grid::Grid<u8>;

#[aoc_generator(day12)]
fn generate(input: &str) -> Garden {
    Garden::new(input.lines().map(|line| line.bytes().collect()).collect())
}

#[aoc(day12, part1)]
fn solve_part1(_input: &Garden) -> usize {
    todo!()
}

#[aoc(day12, part2)]
fn solve_part2(_input: &Garden) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;

    const SAMPLE_INPUT2: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
    const SAMPLE_INPUT3: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    mod part1 {
        use super::*;

        #[test]
        fn example2() {
            assert_eq!(solve_part1(&generate(SAMPLE_INPUT2)), 1930);
        }

        #[test]
        fn example3() {
            assert_eq!(solve_part1(&generate(SAMPLE_INPUT3)), 1930);
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
            assert_eq!(solve_part2(&generate(SAMPLE_INPUT3)), todo!());
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part2(&generate(&crate::get_input(12))), todo!());
        }
    }
}
