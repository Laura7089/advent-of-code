use crate::field2d::{compressed_field::CompressedField, Field2D};

#[aoc_generator(day20)]
pub fn parse_input(input: &str) -> (String, CompressedField<bool>) {
    let mut split = input.split("\n\n");
    let algo = split.next().unwrap().to_string();

    let image_raw = split.next().unwrap();
    let image_axis = image_raw.lines().count();
    let final_size = (image_axis + 2).pow(2);
    let mut image = CompressedField::new(vec![false; final_size], image_axis);

    for (y, line) in image_raw.lines().enumerate() {
        for (x, byte) in line.bytes().enumerate() {
            if byte == b'#' {
                image[(x + 2, y + 2)] = true;
            }
        }
    }

    (algo, image)
}

#[aoc(day20, part1)]
pub fn solve_part1((algo, image): &(String, CompressedField<bool>)) -> usize {
    let image = image.clone();

    for search_ext in 1..=2 {
        for y in search_ext..(image.height() + search_ext) {
            for x in search_ext..(image.width() + search_ext) {
                let mut algo_index = 0;
                let adjacents = image.adjacents((x, y));
            }
        }
    }

    image.iter().filter(|p| p == &&true).count()
}

#[aoc(day20, part2)]
pub fn solve_part2((algo, image): &(String, CompressedField<bool>)) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str =
        "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&parse_input(&EXAMPLE_INPUT)), 35);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&parse_input(&EXAMPLE_INPUT)), unimplemented!());
    }

    #[test]
    fn part1_myinput() {
        let _input = crate::get_input_for_day(20);
        assert_eq!(solve_part1(&parse_input(&_input)), unimplemented!());
    }

    #[test]
    fn part2_myinput() {
        let _input = crate::get_input_for_day(20);
        assert_eq!(solve_part2(&parse_input(&_input)), unimplemented!());
    }
}
