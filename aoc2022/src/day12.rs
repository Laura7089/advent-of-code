#![allow(dead_code)]
#![allow(unused_variables)]
use ndarray::Array2;

type Coord = (usize, usize);

#[aoc_generator(day12)]
fn generate(input: &str) -> (Array2<u8>, Coord, Coord) {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let mut map = Array2::zeros((height, width));
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.bytes().enumerate() {
            map[(i, j)] = match c {
                b'S' => {
                    start = (i, j);
                    b'a'
                }
                b'E' => {
                    end = (i, j);
                    b'z'
                }
                _ => c,
            } - b'a';
        }
    }
    (map, start, end)
}

#[aoc(day12, part1)]
fn solve_part1((map, start, end): &(Array2<u8>, Coord, Coord)) -> usize {
    todo!()
}

#[aoc(day12, part2)]
fn solve_part2((map, start, end): &(Array2<u8>, Coord, Coord)) -> usize {
    todo!()
}
