#[aoc_generator(day09)]
fn generate(input: &[u8]) -> Disk {
    let mut files = Vec::with_capacity((input.len() / 2) + 1);
    let mut spaces = Vec::with_capacity((input.len() / 2) + 1);

    for slice in input.chunks(2) {
        match *slice {
            [size, space] => {
                files.push(size as usize - 48);
                spaces.push(space as usize - 48);
            }
            [size] => files.push(size as usize - 48),
            _ => unreachable!(),
        }
    }

    assert_eq!(files.len(), spaces.len() + 1, "file-space ratio incorrect");

    Disk { files, spaces }
}

#[derive(Debug, Clone)]
struct Disk {
    files: Vec<usize>,
    spaces: Vec<usize>,
}

#[derive(Copy, Clone, Debug)]
enum Item {
    File { id: usize },
    Space { id: usize },
}

impl std::ops::Index<Item> for Disk {
    type Output = usize;

    fn index(&self, index: Item) -> &Self::Output {
        match index {
            Item::File { id } => self.files.get(id).unwrap(),
            Item::Space { id } => self.spaces.get(id).unwrap(),
        }
    }
}

impl Disk {
    fn len(&self) -> usize {
        self.files.iter().chain(self.spaces.iter()).copied().sum()
    }
}

#[derive(Debug, Clone)]
struct Compress<'a> {
    disk: &'a Disk,
    // front cursor and cached items
    front_cursor: usize,
    front_item: Item,
    front_item_start: usize,
    // back cursor and cached items
    back_cursor: usize,
    back_file_id: usize,
    back_file_start: usize,
}

impl<'a> Compress<'a> {
    fn disk(disk: &'a Disk) -> Self {
        let back_file_id = disk.files.len() - 1;
        let disk_len = disk.len();
        Self {
            front_item: Item::File { id: 0 },
            front_item_start: 0,
            front_cursor: 0,
            back_cursor: disk_len - 1,
            back_file_id,
            back_file_start: disk_len - disk.files[back_file_id],
            disk,
        }
    }

    fn bump_front(&mut self) {
        self.front_cursor += 1;
        let next_item_start = self.front_item_start + self.disk[self.front_item];
        if self.front_cursor >= next_item_start {
            self.front_item_start = next_item_start;
            self.front_item = match self.front_item {
                Item::File { id } => Item::Space { id },
                Item::Space { id } => Item::File { id: id + 1 },
            }
        }
    }

    fn bump_back(&mut self) {
        self.back_cursor -= 1;
        if self.back_cursor < self.back_file_start {
            self.back_file_id -= 1;
            let skipped_space = self.disk.spaces[self.back_file_id];
            self.back_cursor -= skipped_space;
            self.back_file_start -= self.disk.files[self.back_file_id] + skipped_space;
        }
    }
}

impl Iterator for Compress<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // if we're past the end of the disk, end iteration
        if self.front_cursor > self.back_cursor {
            return None;
        }

        let id = if let Item::File { id } = self.front_item {
            // we're still in a file
            id
        } else {
            // we're in a space, get the last file in the disk
            let id = self.back_file_id;
            self.bump_back();
            id
        };

        self.bump_front();
        Some(id)
    }
}

#[aoc(day09, part1)]
fn solve_part1(input: &Disk) -> usize {
    Compress::disk(input)
        .enumerate()
        .map(|(pos, id)| pos * id)
        .sum()
}

#[aoc(day09, part2)]
fn solve_part2(_input: &Disk) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;
    use test_case::test_case;

    const SAMPLE_INPUT: &[u8] = b"2333133121414131402";

    #[test_case(SAMPLE_INPUT, Item::File{id: 0} => 2)]
    #[test_case(SAMPLE_INPUT, Item::Space{id: 0} => 3)]
    #[test_case(SAMPLE_INPUT, Item::File{id: 6} => 4)]
    #[test_case(SAMPLE_INPUT, Item::File{id: 2} => 1)]
    fn index_disk(disk_raw: &[u8], index: Item) -> usize {
        generate(disk_raw)[index]
    }

    #[test]
    fn disk_len() {
        assert_eq!(generate(SAMPLE_INPUT).len(), 42);
    }

    mod part1 {
        use super::*;
        use test_case::test_case;

        #[test_case(SAMPLE_INPUT, 6 => vec![0, 0, 9, 9, 8, 1])]
        fn compress_disk(disk_raw: &[u8], len: usize) -> Vec<usize> {
            Compress::disk(&generate(disk_raw)).take(len).collect()
        }

        #[test]
        fn compress_example() {
            let expected: Vec<_> = b"0099811188827773336446555566"
                .iter()
                .map(|&id| id as usize - 48)
                .collect();
            let actual: Vec<_> = Compress::disk(&generate(SAMPLE_INPUT)).collect();
            assert_eq!(actual, expected);
        }

        #[test_case(SAMPLE_INPUT)]
        #[test_case(crate::get_input(09).as_bytes(); "mine")]
        #[test_case(b"12345")]
        fn compressed_length(disk_raw: &[u8]) {
            let disk = generate(disk_raw);
            let disk_files_len: usize = disk.files.iter().map(|&len| len as usize).sum();
            let compressed_len = Compress::disk(&disk).count();
            assert_eq!(disk_files_len, compressed_len);
        }

        #[test]
        fn example() {
            assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), 1928);
        }

        #[test]
        fn mine() {
            assert_eq!(
                solve_part1(&generate(&crate::get_input(09).as_bytes())),
                6399153661894
            );
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
            assert_eq!(
                solve_part2(&generate(&crate::get_input(09).as_bytes())),
                todo!()
            );
        }
    }
}
