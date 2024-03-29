use std::fs;

fn parse_number_list(number_str: &str) -> Vec<usize> {
    number_str
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect()
}

fn parse_number_ignoring_whitespace(number_str: &str) -> usize {
    let digits = number_str.chars().filter(|n| n.is_numeric());

    String::from_iter(digits).parse::<usize>().unwrap()
}

fn run_races(races: &[(usize, usize)]) -> usize {
    races
        .into_iter()
        .map(|(time, record)| {
            (0..=*time)
                .map(|t| t * (time - t))
                .filter(|d| *d > *record)
                .count()
        })
        .product()
}

fn main() {
    let data = fs::read_to_string("day6.txt").expect("Can't read input file");
    let mut lines = data.lines();
    let (_, times_str) = lines.next().unwrap().split_once(':').unwrap();
    let (_, records_str) = lines.next().unwrap().split_once(':').unwrap();

    let times = parse_number_list(times_str);
    let records = parse_number_list(records_str);
    let races: Vec<(usize, usize)> = times.into_iter().zip(records).collect();

    let part1_result: usize = run_races(&races);

    let long_race = (
        parse_number_ignoring_whitespace(times_str),
        parse_number_ignoring_whitespace(records_str),
    );

    let part2_result: usize = run_races(&[long_race]);

    println!("Day 6 Part 1: {}", part1_result);
    println!("Day 6 Part 2: {}", part2_result);
}
