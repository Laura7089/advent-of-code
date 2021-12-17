use std::collections::HashSet;

const FINAL_SIZE: (usize, usize) = (40, 6);

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Fold {
    X,
    Y,
}

type Input = (Vec<[usize; 2]>, Vec<(Fold, usize)>);

pub fn fold(points: &mut [[usize; 2]], (fold, line): &(Fold, usize)) {
    let idx = (fold == &Fold::Y) as usize;
    points
        .iter_mut()
        .filter(|p| p[idx] > *line)
        .for_each(|p| p[idx] = (2 * *line) - p[idx]);
}

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

    let folds = split
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut split_line = line.split_whitespace().nth(2).unwrap().split('=');
            let instr = split_line.next().unwrap();
            let val = split_line.next().unwrap().parse().unwrap();
            (
                match instr {
                    "x" => Fold::X,
                    "y" => Fold::Y,
                    _ => panic!("Bad line formatting: '{}'", line),
                },
                val,
            )
        })
        .collect();

    (pairs, folds)
}

#[aoc(day13, part1)]
pub fn solve_part1((points, instrs): &Input) -> usize {
    let mut points = points.clone();

    fold(&mut points, &instrs[0]);

    // There's probably better ways of getting uniques
    points.into_iter().collect::<HashSet<_>>().len()
}

#[aoc(day13, part2)]
pub fn solve_part2((points, instrs): &Input) -> String {
    let mut points = points.clone();

    for ins in instrs.iter() {
        fold(&mut points, ins);
    }

    let mut field = [['.'; FINAL_SIZE.0]; FINAL_SIZE.1];
    for [x, y] in points.into_iter() {
        field[y][x] = '#';
    }

    let mut visual = String::with_capacity(FINAL_SIZE.0 * FINAL_SIZE.1);
    visual.push('\n');
    for line in field.into_iter() {
        for ch in line.into_iter() {
            visual.push(ch);
        }
        visual.push('\n');
    }

    visual
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
    fn part1_myinput() {
        let _input = crate::get_input_for_day(13);
        assert_eq!(solve_part1(&parse_input(&_input)), 753);
    }

    // NOTE: no part 2 tests for this because I can't be bothered
}
