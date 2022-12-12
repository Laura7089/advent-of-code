#[derive(Clone, Debug, PartialEq)]
enum Filesystem {
    File {
        name: String,
        size: u32,
    },
    Directory {
        name: String,
        contents: Vec<Filesystem>,
    },
}

mod parse {
    use super::Filesystem;
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alpha1, char, line_ending, one_of},
        multi::{many1, separated_list1},
        sequence::{preceded, tuple},
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
        let (input, (size, _, n)) =
            tuple((u32, char(' '), many1(one_of(FILE_ALLOWED_CHARS))))(input)?;

        Ok((
            input,
            Filesystem::File {
                name: n.iter().collect(),
                size,
            },
        ))
    }

    fn dir(input: &str) -> IResult<&str, Filesystem> {
        let (input, n) = preceded(tag("dir "), alpha1)(input)?;
        Ok((
            input,
            Filesystem::Directory {
                name: n.into(),
                contents: vec![],
            },
        ))
    }

    fn cmd(input: &str) -> IResult<&str, Command> {
        let (input, cmd) = preceded(tag("$ "), alpha1)(input)?;
        Ok(match cmd {
            "ls" => (input, Command::ls),
            "cd" => {
                let (i, dest) = preceded(char(' '), alpha1)(input)?;
                (
                    i,
                    if dest == ".." {
                        Command::cdup
                    } else {
                        Command::cd(dest)
                    },
                )
            }
            _ => panic!("Bad command {cmd} detected"),
        })
    }

    fn line(input: &str) -> IResult<&str, Line> {
        Ok(if let Ok((input, item)) = alt((file, dir))(input) {
            (input, Line::Item(item))
        } else {
            let (input, command) = cmd(input)?;
            (input, Line::Cmd(command))
        })
    }

    fn populate_dir<'a>(parent: &mut Filesystem, lines: &[Line<'a>]) -> usize {
        let mut i = 0;
        loop {
            if lines[i..].is_empty() {
                return i;
            }
            match &lines[i] {
                Line::Cmd(Command::cd(dest)) => {
                    if let Filesystem::Directory { contents, .. } = parent {
                        let target = contents
                            .iter_mut()
                            .find(
                                |c| matches!(c, Filesystem::Directory { name, .. } if name == dest),
                            )
                            .expect("Couldn't find folder to `cd` to");
                        i += populate_dir(target, &lines[(i + 1)..]);
                    } else {
                        unreachable!()
                    }
                }
                Line::Item(d) => {
                    if let Filesystem::Directory {
                        ref mut contents, ..
                    } = parent
                    {
                        contents.push(d.clone());
                    }
                }
                Line::Cmd(Command::cdup) => return i + 1,
                Line::Cmd(Command::ls) => (),
            }
            i += 1;
        }
    }

    pub(super) fn parse_all(input: &str) -> IResult<&str, Filesystem> {
        let mut root = Filesystem::Directory {
            name: "/".into(),
            contents: Vec::with_capacity(VEC_PREALLOCATE),
        };

        let (input, _) = tag("$ cd /\n")(input)?;
        let (input, lines) = separated_list1(line_ending, line)(input)?;

        populate_dir(&mut root, &lines);
        Ok((input, root))
    }
}

#[aoc_generator(day7)]
fn generate(input: &str) -> Filesystem {
    parse::parse_all(input).unwrap().1
}

fn dir_size(current: &mut Vec<u32>, dir: &Filesystem) -> u32 {
    let mut total = 0;
    if let Filesystem::Directory { contents, .. } = dir {
        for entry in contents.iter() {
            total += match entry {
                Filesystem::File { size, .. } => *size,
                Filesystem::Directory { .. } => {
                    let sub_size = dir_size(current, entry);
                    current.push(sub_size);
                    sub_size
                }
            }
        }
    } else {
        panic!("{} passed a non-directory arg", stringify!(dir_size));
    }

    total
}

#[aoc(day7, part1)]
fn solve_part1(input: &Filesystem) -> u32 {
    let mut sizes = Vec::new();
    let total_size = dir_size(&mut sizes, input);
    sizes.push(total_size); // Push the overall total on
    sizes.into_iter().filter(|x| x <= &100_000).sum()
}

#[aoc(day7, part2)]
fn solve_part2(_input: &Filesystem) -> usize {
    todo!()
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
}
