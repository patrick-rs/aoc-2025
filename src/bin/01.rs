use std::path::Path;

use advent_2025::days::day1;

fn main() {
    let file_path = Path::new("./input/day1");

    let input: Vec<String> = std::fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect();

    println!("{}", day1::solve(input));
}
