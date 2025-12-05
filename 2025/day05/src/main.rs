use std::ops::RangeInclusive;

fn parse_input(data: &str) -> (Vec<RangeInclusive<usize>>, Vec<usize>) {
    let (range_data, id_data) = data.split_once("\n\n").unwrap();

    let ranges: Vec<RangeInclusive<usize>> = range_data
        .lines()
        .map(|line| {
            let (lo, hi) = line.split_once('-').unwrap();
            lo.parse::<usize>().unwrap()..=hi.parse::<usize>().unwrap()
        })
        .collect();

    let ids: Vec<usize> = id_data
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect();

    (ranges, ids)
}

fn main() {
    let data = include_str!("../day05.txt");

    let (ranges, ids) = parse_input(data);

    let mut fresh_ids: Vec<usize> = Vec::new();

    for id in ids {
        for range in &ranges {
            if range.contains(&id) {
                fresh_ids.push(id);
                break;
            }
        }
    }

    println!("Day 05 Part 1: {}", fresh_ids.len());
}
