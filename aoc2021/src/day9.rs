use std::cmp::Ordering;

#[derive(Debug, Clone)]
struct CompressedField<T> {
    pub map: Vec<T>,
    pub row_len: usize,
}

impl<T: Copy> CompressedField<T> {
    fn adjacents(&self, x: usize, y: usize) -> [Option<T>; 4] {
        // Members of this array are the adjacent squares, clockwise from above
        let mut adjacents = [None; 4];
        assert!(x < self.row_len && y < self.height());

        match (x.cmp(&0), x.cmp(&(self.row_len - 1))) {
            (Ordering::Equal, _) => adjacents[1] = Some(self[(x + 1, y)]),
            (_, Ordering::Equal) => adjacents[3] = Some(self[(x - 1, y)]),
            _ => {
                adjacents[1] = Some(self[(x + 1, y)]);
                adjacents[3] = Some(self[(x - 1, y)])
            }
        }

        match (y.cmp(&0), y.cmp(&(self.height() - 1))) {
            (Ordering::Equal, _) => adjacents[0] = Some(self[(x, y + 1)]),
            (_, Ordering::Equal) => adjacents[2] = Some(self[(x, y - 1)]),
            _ => {
                adjacents[0] = Some(self[(x, y + 1)]);
                adjacents[2] = Some(self[(x, y - 1)])
            }
        }

        adjacents
    }

    fn height(&self) -> usize {
        self.map.len() / self.row_len
    }
}

impl<T> std::ops::Index<(usize, usize)> for CompressedField<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
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
            .map(str::bytes)
            .flatten()
            .map(|p| p as usize - 48)
            .collect(),
    }
}

#[aoc(day9, part1)]
fn solve_part1(input: &CompressedField<usize>) -> usize {
    let mut total_risk = 0;

    for y in 0..input.height() {
        for x in 0..input.row_len {
            let current = input[(x, y)];
            let is_low_point = input
                .adjacents(x, y)
                .iter()
                .filter_map(|p| *p)
                .fold(true, |prev, point| prev && point > current);

            if is_low_point {
                total_risk += current + 1;
            }
        }
    }

    total_risk
}

#[aoc(day9, part2)]
fn solve_part2(_input: &CompressedField<usize>) -> usize {
    unimplemented!()
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
        assert_eq!(solve_part2(&parse_input(&EXAMPLE_INPUT)), unimplemented!());
    }

    #[test]
    fn part1_myinput() {
        let _input = crate::get_input_for_day(9);
        assert_eq!(solve_part1(&parse_input(&_input)), unimplemented!());
    }

    #[test]
    fn part2_myinput() {
        let _input = crate::get_input_for_day(9);
        assert_eq!(solve_part2(&parse_input(&_input)), unimplemented!());
    }
}
