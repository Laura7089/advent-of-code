#[cfg(feature = "rayon")]
use rayon::prelude::*;

// TODO: wish this could be a generator :(
fn generate_rect(y: usize, y_max: usize, x1: usize, len: usize) -> Vec<(usize, usize)> {
    let right_lim = x1 + len + 1;
    let mut coords = Vec::with_capacity(x1 * 3);

    // Bottom row
    if y != 0 {
        for x in x1.saturating_sub(1)..right_lim {
            coords.push((x, y - 1));
        }
    }
    // Middle row
    coords.push((x1.saturating_sub(1), y));
    coords.push((right_lim - 1, y));
    // Top row
    if y != y_max {
        for x in x1.saturating_sub(1)..right_lim {
            coords.push((x, y + 1));
        }
    }

    coords
}

/// Sparse matrix of part markers.
struct PartMap(Vec<Vec<usize>>);

impl PartMap {
    /// Parse the matrix from a string input.
    ///
    /// Treats any characters for which `pred` returns `true` as part markers.
    fn parse_from(input: &str, pred: impl Fn(char) -> bool) -> Self {
        let inner = input
            .lines()
            .map(|line| {
                line.chars()
                    .enumerate()
                    .filter_map(|(x, c)| pred(c).then_some(x))
                    .collect()
            })
            .collect();
        Self(inner)
    }

    /// Find all part markers adjacent to a rectangle.
    ///
    /// If the 1-height, `len`-width rectangle starting at `(x1, y)` is adjacent to
    /// a part marker, it is yielded here.
    fn search_rect<'a>(
        &'a self,
        y: usize,
        x1: usize,
        len: usize,
    ) -> impl Iterator<Item = (usize, usize)> + 'a {
        generate_rect(y, self.0.len() - 1, x1, len)
            .into_iter()
            .filter_map(|(x, y)| self.0[y].contains(&x).then_some((x, y)))
    }

    /// Find if a rectangle has any adjacent part markers.
    ///
    /// See [`Self::search_rect`].
    fn search_rect_any(&self, y: usize, x1: usize, len: usize) -> bool {
        self.search_rect(y, x1, len).next().is_some()
    }
}

#[inline(always)]
fn part1_pred(c: char) -> bool {
    c != '.' && !c.is_digit(10)
}

/// Find the next digit sequence in `line`.
///
/// If one is found, returns `Some((slice, start_idx, len))` where `slice` is the digit sequence.
/// Skips over `skipn` chars at the beginning of the line.
fn find_digit_seq<'a>(line: &'a str, pointer: &mut usize) -> Option<(&'a str, usize, usize)> {
    let mut num_seq = line
        .chars()
        .enumerate()
        .skip(*pointer)
        .skip_while(|v| !v.1.is_digit(10));

    let (start, _) = num_seq.next()?;
    let len = num_seq.take_while(|v| v.1.is_digit(10)).count() + 1;
    *pointer = start + len + 1;
    Some((&line[start..(start + len)], start, len))
}

#[aoc(day03, part1)]
fn solve_part1(input: &str) -> usize {
    let symbols_locs = PartMap::parse_from(input, part1_pred);

    #[cfg(feature = "rayon")]
    let lines = input.lines().collect::<Vec<_>>().into_par_iter();
    #[cfg(not(feature = "rayon"))]
    let lines = input.lines();

    lines
        .enumerate()
        .map(|(y, line)| {
            let mut ptr = 0;
            let mut total = 0;
            while let Some((seq, start, len)) = find_digit_seq(line, &mut ptr) {
                if symbols_locs.search_rect_any(y, start, len) {
                    total += seq.parse::<usize>().unwrap();
                }
            }
            total
        })
        .sum()
}

#[aoc(day03, part2)]
fn solve_part2(input: &str) -> usize {
    let pot_gears = PartMap::parse_from(input, |c| c == '*');
    let mut gear_map: Vec<Vec<(usize, usize)>> = pot_gears
        .0
        .iter()
        .map(|xs| xs.iter().map(|&x| (x, 0)).collect())
        .collect();

    todo!()
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;

    const SAMPLE_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_get_symbols_locs() {
        let locs = PartMap::parse_from(SAMPLE_INPUT, part1_pred);

        assert_eq!(
            locs.0,
            vec![
                vec![],
                vec![3],
                vec![],
                vec![6],
                vec![3],
                vec![5],
                vec![],
                vec![],
                vec![3, 5],
                vec![],
            ]
        );
    }

    #[test]
    fn test_contains_adj_rect() {
        let locs = PartMap::parse_from(SAMPLE_INPUT, part1_pred);

        assert!(locs.search_rect_any(0, 0, 3));
        assert!(locs.search_rect_any(2, 6, 3));
        assert!(locs.search_rect_any(5, 0, 3));
        assert!(locs.search_rect_any(6, 2, 3));
        assert!(locs.search_rect_any(7, 6, 3));
        assert!(!locs.search_rect_any(5, 7, 2));

        // Found earlier
        let test_case = "....\n.12*\n....";
        assert!(PartMap::parse_from(test_case, part1_pred).search_rect_any(1, 1, 2));
    }

    #[test]
    fn test_contains_adj_rect_diags() {
        let test_case = b"....\n.12.\n....";
        for x in [0, 3] {
            for y in [0, 2] {
                let mut current_case = test_case.to_vec();
                current_case[(y * 5) + x] = b'*';
                let current_case = String::from_utf8(current_case).unwrap();
                assert!(PartMap::parse_from(&current_case, part1_pred).search_rect_any(1, 1, 2));
            }
        }
    }

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part1(SAMPLE_INPUT), 4361);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part1(&crate::get_input(03)), 537832);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part2(SAMPLE_INPUT), 467835);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part2(&crate::get_input(03)), todo!());
        }
    }
}
