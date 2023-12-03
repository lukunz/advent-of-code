use regex::Regex;
use std::fs;

fn convert_to_int(value: &str) -> i32 {
    match value {
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => panic!("Not a digit"),
    }
}

fn parse(data: &str, first_re: Regex, last_re: Regex) -> i32 {
    let mut sum = 0;

    for line in data.lines() {
        let mut first_matches = first_re.captures_iter(line);
        let mut last_matches = last_re.captures_iter(line);
        let (_, [first]) = first_matches.next().expect("No digits found").extract();
        let (_, [last]) = last_matches.next().expect("No digits found").extract();

        sum += convert_to_int(first) * 10 + convert_to_int(last);
    }

    sum
}

fn main() {
    let data = fs::read_to_string("day1.txt").expect("Can't read input file");

    let part1 = parse(
        &data,
        Regex::new(r".*?(\d).*").unwrap(),
        Regex::new(r".*(\d).*?").unwrap(),
    );
    let part2 = parse(
        &data,
        Regex::new(r".*?(\d|one|two|three|four|five|six|seven|eight|nine).*").unwrap(),
        Regex::new(r".*(\d|one|two|three|four|five|six|seven|eight|nine).*?").unwrap(),
    );

    println!("Day 1 Part 1: {}", part1);
    println!("Day 1 Part 2: {}", part2);
}
