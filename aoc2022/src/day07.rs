#[derive(Clone, Debug, PartialEq)]
enum Filesystem {
    File {
        name: String,
        size: u32,
    },
    Dir {
        name: String,
        contents: Vec<Filesystem>,
    },
}

impl Filesystem {
    fn get_contents_mut(&mut self) -> &mut Vec<Self> {
        if let Self::Dir { contents, .. } = self {
            return contents;
        } else {
            panic!("Filesystem::get_contents called on a non-dir object");
        }
    }

    fn get_contents(&self) -> &[Self] {
        if let Self::Dir { contents, .. } = self {
            return contents;
        } else {
            panic!("Filesystem::get_contents called on a non-dir object");
        }
    }
}

mod parse {
    use super::Filesystem;
    use nom::{
        branch::alt,
        bytes::complete::{is_a, tag},
        character::complete::{alpha1, char, line_ending},
        combinator::{map, value},
        multi::separated_list1 as seplist,
        sequence::{preceded, separated_pair as seppair},
        IResult,
    };

    const FILE_ALLOWED_CHARS: &str = "abcdefghijklmnopqrstuvwxyz.";
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
        Item(Filesystem),
        Cmd(Command<'a>),
    }

    fn file(input: &str) -> IResult<&str, Filesystem> {
        use nom::character::complete::u32;
        let (input, (size, n)) = seppair(u32, char(' '), is_a(FILE_ALLOWED_CHARS))(input)?;
        Ok((
            input,
            Filesystem::File {
                name: n.to_owned(),
                size,
            },
        ))
    }

    fn dir(input: &str) -> IResult<&str, Filesystem> {
        let (input, n) = preceded(tag("dir "), alpha1)(input)?;
        Ok((
            input,
            Filesystem::Dir {
                name: n.to_owned(),
                contents: vec![],
            },
        ))
    }

    fn cmd(input: &str) -> IResult<&str, Command> {
        let ls = value(Command::ls, tag("ls"));
        let cd = preceded(
            tag("cd "),
            alt((value(Command::cdup, tag("..")), map(alpha1, Command::cd))),
        );
        preceded(tag("$ "), alt((ls, cd)))(input)
    }

    fn line(input: &str) -> IResult<&str, Line> {
        let item = map(alt((file, dir)), Line::Item);
        let command = map(cmd, Line::Cmd);
        alt((item, command))(input)
    }

    fn populate_dir<'a>(parent: &mut Filesystem, lines: &mut impl Iterator<Item = Line<'a>>) {
        // We're using a while loop here so that we can pull out of the iterator within too
        while let Some(line) = lines.next() {
            match line {
                Line::Cmd(Command::cd(dest)) => {
                    // Find the dir to `cd` to
                    let target = parent
                        .get_contents_mut()
                        .iter_mut()
                        .find(|c| matches!(c, Filesystem::Dir { name, .. } if name == dest))
                        .expect("Couldn't find folder {dest} to `cd` to");
                    // Recurse
                    populate_dir(target, lines);
                }
                Line::Item(d) => parent.get_contents_mut().push(d),
                Line::Cmd(Command::cdup) => return,
                Line::Cmd(Command::ls) => (),
            }
        }
    }

    pub(super) fn parse_all(input: &str) -> IResult<&str, Filesystem> {
        let mut root = Filesystem::Dir {
            name: "/".to_owned(),
            contents: Vec::with_capacity(VEC_PREALLOCATE),
        };
        let (input, lines) = preceded(tag("$ cd /\n"), seplist(line_ending, line))(input)?;
        populate_dir(&mut root, &mut lines.into_iter());
        Ok((input, root))
    }
}

#[aoc_generator(day7)]
fn generate(input: &str) -> Filesystem {
    parse::parse_all(input).unwrap().1
}

fn dir_size(current: &mut Vec<u32>, dir: &Filesystem) -> u32 {
    let total = dir
        .get_contents()
        .iter()
        .map(|entry| match entry {
            Filesystem::File { size, .. } => *size,
            Filesystem::Dir { .. } => dir_size(current, entry),
        })
        .sum();

    current.push(total);
    total
}

#[aoc(day7, part1)]
fn solve_part1(input: &Filesystem) -> u32 {
    let mut sizes = Vec::with_capacity(500);
    dir_size(&mut sizes, input);
    sizes.into_iter().filter(|x| x <= &100_000).sum()
}

const TOTAL_SPACE: u32 = 70_000_000;
const NEEDED_FREE: u32 = 30_000_000;

#[aoc(day7, part2)]
fn solve_part2(input: &Filesystem) -> u32 {
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
