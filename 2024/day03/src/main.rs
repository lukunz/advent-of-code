fn main() {
    let data = include_str!("../day03.txt");
    let mut rest = data;
    let mut numbers: Vec<(u32, u32)> = Vec::new();

    while let Some(start) = rest.find("mul(") {
        rest = &rest[start + 4..];

        let number = parse_number(rest);

        if number.is_none() {
            continue;
        }

        let (number_a, end) = number.unwrap();
        rest = &rest[end..];

        if !is_next_char(rest, ',') {
            continue;
        }

        rest = &rest[1..];

        let number = parse_number(rest);

        if number.is_none() {
            continue;
        }

        let (number_b, end) = number.unwrap();
        rest = &rest[end..];

        if !is_next_char(rest, ')') {
            continue;
        }

        rest = &rest[1..];

        numbers.push((number_a, number_b));
    }

    let sum = numbers.iter().map(|(a, b)| a * b).sum::<u32>();

    println!("Day 03 Part 1: {}", sum);
}

fn parse_number(input: &str) -> Option<(u32, usize)> {
    let mut end = 0;

    for char in input.chars().take(3) {
        if char.is_ascii_digit() {
            end += 1;
        } else {
            break;
        }
    }

    if end > 0 {
        let number = input[..end].parse::<u32>();

        if let Ok(number) = number {
            return Some((number, end));
        }
    }

    None
}

fn is_next_char(input: &str, c: char) -> bool {
    let char = input.chars().next();

    char.is_some() && char.unwrap() == c
}
