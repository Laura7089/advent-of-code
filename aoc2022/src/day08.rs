#![allow(clippy::reversed_empty_ranges)]

use aoc_helpers::UPoint as Point;
use ndarray::prelude::*;

#[aoc_generator(day8)]
fn generate(input: &str) -> Array2<u8> {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let mut array = Array2::zeros((width, height));

    for (j, row) in input.lines().enumerate() {
        for (i, tree) in row.bytes().enumerate() {
            array[(i, j)] = tree - b'0';
        }
    }

    array
}

fn visible<T>(input: impl Iterator<Item = (T, u8)>) -> impl Iterator<Item = T> {
    input
        .scan(0, |prev_highest, (n, tree)| {
            Some((
                n,
                if *prev_highest < tree {
                    *prev_highest = tree;
                    true
                } else {
                    false
                },
            ))
        })
        .filter_map(|(n, t)| if t { Some(n) } else { None })
}

#[aoc(day8, part1)]
fn solve_part1(input: &Array2<u8>) -> usize {
    let mut vis = Array2::from_elem(input.dim(), false);

    // Outside trees are always visible
    for edge in [
        s![.., 0],  // Top
        s![.., -1], // Bottom
        s![0, ..],  // Left
        s![-1, ..], // Right
    ] {
        vis.slice_mut(edge).fill(true);
    }

    for (x, col) in input.axis_iter(Axis(0)).enumerate() {
        let col = col.iter().copied().enumerate();
        visible(col.clone()).for_each(|y| vis[(x, y)] = true);
        visible(col.clone().rev()).for_each(|y| vis[(x, y)] = true);
    }

    for (y, row) in input.axis_iter(Axis(1)).enumerate() {
        let row = row.iter().copied().enumerate();
        visible(row.clone()).for_each(|x| vis[(x, y)] = true);
        visible(row.clone().rev()).for_each(|x| vis[(x, y)] = true);
    }

    vis.into_iter().filter(|v| *v).count()
}

fn sight_len(slice: ArrayView1<u8>, tree: u8) -> usize {
    let visible = slice.into_iter().take_while(|&&t| t < tree).count();

    if visible == slice.len() {
        // We're looking out of the grove
        visible
    } else {
        // Add on the tree that blocks us
        visible + 1
    }
}

fn scenic_score(grove: ArrayView2<u8>, tree @ (x, y): Point) -> usize {
    let height = grove[tree];
    let rays = [
        s![(x + 1).., y], // Right
        s![..x;-1,    y], // Left
        s![x, (y + 1)..], // Up
        s![x, ..y;-1   ], // Down
    ];

    rays.into_iter()
        .map(|ray| sight_len(grove.slice(ray), height))
        .product()
}

#[aoc(day8, part2)]
fn solve_part2(input: &Array2<u8>) -> usize {
    input
        .indexed_iter()
        .map(|(tree, _)| scenic_score(input.view(), tree))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_INPUT: &str = "30373
25512
65332
33549
35390";

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), 21);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part1(&generate(&crate::get_input(8))), 1546);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example_scenic() {
            let grove = generate(SAMPLE_INPUT);
            let example_tree = (2, 3);
            assert_eq!(scenic_score(grove.view(), example_tree), 8);
        }

        #[test]
        fn example() {
            assert_eq!(solve_part2(&generate(SAMPLE_INPUT)), 8);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part2(&generate(&crate::get_input(8))), 519064);
        }
    }
}
