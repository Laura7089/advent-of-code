use std::ops::Index;

#[derive(Debug)]
pub struct ForestedSlope {
    grid: Vec<Vec<bool>>,
    pub length: usize,
    pub width: usize,
}

impl Index<(usize, usize)> for ForestedSlope {
    type Output = bool;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        if y >= self.length {
            panic!("Index out of bounds - off the end of the slope");
        }
        &self.grid[y][x % self.width]
    }
}

#[aoc_generator(day3)]
pub fn get_slope(input: &str) -> ForestedSlope {
    let grid: Vec<Vec<bool>> = input
        .lines()
        .map(|l| l.as_bytes().iter().map(|s| s == &('#' as u8)).collect())
        .collect();
    ForestedSlope {
        length: grid.len(),
        width: grid[0].len(),
        grid,
    }
}

#[aoc(day3, part1)]
pub fn solve_input_part1(input: &ForestedSlope) -> usize {
    (0..input.length).filter(|&i| input[(i * 3, i)]).count()
}

const PART_TWO_STEPS: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

#[aoc(day3, part2)]
pub fn solve_input_part2(input: &ForestedSlope) -> usize {
    // let mut total = 1;
    // for (xmul, ymul) in PART_TWO_STEPS.iter() {
    //     total *= (0..input.length / ymul)
    //         .filter(|&i| input[(i * xmul, i * ymul)])
    //         .count();
    // }
    // total

    PART_TWO_STEPS.iter().fold(1, |prev, (xmul, ymul)| {
        prev * (0..input.length / ymul)
            .filter(|&i| input[(i * xmul, i * ymul)])
            .count()
    })
}
