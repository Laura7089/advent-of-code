use crate::field2d::compressed_field::CompressedField;

#[derive(Copy, Clone, Debug)]
pub enum Fold {
    X(usize),
    Y(usize),
}

type Input = (Vec<[usize; 2]>, Vec<Fold>);

#[aoc_generator(day13)]
pub fn parse_input(input: &str) -> Input {
    let mut split = input.split("\n\n");

    let pairs: Vec<[usize; 2]> = split
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut split_line = line.split(',');
            [
                split_line.next().unwrap().parse().unwrap(),
                split_line.next().unwrap().parse().unwrap(),
            ]
        })
        .collect();

    let folds: Vec<Fold> = split
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut split_line = line.split_whitespace().nth(2).unwrap().split('=');
            let instr = split_line.next().unwrap();
            let val = split_line.next().unwrap().parse().unwrap();
            match instr {
                "x" => Fold::X(val),
                "y" => Fold::Y(val),
                _ => panic!("Bad line formatting: '{}'", line),
            }
        })
        .collect();

    (pairs, folds)
}

#[aoc(day13, part1)]
pub fn solve_part1((points, instrs): &Input) -> usize {
    let max = points.iter().flat_map(|p| p.iter()).max().unwrap() + 1;
    let mut grid = CompressedField::new(vec![false; max.pow(2)], max);

    points.iter().for_each(|[x, y]| grid[(*x, *y)] = true);

    for ins in instrs.iter() {
        match ins {
            Fold::X(divide) => {
                for _x in 0..=(max - divide) {
                    for _y in 0..=max {}
                }
            }
            Fold::Y(_divide) => {}
        }
    }

    grid.field.iter().filter(|v| **v).count()
}

#[aoc(day13, part2)]
pub fn solve_part2(_input: &Input) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&parse_input(&EXAMPLE_INPUT)), 17);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&parse_input(&EXAMPLE_INPUT)), unimplemented!());
    }

    #[test]
    fn part1_myinput() {
        let _input = crate::get_input_for_day(13);
        assert_eq!(solve_part1(&parse_input(&_input)), unimplemented!());
    }

    #[test]
    fn part2_myinput() {
        let _input = crate::get_input_for_day(13);
        assert_eq!(solve_part2(&parse_input(&_input)), unimplemented!());
    }
}
