use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

type Label = [u8; 3];
const START: &[u8] = b"AAA";
const GOAL: &[u8] = b"ZZZ";

type NodeTree = Vec<([u8; 3], usize, usize)>;

fn label_pos(labels: &mut HashMap<[u8; 3], usize>, label_raw: &[u8], cursor: &mut usize) -> usize {
    let label = Label::try_from(label_raw).unwrap();
    *labels.entry(label).or_insert_with(|| {
        let prev = *cursor;
        *cursor += 1;
        prev
    })
}

#[aoc_generator(day08)]
fn generate(input: &str) -> (Vec<Direction>, NodeTree, usize, usize) {
    let (dirs_raw, nodes) = input.split_once("\n\n").expect("Bad input format");

    let dirs = dirs_raw
        .bytes()
        .map(|b| match b {
            b'R' => Direction::Right,
            b'L' => Direction::Left,
            _ => panic!("Bad input"),
        })
        .collect();

    let mut labels = HashMap::new();
    let mut tree = vec![None; nodes.lines().count()];
    let mut cursor = 0;

    for node_raw in nodes.lines() {
        let node_raw = node_raw.as_bytes();

        let label = Label::try_from(&node_raw[0..3]).unwrap();
        let label_i = label_pos(&mut labels, &node_raw[0..3], &mut cursor);
        let llabel_i = label_pos(&mut labels, &node_raw[7..10], &mut cursor);
        let rlabel_i = label_pos(&mut labels, &node_raw[12..15], &mut cursor);

        tree[label_i] = Some((label, llabel_i, rlabel_i));
    }

    (
        dirs,
        tree.into_iter()
            .map(|n| n.expect("Uninitialised node found"))
            .collect(),
        labels[START],
        labels[GOAL],
    )
}

#[aoc(day08, part1)]
fn solve_part1((dirs, tree, start, goal): &(Vec<Direction>, NodeTree, usize, usize)) -> usize {
    let mut cursor = *start;
    for (n, di) in (0..dirs.len()).cycle().enumerate() {
        cursor = match dirs[di] {
            Direction::Left => tree[cursor].1,
            Direction::Right => tree[cursor].2,
        };
        if cursor == *goal {
            return n + 1;
        }
    }
    unreachable!()
}

fn find_last_letters(tree: &NodeTree, letter: u8) -> Vec<usize> {
    tree.iter()
        .enumerate()
        .filter_map(|(i, (label, _, _))| (label[2] == letter).then_some(i))
        .collect()
}

#[aoc(day08, part2)]
fn solve_part2((dirs, tree, _, _): &(Vec<Direction>, NodeTree, usize, usize)) -> usize {
    let mut cursors = find_last_letters(tree, b'A');
    let goals = find_last_letters(tree, b'Z');

    for (n, di) in (0..dirs.len()).cycle().enumerate() {
        println!("{cursors:?}");
        for cur in &mut cursors {
            *cur = match dirs[di] {
                Direction::Left => tree[*cur].1,
                Direction::Right => tree[*cur].2,
            };
        }
        if cursors.iter().all(|c| goals.contains(c)) {
            return n + 1;
        }
    }
    unreachable!()
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
            assert_eq!(solve_part2(&generate(SAMPLE_INPUT)), 6);
        }

        #[test]
        #[ignore]
        fn mine() {
            assert_eq!(solve_part2(&generate(&crate::get_input(08))), todo!());
        }
    }
}
