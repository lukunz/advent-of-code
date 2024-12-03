enum Token {
    Mul(u32, u32),
    Do,
    Dont,
}

fn main() {
    let data = include_str!("../day03.txt");
    let mut rest = data;
    let mut tokens: Vec<Token> = Vec::new();

    loop {
        if rest.is_empty() {
            break;
        }

        match rest {
            s if s.starts_with("mul(") => {
                let number = parse_mul(rest);
                rest = &rest[4..];

                if let Some((a, b)) = number {
                    tokens.push(Token::Mul(a, b));
                }
            }
            s if s.starts_with("do()") => {
                tokens.push(Token::Do);
                rest = &rest[4..];
            }
            s if s.starts_with("don't()") => {
                tokens.push(Token::Dont);
                rest = &rest[7..];
            }
            _ => {
                rest = &rest[1..];
            }
        }
    }

    let mut part1_sum = 0;
    let mut part2_sum = 0;
    let mut enabled = true;

    for token in tokens {
        match token {
            Token::Mul(x, y) => {
                part1_sum += x * y;

                if enabled {
                    part2_sum += x * y;
                }
            }
            Token::Do => {
                enabled = true;
            }
            Token::Dont => {
                enabled = false;
            }
        }
    }

    println!("Day 03 Part 1: {}", part1_sum);
    println!("Day 03 Part 2: {}", part2_sum);
}

fn parse_mul(input: &str) -> Option<(u32, u32)> {
    if !input.starts_with("mul(") {
        return None;
    }

    let mut rest = input;
    rest = &rest[4..];

    let (number_a, end) = parse_number(rest)?;
    rest = &rest[end..];

    if !is_next_char(rest, ',') {
        return None;
    }

    rest = &rest[1..];

    let (number_b, end) = parse_number(rest)?;
    rest = &rest[end..];

    if !is_next_char(rest, ')') {
        return None;
    }

    Some((number_a, number_b))
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
