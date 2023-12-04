use std::collections::HashSet;
use std::fs;

struct Card {
    id: usize,
    winning_numbers: HashSet<usize>,
    numbers: HashSet<usize>,
}

fn parse_number_list(number_str: &str) -> HashSet<usize> {
    number_str
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect()
}

fn parse_card(line: &str) -> Card {
    let (card_str, all_numbers_str) = line.split_once(": ").unwrap();
    let (_, card_id_str) = card_str.split_once(' ').unwrap();
    let (winning_numbers_str, numbers_str) = all_numbers_str.split_once(" | ").unwrap();
    let winning_numbers = parse_number_list(winning_numbers_str);
    let numbers = parse_number_list(numbers_str);

    Card {
        id: card_id_str.trim().parse::<usize>().unwrap(),
        winning_numbers,
        numbers,
    }
}

fn calculate_score(card: &Card) -> usize {
    let matching_numbers_count = card.numbers.intersection(&card.winning_numbers).count() as u32;

    if matching_numbers_count == 0 {
        0
    } else {
        2usize.pow(matching_numbers_count - 1)
    }
}

fn main() {
    let data = fs::read_to_string("day4.txt").expect("Can't read input file");

    let cards: Vec<Card> = data.lines().map(parse_card).collect();

    let part1_result: usize = cards.iter().map(calculate_score).sum();

    println!("Day 4 Part 1: {}", part1_result);
}
