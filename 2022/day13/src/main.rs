use std::cmp::Ordering;
use std::fs;
use std::slice::Iter;

#[derive(Debug, PartialEq)]
enum Token {
    OpenParen,
    CloseParen,
    Number(u64),
}

#[derive(Debug)]
enum Item {
    List(Vec<Item>),
    Value(u64),
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Value(self_value), Self::Value(other_value)) => self_value.cmp(other_value),
            (Self::List(self_items), Self::List(other_items)) => self_items.cmp(other_items),
            (Self::Value(self_value), Self::List(other_items)) => {
                vec![Self::Value(*self_value)].cmp(other_items)
            }
            (Self::List(self_items), Self::Value(other_value)) => {
                self_items.cmp(&vec![Self::Value(*other_value)])
            }
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Value(self_value), Self::Value(other_value)) => self_value.eq(other_value),
            (Self::List(self_items), Self::List(other_items)) => self_items.eq(other_items),
            (Self::Value(self_value), Self::List(other_items)) => {
                vec![Self::Value(*self_value)].eq(other_items)
            }
            (Self::List(self_items), Self::Value(other_value)) => {
                self_items.eq(&vec![Self::Value(*other_value)])
            }
        }
    }
}

impl Eq for Item {}

fn lex_line(line: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut number_buffer: Option<String> = None;

    for c in line.chars() {
        match c {
            '[' => {
                tokens.push(Token::OpenParen);
            }
            ']' => {
                if let Some(number_str) = number_buffer {
                    tokens.push(Token::Number(number_str.parse().expect("impossible")));
                    number_buffer = None;
                }
                tokens.push(Token::CloseParen);
            }
            '0'..='9' => {
                if let Some(mut number_string) = number_buffer {
                    number_string.push(c);
                    number_buffer = Some(number_string);
                } else {
                    number_buffer = Some(String::from(c));
                }
            }
            ',' => {
                if let Some(number_str) = number_buffer {
                    tokens.push(Token::Number(number_str.parse().expect("impossible")));
                    number_buffer = None;
                }
            }
            _ => {}
        }
    }

    tokens
}

fn parse_line(line: &str) -> Item {
    let tokens = lex_line(line);
    let mut tokens = tokens.iter();
    // Remove starting OpenParam
    tokens.next();

    parse_list(&mut tokens)
}

fn parse_list(tokens: &mut Iter<Token>) -> Item {
    let mut items = Vec::new();

    while let Some(token) = tokens.next() {
        match token {
            Token::CloseParen => {
                break;
            }
            Token::Number(number) => {
                items.push(Item::Value(*number));
            }
            Token::OpenParen => {
                items.push(parse_list(tokens));
            }
        }
    }

    Item::List(items)
}

fn part_one(file: &str) -> usize {
    file.split("\n\n")
        .enumerate()
        .map(|(index, pair_str)| {
            let (left, right) = pair_str.split_once('\n').expect("Invalid input");
            let (left, right) = (parse_line(left), parse_line(right));

            if left.cmp(&right) == Ordering::Less {
                index + 1
            } else {
                0
            }
        })
        .sum()
}

fn part_two(file: &str) -> usize {
    let mut lines: Vec<Item> = file
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| parse_line(line))
        .collect();

    lines.push(Item::List(vec![Item::List(vec![Item::Value(2)])]));
    lines.push(Item::List(vec![Item::List(vec![Item::Value(6)])]));

    lines.sort();

    let divider = vec![
        Item::List(vec![Item::List(vec![Item::Value(2)])]),
        Item::List(vec![Item::List(vec![Item::Value(6)])]),
    ];

    lines
        .iter()
        .enumerate()
        .map(|(index, line)| if divider.contains(line) { index + 1 } else { 1 })
        .reduce(|a, b| a * b)
        .expect("error")
}

fn main() {
    let data = fs::read_to_string("../input.txt").expect("Can't read input file");
    let sorted_count = part_one(&data);
    let key = part_two(&data);

    println!("Part one: {}", sorted_count);
    println!("Part two: {}", key);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_line() {
        let line = "[1,2,34,[1]]";
        let tokens = lex_line(line);

        assert_eq!(8, tokens.len());
        assert_eq!(Token::OpenParen, tokens[0]);
        assert_eq!(Token::Number(1), tokens[1]);
        assert_eq!(Token::Number(2), tokens[2]);
        assert_eq!(Token::Number(34), tokens[3]);
        assert_eq!(Token::OpenParen, tokens[4]);
        assert_eq!(Token::Number(1), tokens[5]);
        assert_eq!(Token::CloseParen, tokens[6]);
        assert_eq!(Token::CloseParen, tokens[7]);
    }
}
