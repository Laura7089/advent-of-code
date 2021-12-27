type Bounds = [[isize; 2]; 2];

#[derive(Copy, Clone, Default, Debug)]
struct Probe {
    x: isize,
    y: isize,
    vx: isize,
    vy: isize,
}

#[aoc_generator(day17)]
pub fn parse_input(input: &str) -> Bounds {
    let replaced = input.replace(",", "");
    let mut line = replaced.split_whitespace().skip(2);
    let mut bounds = [[0; 2]; 2];

    // (x,y) at indices (0,1)
    for axis in &mut bounds {
        let mut raw = line
            .next()
            .unwrap()
            .split('=')
            .nth(1)
            .unwrap()
            .split("..")
            .map(str::parse);
        axis[0] = raw.next().unwrap().unwrap();
        axis[1] = raw.next().unwrap().unwrap();
    }

    bounds
}

fn _sim_step(probe: Probe) -> Probe {
    Probe {
        x: probe.x + probe.vx,
        y: probe.y + probe.vy,
        vx: if probe.vx.is_positive() {
            probe.vx - 1
        } else if probe.vx.is_negative() {
            probe.vx + 1
        } else {
            0
        },
        vy: probe.vy - 1,
    }
}

#[aoc(day17, part1)]
pub fn solve_part1(_input: &Bounds) -> usize {
    todo!()
}

#[aoc(day17, part2)]
pub fn solve_part2(_input: &Bounds) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&parse_input(&EXAMPLE_INPUT)), 45);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&parse_input(&EXAMPLE_INPUT)), todo!());
    }

    #[test]
    fn part1_myinput() {
        let _input = crate::get_input_for_day(17);
        assert_eq!(solve_part1(&parse_input(&_input)), todo!());
    }

    #[test]
    fn part2_myinput() {
        let _input = crate::get_input_for_day(17);
        assert_eq!(solve_part2(&parse_input(&_input)), todo!());
    }
}
