use std::fs;

fn calculate_hash(step: &str) -> u64 {
    step.chars()
        .map(|c| c as u64)
        .fold(0, |hash, c| ((hash + c) * 17) % 256)
}

fn main() {
    let data = fs::read_to_string("day15.txt").expect("Can't read input file");

    let steps: Vec<&str> = data.split(',').map(|step| step.trim()).collect();

    let part1_result: u64 = steps.iter().map(|step| calculate_hash(step)).sum();

    println!("Day 15 Part 1: {}", part1_result);
}
