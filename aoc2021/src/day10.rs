fn is_corrupt(line: &str) -> (Option<char>, Vec<char>) {
    let mut open_stack = Vec::with_capacity(line.len() / 2);

    for character in line.chars() {
        match character {
            '{' | '[' | '(' | '<' => open_stack.push(character),
            '}' | ']' | ')' | '>' => {
                // This could probably be done with ascii maths
                let opener = match character {
                    '}' => '{',
                    ']' => '[',
                    ')' => '(',
                    '>' => '<',
                    _ => panic!(),
                };

                if open_stack[open_stack.len() - 1] != opener {
                    return (Some(character), open_stack);
                } else {
                    open_stack.pop();
                }
            }
            c => panic!("Bad character {}", c),
        }
    }

    (None, open_stack)
}

#[aoc(day10, part1)]
fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|c| is_corrupt(c).0)
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!(),
        })
        .sum()
}

#[aoc(day10, part2)]
fn solve_part2(input: &str) -> usize {
    let incomplete: Vec<Vec<char>> = input
        .lines()
        .map(is_corrupt)
        // Select only non-corrupt ones
        .filter(|c| c.0 == None)
        .map(|c| c.1)
        .collect();
    let mut scores = vec![0; incomplete.len()];

    for (i, stack) in incomplete.into_iter().enumerate() {
        let mut score = 0;
        for ch in stack.into_iter().rev() {
            score *= 5;
            score += match ch {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!(),
            };
        }
        scores[i] = score;
    }

    scores.sort();
    // Length guaranteed to be odd so this will always get the middle
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&EXAMPLE_INPUT), 26397);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&EXAMPLE_INPUT), 288957);
    }

    #[test]
    fn part1_myinput() {
        let _input = crate::get_input_for_day(10);
        assert_eq!(solve_part1(&_input), 315693);
    }

    #[test]
    fn part2_myinput() {
        let _input = crate::get_input_for_day(10);
        assert_eq!(solve_part2(&_input), 1870887234);
    }
}
