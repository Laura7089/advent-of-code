use std::cmp::Ordering;
use std::collections::HashSet;
use std::ops::{Index, Mul};

const NUM_LARGEST: usize = 3;

type Idx = (usize, usize);

#[derive(Debug, Clone)]
struct CompressedField<T> {
    pub map: Vec<T>,
    pub row_len: usize,
}

impl<T: Copy> CompressedField<T> {
    fn adjacents(&self, x: usize, y: usize) -> [Option<Idx>; 4] {
        // Members of this are adjacent squares, clockwise from top
        let mut adjacents = [None; 4];
        assert!(x < self.row_len && y < self.height());

        match (x.cmp(&0), x.cmp(&(self.row_len - 1))) {
            (Ordering::Equal, _) => adjacents[1] = Some((x + 1, y)),
            (_, Ordering::Equal) => adjacents[3] = Some((x - 1, y)),
            _ => {
                adjacents[1] = Some((x + 1, y));
                adjacents[3] = Some((x - 1, y))
            }
        }

        match (y.cmp(&0), y.cmp(&(self.height() - 1))) {
            (Ordering::Equal, _) => adjacents[0] = Some((x, y + 1)),
            (_, Ordering::Equal) => adjacents[2] = Some((x, y - 1)),
            _ => {
                adjacents[0] = Some((x, y + 1));
                adjacents[2] = Some((x, y - 1))
            }
        }

        adjacents
    }
}

impl<T> CompressedField<T> {
    fn height(&self) -> usize {
        self.map.len() / self.row_len
    }
}

impl<T: PartialOrd + Copy> CompressedField<T> {
    fn low_points(&self) -> Vec<Idx> {
        let mut low_points = Vec::with_capacity(self.height());

        for y in 0..self.height() {
            for x in 0..self.row_len {
                let current = self[(x, y)];
                let is_low_point = self
                    .adjacents(x, y)
                    .iter()
                    .filter_map(|p| *p)
                    .all(|point| self[point] > current);

                if is_low_point {
                    low_points.push((x, y));
                }
            }
        }

        low_points
    }
}

impl<T> Index<Idx> for CompressedField<T> {
    type Output = T;

    fn index(&self, (x, y): Idx) -> &Self::Output {
        assert!(x < self.row_len);
        assert!(y < self.height());
        &self.map[(y * self.row_len) + x]
    }
}

#[aoc_generator(day9)]
fn parse_input(input: &str) -> CompressedField<usize> {
    let row_len = input.lines().next().unwrap().len();

    CompressedField {
        row_len,
        map: input
            .lines()
            .flat_map(str::bytes)
            .map(|p| p as usize - 48)
            .collect(),
    }
}

#[aoc(day9, part1)]
fn solve_part1(input: &CompressedField<usize>) -> usize {
    let low_points = input.low_points();
    low_points.iter().map(|i| input[*i]).sum::<usize>() + low_points.len()
}

#[aoc(day9, part2)]
fn solve_part2(input: &CompressedField<usize>) -> usize {
    let mut largest_sizes = [0_usize; NUM_LARGEST];

    for (lx, ly) in input.low_points().into_iter() {
        // Note: Use the field width as a capacity so it scales correctly
        // Stores the final basin
        let mut basin = HashSet::with_capacity(input.row_len);
        // Stores the uninspected edges
        let mut basin_edge = Vec::with_capacity(input.row_len);
        basin.insert((lx, ly));
        basin_edge.push((lx, ly));

        // Look through the outer edge of the basin
        while let Some((x, y)) = basin_edge.pop() {
            let current_val = input[(x, y)];

            // Iterate through the squares adjacent to it
            for adj in input.adjacents(x, y).into_iter().filter_map(|p| p) {
                let adj_val = input[adj];
                // If:
                // - it's not 9
                // - it's higher than the current square
                // - we haven't already recorded it
                if adj_val != 9 && current_val < adj_val && !basin.contains(&adj) {
                    basin.insert(adj);
                    basin_edge.push(adj);
                }
            }
        }

        // Add it to the array of largest sizes if it's larger than any of them
        largest_sizes.sort();
        for n in largest_sizes.iter_mut() {
            if &basin.len() > n {
                *n = basin.len();
                break;
            } else {
            }
        }
    }

    largest_sizes.into_iter().reduce(usize::mul).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&parse_input(&EXAMPLE_INPUT)), 15);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&parse_input(&EXAMPLE_INPUT)), 1134);
    }

    #[test]
    fn part1_myinput() {
        let input = crate::get_input_for_day(9);
        assert_eq!(solve_part1(&parse_input(&input)), 475);
    }

    #[test]
    fn part2_myinput() {
        let _input = crate::get_input_for_day(9);
        assert_eq!(solve_part2(&parse_input(&_input)), 1092012);
    }
}
