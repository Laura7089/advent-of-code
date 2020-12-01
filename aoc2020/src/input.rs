pub fn list_of_numbers(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect()
}
