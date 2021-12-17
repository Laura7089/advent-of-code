use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Cave {
    Start,
    End,
    Large(usize),
    Small(usize),
}

#[aoc_generator(day12)]
fn parse_input(input: &str) -> Vec<[Cave; 2]> {
    let lines = input.lines().count();
    let mut ids: HashMap<String, usize> = HashMap::with_capacity(lines / 2);
    let mut id_counter = 0;
    let mut pairs = Vec::with_capacity(lines);

    for line in input.lines() {
        let mut pair = [Cave::Start; 2];
        let mut split = line.split('-');
        for i in 0..2 {
            let id_str = split.next().unwrap();
            let this_id = if let Some(i) = ids.get(id_str) {
                *i
            } else {
                ids.insert(id_str.to_string(), id_counter);
                id_counter += 1;
                id_counter - 1
            };

            pair[i] = match id_str {
                "start" => Cave::Start,
                "end" => Cave::End,
                cave if cave == cave.to_lowercase() => Cave::Small(this_id),
                cave if cave == cave.to_uppercase() => Cave::Large(this_id),
                _ => panic!(),
            };
        }
        pairs.push(pair);
    }

    pairs
}

fn find_routes_part1(links: &[[Cave; 2]], current: &Cave, visited: &HashSet<Cave>) -> usize {
    let adjacents = links
        .iter()
        .filter_map(|[a, b]| match (a == current, b == current) {
            (true, _) if !visited.contains(b) => Some(b),
            (_, true) if !visited.contains(a) => Some(a),
            _ => None,
        });
    let mut routes = 0;

    for cave in adjacents {
        if cave == &Cave::End {
            routes += 1;
        } else {
            let mut visited = visited.clone();
            if let Cave::Small(_) = cave {
                visited.insert(*cave);
            }
            routes += find_routes_part1(links, &cave, &visited);
        }
    }
    routes
}

#[aoc(day12, part1)]
fn solve_part1(input: &[[Cave; 2]]) -> usize {
    let mut visited = HashSet::with_capacity(input.len() / 2);
    visited.insert(Cave::Start);
    find_routes_part1(input, &Cave::Start, &visited)
}

fn find_routes_part2(
    links: &[[Cave; 2]],
    current: &Cave,
    visited: &HashSet<Cave>,
    double: bool,
) -> usize {
    let mut routes = 0;
    let adjacents = links
        .iter()
        .filter_map(|[a, b]| match (a == current, b == current) {
            (true, _) => Some(b),
            (_, true) => Some(a),
            _ => None,
        })
        .filter(|c| match (c, visited.contains(c), double) {
            (Cave::Start, _, _) => false,
            (Cave::Large(_), _, _) => true,
            (Cave::Small(_), false, _) => true,
            (Cave::Small(_), true, false) => true,
            _ => false,
        });

    for cave in adjacents {
        println!("Examining cave: {:?}", cave);
        if cave == &Cave::End {
            routes += 1;
        } else {
            let mut visited = visited.clone();
            routes += if visited.contains(cave) {
                find_routes_part2(links, cave, &visited, true)
            } else {
                visited.insert(*cave);
                find_routes_part2(links, cave, &visited, double)
            };
        }
    }

    routes
}

#[aoc(day12, part2)]
fn solve_part2(input: &[[Cave; 2]]) -> usize {
    let visited = HashSet::with_capacity(input.len() / 2);
    find_routes_part2(input, &Cave::Start, &visited, false)
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(solve_part2(&parse_input(&_input)), unimplemented!());
    }
}
