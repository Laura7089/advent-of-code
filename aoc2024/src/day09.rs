#[aoc_generator(day09)]
fn generate(input: &[u8]) -> Disk {
    let mut files = Vec::with_capacity((input.len() / 2) + 1);
    let mut spaces = Vec::with_capacity((input.len() / 2) + 1);

    for slice in input.chunks(2) {
        match *slice {
            [size, space] => {
                files.push(size - 48);
                spaces.push(space - 48);
            }
            [size] => files.push(size - 48),
            _ => unreachable!(),
        }
    }

    assert_eq!(files.len(), spaces.len() + 1, "file-space ratio incorrect");

    Disk { files, spaces }
}

#[derive(Debug, Clone)]
struct Disk {
    files: Vec<u8>,
    spaces: Vec<u8>,
}

impl Disk {
    fn get(&self, index: usize) -> Option<usize> {
        let mut offset = 0;
        for id in 0..self.files.len() {
            offset += self.files[id] as usize;
            if index < offset {
                return Some(id);
            }
            if let Some(&space_size) = self.spaces.get(id) {
                offset += space_size as usize;
                if index < offset {
                    return None;
                }
            }
        }

        None
    }

    fn len(&self) -> usize {
        self.files
            .iter()
            .chain(self.spaces.iter())
            .map(|&len| len as usize)
            .sum()
    }

    fn total_space(&self) -> usize {
        self.spaces.iter().map(|&size| size as usize).sum()
    }

    fn start_compress(self) -> CompressingDisk {
        let disk_len = self.len();
        CompressingDisk {
            disk: self,
            cursor: 0,
            disk_len,
        }
    }
}

#[derive(Debug, Clone)]
struct CompressingDisk {
    disk: Disk,
    cursor: usize,
    disk_len: usize,
}

impl Iterator for CompressingDisk {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.disk_len <= self.cursor {
            return None;
        }

        if let id @ Some(_) = self.disk.get(self.cursor) {
            self.cursor += 1;
            return id;
        }

        let file_id = self.disk.files.len() - 1;
        let file_size = &mut self.disk.files[file_id];

        *file_size -= 1;
        self.disk_len -= 1;
        if *file_size == 0 {
            self.disk.files.pop();
            self.disk_len -= self.disk.spaces.pop().expect("couldn't pop space") as usize;
        }

        self.cursor += 1;
        Some(file_id)
    }
}

#[aoc(day09, part1)]
fn solve_part1(input: &Disk) -> usize {
    input
        .clone()
        .start_compress()
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

    #[test_case(SAMPLE_INPUT, 0 => Some(0))]
    #[test_case(SAMPLE_INPUT, 2 => None)]
    #[test_case(SAMPLE_INPUT, 4 => None)]
    #[test_case(SAMPLE_INPUT, 5 => Some(1))]
    #[test_case(SAMPLE_INPUT, 7 => Some(1))]
    fn get_on_disk(disk_raw: &[u8], index: usize) -> Option<usize> {
        generate(disk_raw).get(index)
    }

    fn disk_len() {
        assert_eq!(generate(SAMPLE_INPUT).len(), 41);
    }

    #[test_case(SAMPLE_INPUT, 6 => vec![0, 0, 9, 9, 8, 1])]
    fn compress_disk(disk_raw: &[u8], len: usize) -> Vec<usize> {
        generate(disk_raw).start_compress().take(len).collect()
    }

    #[test]
    fn compress_example() {
        let expected: Vec<_> = b"0099811188827773336446555566"
            .iter()
            .map(|&id| id as usize - 48)
            .collect();
        let actual: Vec<_> = generate(SAMPLE_INPUT).start_compress().collect();
        assert_eq!(actual, expected);
    }

    mod part1 {
        use super::*;

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
