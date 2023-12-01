use std::fs;

fn main() {
    let data = fs::read_to_string("day1.txt").expect("Can't read input file");

    let result: i32 = data.lines()
        .map(|line| line.chars())
        .map(|mut chars| (
                chars.clone().find(|c| c.is_numeric()).expect("No digets").to_string().parse::<i32>().unwrap(),
                chars.rfind(|c| c.is_numeric()).expect("No digets").to_string().parse::<i32>().unwrap()
            )
        ).map(|(first_digit, last_digit)| first_digit * 10 + last_digit)
        .sum();

    println!("Day 1 Part 1: {}", result);
}
