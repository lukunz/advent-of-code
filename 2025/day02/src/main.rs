use std::collections::HashSet;

fn calculate_id(n: u64, num_repeats: usize) -> u64 {
    let mag = 10_u64.pow(n.checked_ilog10().unwrap_or(0) + 1);

    let mut result: u64 = n;
    for _ in 1..num_repeats {
        result = result * mag + n;
    }

    result
}

fn normalize_ranges(ranges: Vec<(&str, &str)>) -> Vec<(String, String)> {
    let mut normalized_ranges: Vec<(String, String)> = Vec::new();

    for (start, end) in ranges {
        assert!(end.len() - start.len() < 2);

        if end.len() - start.len() == 0 {
            normalized_ranges.push((start.to_string(), end.to_string()));
        } else {
            let split_point = 10_u64.pow(end.len() as u32 - 1);

            normalized_ranges.push((split_point.to_string(), end.to_string()));
            normalized_ranges.push((start.to_string(), (split_point - 1).to_string()));
        }
    }

    normalized_ranges
}

fn find_ids(start: &str, end: &str, num_repeats: usize, ids: &mut HashSet<u64>) {
    if !start.len().is_multiple_of(num_repeats) {
        return;
    }

    let pattern_len = start.len() / num_repeats;
    let range = (start.parse::<u64>().unwrap())..=(end.parse::<u64>().unwrap());
    let start_part = start[..pattern_len].parse::<u64>().unwrap();
    let end_part = end[..pattern_len].parse::<u64>().unwrap();

    for n in start_part..=end_part {
        let id = calculate_id(n, num_repeats);

        if range.contains(&id) {
            ids.insert(id);
        }
    }
}

fn main() {
    let data = include_str!("../day02.txt").trim();

    let ranges = data
        .split(',')
        .map(|range| range.split_once('-').unwrap())
        .collect::<Vec<(&str, &str)>>();

    let ranges = normalize_ranges(ranges);

    let mut result: HashSet<u64> = HashSet::new();

    for (start, end) in &ranges {
        find_ids(start, end, 2, &mut result);
    }

    println!("Result Day 2 Part 1: {}", result.iter().sum::<u64>());

    for (start, end) in &ranges {
        for num_repeats in 3..=start.len() {
            find_ids(start, end, num_repeats, &mut result);
        }
    }

    println!("Result Day 2 Part 2: {}", result.iter().sum::<u64>());
}
