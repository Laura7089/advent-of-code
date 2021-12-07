const DAY_COUNT: usize = 18;
const CYCLE_TIME: usize = 7;

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Vec<usize> {
    input.split(",").map(|n| n.parse().unwrap()).collect()
}
#[aoc(day6, part1)]
fn solve_part1(input: &[usize]) -> usize {
    // let mut fish: Vec<usize> = Vec::from(input);
    // let mut days_remaining = DAY_COUNT;
    // let mut step = 0;

    // println!("Start: {:?}", fish);

    // while step <= days_remaining {
    //     for i in 0..fish.len() {
    //         if fish[i] == 0 {
    //             fish.push(CYCLE_TIME + 1);
    //             fish[i] = CYCLE_TIME - 1;
    //         }
    //     }

    //     step = *fish.iter().min().unwrap();
    //     for i in 0..fish.len() {
    //         fish[i] -= step;
    //     }
    //     days_remaining -= step;
    //     println!(
    //         "Day {} (step {}): {:?}",
    //         DAY_COUNT - days_remaining,
    //         step,
    //         fish
    //     );
    // }

    // fish.len()

    let shoal = Vec::from(input);
    let mut total = 0;

    for fish in shoal.iter() {
        let num_spawns = (DAY_COUNT - fish + 1) / CYCLE_TIME;
        println!(
            "Fish {} will spawn {} times in the coming {} days",
            fish, num_spawns, DAY_COUNT
        );
        let children = 2_usize.pow(num_spawns as u32);

        total += children;
    }
    total
}

#[aoc(day6, part2)]
fn solve_part2(input: &[usize]) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "3,4,3,1,2";

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&parse_input(&EXAMPLE_INPUT)), 5934);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&parse_input(&EXAMPLE_INPUT)), unimplemented!());
    }
}
