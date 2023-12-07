use std::fs;

fn parse_number_list(number_str: &str) -> Vec<usize> {
    number_str
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect()
}

fn main() {
    let data = fs::read_to_string("day6.txt").expect("Can't read input file");
    let mut lines = data.lines();
    let (_, times) = lines.next().unwrap().split_once(':').unwrap();
    let (_, records) = lines.next().unwrap().split_once(':').unwrap();

    let times = parse_number_list(times);
    let records = parse_number_list(records);
    let races = times.iter().zip(records);

    let part1_result: usize = races
        .map(|(time, record)| {
            (0..=*time)
                .map(|t| t * (time - t))
                .filter(|d| *d > record)
                .count()
        })
        .product();

    println!("Day 6 Part 1: {}", part1_result);
}
