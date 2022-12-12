#[allow(non_camel_case_types)]
#[derive(Copy, Clone, PartialEq, Debug)]
enum Instr {
    noop,
    addx(isize),
}

#[derive(Clone, PartialEq, Debug)]
#[allow(non_snake_case)]
struct CPUState {
    X: isize,
    cycle: usize,
}

#[aoc_generator(day10)]
fn generate(input: &str) -> Vec<Instr> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(' ');
            match split.next().unwrap() {
                "noop" => Instr::noop,
                "addx" => Instr::addx(split.next().unwrap().parse().unwrap()),
                s => panic!("Cannot parse input line '{s}'"),
            }
        })
        .collect()
}

const OUTPUT_START: usize = 20;
const OUTPUT_PERIOD: usize = 40;

fn is_output_cycle(cycle: usize) -> bool {
    if cycle < OUTPUT_START {
        return false;
    }
    (cycle - OUTPUT_START) % OUTPUT_PERIOD == 0
}

#[aoc(day10, part1)]
fn solve_part1(input: &[Instr]) -> isize {
    input
        .iter()
        .scan(CPUState { X: 1, cycle: 1 }, |state, &ins| {
            let prev = state.clone();
            match ins {
                Instr::noop => state.cycle += 1,
                Instr::addx(x) => {
                    state.cycle += 2;
                    state.X += x;
                    let half_cycle = prev.cycle + 1;
                    if is_output_cycle(half_cycle) {
                        return Some(half_cycle as isize * prev.X);
                    }
                }
            }

            if is_output_cycle(prev.cycle) {
                Some(prev.cycle as isize * prev.X)
            } else {
                Some(0) // slightly inelegant
            }
        })
        .sum()
}

const SCREEN_SIZE: (usize, usize) = (40, 6);

fn perform_output(state: &CPUState, buf: &mut String) {
    buf.push(
        if isize::abs_diff(state.X, ((state.cycle - 1) % SCREEN_SIZE.0) as isize) <= 1 {
            '#'
        } else {
            '.'
        },
    );
    if state.cycle % 40 == 0 && state.cycle != SCREEN_SIZE.0 * SCREEN_SIZE.1 {
        buf.push('\n');
    }
}

#[aoc(day10, part2)]
fn solve_part2(input: &[Instr]) -> String {
    let capacity = ((SCREEN_SIZE.0 + 1) * SCREEN_SIZE.1) + 1;
    let mut out = String::with_capacity(capacity);
    // Extra newline at the beginning so the AOC output is readable
    out.push('\n');

    let mut state = CPUState { X: 1, cycle: 1 };

    for &ins in input {
        // Output is performed *during* the cycle
        perform_output(&state, &mut out);
        match ins {
            Instr::noop => (),
            Instr::addx(x) => {
                // Simulate the interim cycle
                state.cycle += 1;
                perform_output(&state, &mut out);

                state.X += x;
            }
        }
        state.cycle += 1;
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&generate(SAMPLE_INPUT)), 13140);
    }

    #[test]
    fn part1_mine() {
        assert_eq!(solve_part1(&generate(&crate::get_input(10))), 12460);
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            solve_part2(&generate(SAMPLE_INPUT)),
            "\n##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
                .to_owned()
        );
    }

    #[test]
    fn part2_mine() {
        assert_eq!(
            solve_part2(&generate(&crate::get_input(10))),
            "\n####.####.####.###..###...##..#..#.#....
#.......#.#....#..#.#..#.#..#.#.#..#....
###....#..###..#..#.#..#.#..#.##...#....
#.....#...#....###..###..####.#.#..#....
#....#....#....#....#.#..#..#.#.#..#....
####.####.#....#....#..#.#..#.#..#.####."
                .to_owned()
        );
    }
}
