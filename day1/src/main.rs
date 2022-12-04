use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("day1/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut totals = Vec::new();
    let mut current_total = 0;
    let mut max_total = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            totals.push(current_total);
            max_total = max(current_total, max_total);
            current_total = 0;
        } else {
            let calories: u32 = line.parse().unwrap();
            current_total += calories;
        }
    }

    totals.push(current_total);
    max_total = max(current_total, max_total);
    totals.sort();
    let top_three_total: u32 = totals.iter().rev().take(3).sum();

    println!("Top Elf Total: {max_total}");
    println!("Top Three Elf Total: {top_three_total}");
}
