use ndarray::{Array2, Axis};

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

fn visible(input: impl Iterator<Item = (usize, u8)>) -> impl Iterator<Item = usize> {
    input
        .scan(0, |max_height, (n, tree)| {
            Some((
                n,
                if *max_height >= tree {
                    false
                } else if tree == 0 {
                    true
                } else {
                    let clipped_tree = tree - *max_height;
                    *max_height += clipped_tree;
                    clipped_tree != 0
                },
            ))
        })
        .filter_map(|(n, t)| if t { Some(n) } else { None })
}

#[aoc(day8, part1)]
fn solve_part1(input: &Array2<u8>) -> usize {
    let mut vis = Array2::from_elem(input.dim(), false);
    vis.column_mut(0).iter_mut().for_each(|v| *v = true);
    vis.column_mut(vis.dim().0 - 1)
        .iter_mut()
        .for_each(|v| *v = true);
    vis.row_mut(0).iter_mut().for_each(|v| *v = true);
    vis.row_mut(vis.dim().1 - 1)
        .iter_mut()
        .for_each(|v| *v = true);

    for (x, col) in input.axis_iter(Axis(0)).enumerate() {
        visible(col.iter().copied().enumerate()).for_each(|y| vis[(x, y)] = true);
        visible(col.iter().copied().enumerate().rev()).for_each(|y| vis[(x, y)] = true);
    }

    for (y, row) in input.axis_iter(Axis(1)).enumerate() {
        visible(row.iter().copied().enumerate()).for_each(|x| vis[(x, y)] = true);
        visible(row.iter().copied().enumerate().rev()).for_each(|x| vis[(x, y)] = true);
    }

    vis.into_iter().filter(|v| *v).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), 21);
    }

    #[test]
    fn part1_mine() {
        assert_eq!(solve_part1(&generate(&crate::get_input(8))), 1546);
    }
}
