type MapRange = [usize; 3];

fn parse_range(input: &str) -> MapRange {
    let mut elems_raw = input.split(" ");
    std::array::from_fn(|_| {
        elems_raw
            .next()
            .expect("too few elements in range")
            .parse()
            .expect("bad integer literal")
    })
}

#[derive(Debug, PartialEq, Clone)]
struct Map<K> {
    ranges: Vec<[usize; 3]>,
    source: K,
    dest: K,
}

impl<'a> Map<&'a str> {
    fn from(raw: &'a str) -> Self {
        let mut lines = raw.lines();

        let (source, dest) = {
            let line = lines.next().expect("empty string");
            line[..(line.len() - 5)]
                .split_once("-to-")
                .expect("bad map format")
        };

        Self {
            source,
            dest,
            ranges: lines.map(parse_range).collect(),
        }
    }
}

impl<K> Map<K> {
    fn lookup(&self, val: usize) -> usize {
        for &[ds, ss, len] in self.ranges.iter() {
            if (ss..=(ss + len)).contains(&val) {
                if ds > ss {
                    return val + (ds - ss);
                } else {
                    return val - (ss - ds);
                }
            }
        }
        val
    }
}

fn parse_seeds(line: &str) -> impl Iterator<Item = usize> + '_ {
    line[7..]
        .split(" ")
        .map(|v| v.parse().expect("bad integer literal"))
}

fn parse_input<'a>(input: impl IntoIterator<Item = &'a str>) -> Vec<Map<usize>> {
    let maps: Vec<_> = input.into_iter().map(Map::from).collect();
    let mut keys: Vec<_> = maps
        .iter()
        .map(|m| m.source)
        .filter(|&k| k != "seed")
        .collect();
    keys.insert(0, "seed");
    keys.push("location");

    let mut maps: Vec<_> = maps
        .into_iter()
        .map(|m| Map {
            source: keys.iter().position(|k| k == &m.source).unwrap(),
            dest: keys.iter().position(|k| k == &m.dest).unwrap(),
            ranges: m.ranges,
        })
        .collect();

    maps.sort_by_key(|m| m.source);
    maps
}

fn follow_through(maps: &[Map<usize>], seed: usize) -> usize {
    let mut cur = maps[0].lookup(seed);
    let mut dest = maps[0].dest;
    for _ in 1..maps.len() {
        cur = maps[dest].lookup(cur);
        dest = maps[dest].dest;
    }
    cur
}

#[aoc(day05, part1)]
fn solve_part1(input: &str) -> usize {
    let mut iter = input.split("\n\n");
    let seeds = parse_seeds(&iter.next().expect("empty input"));
    let maps = parse_input(iter);

    seeds
        .map(|s| follow_through(&maps, s))
        .min()
        .expect("no maps in input")
}

#[aoc(day05, part2)]
fn solve_part2(input: &str) -> usize {
    let mut iter = input.split("\n\n");
    let seeds: Vec<_> = parse_seeds(&iter.next().expect("empty input")).collect();
    let seeds = seeds.chunks(2).map(|c| (c[0], c[1]));

    let maps = parse_input(iter);

    seeds
        .flat_map(|(start, len)| start..=(start + len))
        .map(|s| follow_through(&maps, s))
        .min()
        .expect("no maps in input")
}

#[cfg(test)]
mod tests {
    #![allow(unreachable_code)]
    use super::*;

    const SAMPLE_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part1(SAMPLE_INPUT), 35);
        }

        #[test]
        fn mine() {
            assert_eq!(solve_part1(&crate::get_input(05)), 251346198);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(solve_part2(SAMPLE_INPUT), 46);
        }

        #[test]
        #[ignore]
        fn mine() {
            assert_eq!(solve_part2(&crate::get_input(05)), todo!());
        }
    }
}
