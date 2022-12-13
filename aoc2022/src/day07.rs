#[derive(Clone, Debug, PartialEq)]
struct Dir {
    name: String,
    subdirs: Vec<Self>,
    size: u32,
}

mod parse {
    use super::Dir;
    use nom::{
        branch::alt,
        bytes::complete::{is_a, tag},
        character::complete::{alpha1, line_ending as le},
        combinator::{map, value},
        multi::separated_list1 as seplist,
        sequence::{preceded, terminated},
        IResult,
    };

    const FILE_CHARS: &str = "abcdefghijklmnopqrstuvwxyz. ";
    const VEC_PREALLOCATE: usize = 10;

    #[allow(non_camel_case_types)]
    #[derive(Clone, PartialEq, Debug)]
    enum Command<'a> {
        ls,
        cd(&'a str),
        cdup,
    }

    #[derive(Clone, PartialEq, Debug)]
    enum Line<'a> {
        Dir(Dir),
        File(u32),
        Cmd(Command<'a>),
    }

    fn file(input: &str) -> IResult<&str, Line> {
        use nom::character::complete::u32;
        map(terminated(u32, is_a(FILE_CHARS)), Line::File)(input)
    }

    fn dir(input: &str) -> IResult<&str, Line> {
        let (input, n) = preceded(tag("dir "), alpha1)(input)?;
        Ok((
            input,
            Line::Dir(Dir {
                name: n.to_owned(),
                subdirs: vec![],
                size: 0,
            }),
        ))
    }

    fn cmd(input: &str) -> IResult<&str, Line> {
        let ls = value(Command::ls, tag("ls"));
        let cd = preceded(
            tag("cd "),
            alt((value(Command::cdup, tag("..")), map(alpha1, Command::cd))),
        );
        map(preceded(tag("$ "), alt((ls, cd))), Line::Cmd)(input)
    }

    fn line(input: &str) -> IResult<&str, Line> {
        alt((file, dir, cmd))(input)
    }

    fn populate_dir<'a>(parent: &mut Dir, lines: &mut impl Iterator<Item = Line<'a>>) {
        // We're using a while loop here so that we can pull out of the
        // iterator inside (a for loop wouldn't let us)
        while let Some(line) = lines.next() {
            match line {
                Line::Cmd(Command::cd(dest)) => {
                    populate_dir(
                        parent
                            .subdirs
                            .iter_mut()
                            .find(|d| d.name == dest)
                            .expect("Couldn't find folder {dest} to `cd` to"),
                        lines,
                    );
                }
                Line::Dir(d) => parent.subdirs.push(d),
                Line::File(s) => parent.size += s,
                Line::Cmd(Command::cdup) => return,
                Line::Cmd(Command::ls) => (),
            }
        }
    }

    pub(super) fn parse_all(input: &str) -> IResult<&str, Dir> {
        let mut root = Dir {
            name: "/".to_owned(),
            subdirs: Vec::with_capacity(VEC_PREALLOCATE),
            size: 0,
        };
        let first = tag("$ cd /\n");
        let (input, lines) = preceded(first, seplist(le, line))(input)?;
        populate_dir(&mut root, &mut lines.into_iter());
        Ok((input, root))
    }
}

#[aoc_generator(day7)]
fn generate(input: &str) -> Dir {
    parse::parse_all(input).unwrap().1
}

fn dir_size(current: &mut Vec<u32>, dir: &Dir) -> u32 {
    let total = dir
        .subdirs
        .iter()
        .map(|entry| dir_size(current, entry))
        .sum::<u32>()
        + dir.size;

    current.push(total);
    total
}

#[aoc(day7, part1)]
fn solve_part1(input: &Dir) -> u32 {
    let mut sizes = Vec::with_capacity(500);
    dir_size(&mut sizes, input);
    sizes.into_iter().filter(|x| x <= &100_000).sum()
}

const TOTAL_SPACE: u32 = 70_000_000;
const NEEDED_FREE: u32 = 30_000_000;

#[aoc(day7, part2)]
fn solve_part2(input: &Dir) -> u32 {
    let mut sizes = Vec::with_capacity(500);
    dir_size(&mut sizes, input);

    let need_to_delete = NEEDED_FREE - (TOTAL_SPACE - *sizes.last().unwrap());
    sizes
        .into_iter()
        .filter(|s| s >= &need_to_delete)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), 95437);
    }

    #[test]
    fn part1_mine() {
        assert_eq!(solve_part1(&generate(&crate::get_input(7))), 1444896);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&generate(SAMPLE_INPUT)), 24933642);
    }

    #[test]
    fn part2_mine() {
        assert_eq!(solve_part2(&generate(&crate::get_input(7))), 404395);
    }
}
