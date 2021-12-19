use hashbrown::{HashMap, HashSet};

type Links = HashMap<Cave, Vec<Cave>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Cave {
    Start,
    End,
    Large(usize),
    Small(usize),
}

#[aoc_generator(day12)]
fn parse_input(input: &str) -> Links {
    let lines = input.lines().count();
    let mut ids: HashMap<String, usize> = HashMap::with_capacity(lines / 2);
    let mut id_counter = 0;
    let mut links: Links = HashMap::with_capacity(lines);

    for line in input.lines() {
        let mut pair = [Cave::Start; 2];
        // let mut contains_start = false;

        let mut split = line.split('-');
        for cave in &mut pair {
            *cave = match split.next().unwrap() {
                "start" => {
                    // contains_start = true;
                    Cave::Start
                }
                "end" => Cave::End,
                cave_raw => {
                    // Generate a unique cave ID
                    let this_id = if let Some(i) = ids.get(cave_raw) {
                        *i
                    } else {
                        ids.insert(cave_raw.to_string(), id_counter);
                        id_counter += 1;
                        id_counter - 1
                    };

                    if cave_raw == cave_raw.to_lowercase() {
                        Cave::Small(this_id)
                    } else {
                        Cave::Large(this_id)
                    }
                }
            };
        }

        // for i in 0..=(contains_start as usize) {
        for i in 0..2 {
            links
                .entry(pair[i])
                .or_insert_with(|| Vec::with_capacity(lines / 5))
                .push(pair[(i + 1) % 2]);
        }
    }

    links
}

fn find_routes_part1(links: &Links, current: &Cave, visited: &mut HashSet<Cave>) -> usize {
    links
        .get(current)
        .unwrap()
        .iter()
        .map(|cave| match cave {
            Cave::End => 1,
            Cave::Small(_) if !visited.contains(cave) => {
                visited.insert(*cave);
                let num = find_routes_part1(links, cave, visited);
                visited.remove(cave);
                num
            }
            Cave::Large(_) => find_routes_part1(links, cave, visited),
            _ => 0,
        })
        .sum()
}

#[aoc(day12, part1)]
fn solve_part1(input: &Links) -> usize {
    let mut visited = HashSet::with_capacity(input.len() / 2);
    visited.insert(Cave::Start);
    find_routes_part1(input, &Cave::Start, &mut visited)
}

fn find_routes_part2(
    links: &Links,
    current: &Cave,
    visited: &mut HashSet<Cave>,
    double: bool,
) -> usize {
    links
        .get(current)
        .unwrap()
        .iter()
        .filter_map(|cave| match cave {
            Cave::End => Some(1),
            Cave::Large(_) => Some(find_routes_part2(links, cave, visited, double)),
            Cave::Small(_) if !visited.contains(cave) => {
                visited.insert(*cave);
                let num = Some(find_routes_part2(links, cave, visited, double));
                visited.remove(cave);
                num
            }
            Cave::Small(_) if !double => Some(find_routes_part2(links, cave, visited, true)),
            _ => None,
        })
        .sum()
}

#[aoc(day12, part2)]
fn solve_part2(input: &Links) -> usize {
    let mut visited = HashSet::with_capacity(input.len() / 2);
    find_routes_part2(input, &Cave::Start, &mut visited, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    const EXAMPLE_INPUT_LARGER: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    const EXAMPLE_INPUT_EVEN_LARGER: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&parse_input(&EXAMPLE_INPUT)), 10);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&parse_input(&EXAMPLE_INPUT)), 36);
    }

    #[test]
    fn part1_l_example() {
        assert_eq!(solve_part1(&parse_input(&EXAMPLE_INPUT_LARGER)), 19);
    }

    #[test]
    fn part2_l_example() {
        assert_eq!(solve_part2(&parse_input(&EXAMPLE_INPUT_LARGER)), 103);
    }

    #[test]
    fn part1_el_example() {
        assert_eq!(solve_part1(&parse_input(&EXAMPLE_INPUT_EVEN_LARGER)), 226);
    }

    #[test]
    fn part2_el_example() {
        assert_eq!(solve_part2(&parse_input(&EXAMPLE_INPUT_EVEN_LARGER)), 3509);
    }

    #[test]
    fn part1_myinput() {
        let _input = crate::get_input_for_day(12);
        assert_eq!(solve_part1(&parse_input(&_input)), 3463);
    }

    #[test]
    fn part2_myinput() {
        let _input = crate::get_input_for_day(12);
        assert_eq!(solve_part2(&parse_input(&_input)), 91533);
    }
}
