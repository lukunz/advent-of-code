use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut current_total = 0;
    let mut max_total = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            max_total = max(current_total, max_total);
            current_total = 0;
        } else {
            let calories: u32 = line.parse().unwrap();
            current_total += calories;
        }
    }

    max_total = max(current_total, max_total);

    println!("{max_total}");
}
