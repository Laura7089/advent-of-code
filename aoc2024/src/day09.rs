type File = (u8, Option<u8>);

#[aoc_generator(day09)]
fn generate(input: &str) -> Vec<File> {
    input
        .as_bytes()
        .chunks(2)
        .map(|pair| match pair {
            &[size, space] => (size - 48, Some(space - 48)),
            &[size] => (size - 48, None),
            _ => unreachable!(),
        })
        .collect()
}

#[aoc(day09, part1)]
fn solve_part1(input: &[File]) -> usize {
    let mut final_disk = Vec::new();

    for (id, &(file_size, space_size)) in input.iter().enumerate() {
        final_disk.resize(final_disk.len() + file_size as usize, Some(id));
        if let Some(size) = space_size {
            final_disk.resize(final_disk.len() + size as usize, None);
        }
    }

    for i in (0..final_disk.len()).rev() {
        let Some(id) = final_disk[i].take() else {
            continue;
        };

        let _ = final_disk
            .iter_mut()
            .find(|elem| elem.is_none())
            .expect("no space in final disk")
            .insert(id);
    }

    final_disk
        .into_iter()
        .flat_map(|space| space)
        .enumerate()
        .map(|(i, e)| i * e)
        .sum()
}

#[aoc(day09, part2)]
fn solve_part2(_input: &[File]) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;

    const SAMPLE_INPUT: &str = "2333133121414131402";

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), 1928);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part1(&generate(&crate::get_input(09))), 6399153661894);
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
            assert_eq!(solve_part2(&generate(&crate::get_input(09))), todo!());
        }
    }
}
