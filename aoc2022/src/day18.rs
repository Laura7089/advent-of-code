use ndarray::{Array3, Axis};

type Drop = Array3<bool>;

#[aoc_generator(day18)]
fn generate(input: &str) -> Drop {
    let cubes: Vec<_> = input
        .lines()
        .map(|l| {
            // Note that we subtract one to make it zero-based
            let mut line = l.split(',').map(|n| n.parse::<usize>().unwrap() - 1);
            (
                line.next().unwrap(),
                line.next().unwrap(),
                line.next().unwrap(),
            )
        })
        .collect();

    let max = cubes.iter().flat_map(|&(x, y, z)| [x, y, z]).max().unwrap();

    let mut drop = Array3::from_elem((max, max, max), false);
    for cube in cubes {
        drop[cube] = true;
    }
    drop
}

#[aoc(day18, part1)]
fn solve_part1(input: &Drop) -> usize {
    let mut area = 0;

    for axis in (0..3).map(Axis) {
        for slice in input.axis_iter(axis) {}
    }
    todo!()
}

#[aoc(day18, part2)]
fn solve_part2(_input: &Drop) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;

    const SAMPLE_INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), 64);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part1(&generate(&crate::get_input(18))), todo!());
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
            assert_eq!(solve_part2(&generate(&crate::get_input(18))), todo!());
        }
    }
}
