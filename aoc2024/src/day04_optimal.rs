use bitvec::prelude::*;

type Wordsearch = BitVec;

#[aoc_generator(day04, part1, Optimal)]
fn parse(input: &[u8]) -> Wordsearch {
    let mut search = Wordsearch::new();

    let input = input.iter().filter(|b| b != &&b'\n');
    for character in input {
        match character {
            b'X' => search.extend_from_bitslice(bits![0, 0]),
            b'M' => search.extend_from_bitslice(bits![0, 1]),
            b'A' => search.extend_from_bitslice(bits![1, 0]),
            b'S' => search.extend_from_bitslice(bits![1, 1]),
            other => panic!("unexpected character '{other}'"),
        }
    }

    search
}
