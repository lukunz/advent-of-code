use std::collections::HashSet;
use std::fs;

struct Card {
    winning_numbers: HashSet<usize>,
    numbers: HashSet<usize>,
    copies: usize,
}

fn parse_number_list(number_str: &str) -> HashSet<usize> {
    number_str
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect()
}

fn parse_card(line: &str) -> Card {
    let (_, all_numbers_str) = line.split_once(": ").unwrap();
    let (winning_numbers_str, numbers_str) = all_numbers_str.split_once(" | ").unwrap();

    Card {
        winning_numbers: parse_number_list(winning_numbers_str),
        numbers: parse_number_list(numbers_str),
        copies: 1,
    }
}

fn count_matching_numbers(card: &Card) -> u32 {
    card.numbers.intersection(&card.winning_numbers).count() as u32
}

fn calculate_score(card: &Card) -> usize {
    let matching_numbers_count = count_matching_numbers(card);

    if matching_numbers_count == 0 {
        0
    } else {
        2usize.pow(matching_numbers_count - 1)
    }
}

fn main() {
    let data = fs::read_to_string("day4.txt").expect("Can't read input file");

    let mut cards: Vec<Card> = data.lines().map(parse_card).collect();

    let part1_result: usize = cards.iter().map(calculate_score).sum();

    println!("Day 4 Part 1: {}", part1_result);

    for index in 0..cards.len() {
        let matching_numbers_count = count_matching_numbers(&cards[index]);

        for index2 in index + 1..cards.len().min(index + 1 + matching_numbers_count as usize) {
            cards[index2].copies += cards[index].copies;
        }
    }

    let part2_result: usize = cards.iter().map(|card| card.copies).sum();

    println!("Day 4 Part 2: {}", part2_result);
}
