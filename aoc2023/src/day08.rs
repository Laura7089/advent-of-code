use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

const START: &[u8] = b"AAA";
const GOAL: &[u8] = b"ZZZ";

type NodeTree = Vec<(usize, usize)>;

fn label_pos<'a>(
    labels: &mut HashMap<&'a [u8], usize>,
    label: &'a [u8],
    cursor: &mut usize,
) -> usize {
    *labels.entry(label).or_insert_with(|| {
        let prev = *cursor;
        *cursor += 1;
        prev
    })
}

#[aoc_generator(day08)]
fn generate(input: &str) -> (Vec<Direction>, NodeTree, usize, usize) {
    let (dirs_raw, nodes_raw) = input.split_once("\n\n").expect("Bad input format");

    let dirs = dirs_raw
        .bytes()
        .map(|b| match b {
            b'R' => Direction::Right,
            b'L' => Direction::Left,
            _ => panic!("Bad input"),
        })
        .collect();

    let mut labels = HashMap::new();
    let mut tree = vec![None; nodes_raw.lines().count()];
    let mut cursor = 0;

    for node_raw in nodes_raw.lines() {
        let node_raw = node_raw.as_bytes();

        let label_i = label_pos(&mut labels, &node_raw[0..3], &mut cursor);
        let llabel_i = label_pos(&mut labels, &node_raw[7..10], &mut cursor);
        let rlabel_i = label_pos(&mut labels, &node_raw[12..15], &mut cursor);

        tree[label_i] = Some((llabel_i, rlabel_i));
    }

    (
        dirs,
        tree.into_iter()
            .map(|n| n.expect("Uninitialised node found"))
            .collect(),
        labels[&START],
        labels[&GOAL],
    )
}

#[aoc(day08, part1)]
fn solve_part1((dirs, tree, start, goal): &(Vec<Direction>, NodeTree, usize, usize)) -> usize {
    let mut cursor = *start;
    for (n, di) in (0..dirs.len()).cycle().enumerate() {
        cursor = match dirs[di] {
            Direction::Left => tree[cursor].0,
            Direction::Right => tree[cursor].1,
        };
        if cursor == *goal {
            return n + 1;
        }
    }
    unreachable!()
}

#[aoc(day08, part2)]
fn solve_part2((dirs, tree, start, goal): &(Vec<Direction>, NodeTree, usize, usize)) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;

    const SAMPLE_INPUT: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), 6);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part1(&generate(&crate::get_input(08))), 14893);
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
            assert_eq!(solve_part2(&generate(&crate::get_input(08))), todo!());
        }
    }
}
