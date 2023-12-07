use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

#[derive(Debug, PartialOrd, PartialEq, Ord, Eq)]
enum Strength {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Clone, Copy)]
enum Card {
    A,
    K,
    Q,
    T,
    C9,
    C8,
    C7,
    C6,
    C5,
    C4,
    C3,
    C2,
    J,
}

impl Card {
    fn from_char(c: char) -> Self {
        match c {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::T,
            '9' => Self::C9,
            '8' => Self::C8,
            '7' => Self::C7,
            '6' => Self::C6,
            '5' => Self::C5,
            '4' => Self::C4,
            '3' => Self::C3,
            '2' => Self::C2,
            _ => panic!("Unknown card {}", c),
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: usize,
    strength: Strength,
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(' ').unwrap();
        let cards: Vec<Card> = cards.chars().map(Card::from_char).collect();
        let bid = bid.parse::<usize>().unwrap();

        let (card_counts, joker_count) = count_cards(&cards);
        let strength = calculate_strength(&card_counts, joker_count);

        Ok(Hand {
            cards,
            bid,
            strength,
        })
    }
}

impl Eq for Hand {}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.strength == other.strength && self.cards.eq(&other.cards)
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.strength.cmp(&other.strength) {
            Ordering::Equal => {
                for (a, b) in self.cards.iter().zip(&other.cards) {
                    match a.cmp(b) {
                        Ordering::Equal => {}
                        o => return o,
                    }
                }

                Ordering::Equal
            }
            o => o,
        }
    }
}

fn count_cards(cards: &[Card]) -> (Vec<usize>, usize) {
    let mut card_counts: HashMap<Card, usize> = HashMap::new();

    for card in cards {
        if let Some(count) = card_counts.get(card) {
            card_counts.insert(*card, count + 1);
        } else {
            card_counts.insert(*card, 1);
        }
    }

    let joker_count = card_counts.remove(&Card::J).unwrap_or(0);

    (card_counts.into_values().collect(), joker_count)
}

fn calculate_strength(card_counts: &[usize], joker_count: usize) -> Strength {
    if is_five_of_a_kind(card_counts, joker_count) {
        Strength::FiveOfAKind
    } else if is_four_of_a_kind(card_counts, joker_count) {
        Strength::FourOfAKind
    } else if is_full_house(card_counts, joker_count) {
        Strength::FullHouse
    } else if is_three_of_a_kind(card_counts, joker_count) {
        Strength::ThreeOfAKind
    } else if is_two_pair(card_counts, joker_count) {
        Strength::TwoPair
    } else if is_one_pair(card_counts, joker_count) {
        Strength::OnePair
    } else {
        Strength::HighCard
    }
}

fn is_five_of_a_kind(card_counts: &[usize], joker_count: usize) -> bool {
    if card_counts.is_empty() {
        true
    } else {
        card_counts.iter().max().unwrap() + joker_count == 5
    }
}

fn is_four_of_a_kind(card_counts: &[usize], joker_count: usize) -> bool {
    if card_counts.is_empty() {
        false
    } else {
        card_counts.iter().max().unwrap() + joker_count == 4
    }
}

fn is_full_house(card_counts: &[usize], joker_count: usize) -> bool {
    if joker_count > 3 {
        false
    } else if joker_count == 3 {
        true
    } else {
        card_counts.contains(&3) && card_counts.contains(&2)
            || joker_count == 1 && card_counts.iter().filter(|c| **c == 2).count() == 2
            || joker_count == 1 && card_counts.contains(&3)
            || joker_count == 2 && card_counts.contains(&3)
            || joker_count == 2 && card_counts.contains(&2)
    }
}

fn is_three_of_a_kind(card_counts: &[usize], joker_count: usize) -> bool {
    if joker_count > 2 {
        false
    } else if joker_count == 2 {
        true
    } else {
        card_counts.contains(&3) && !card_counts.contains(&2)
            || joker_count == 1 && card_counts.contains(&2)
    }
}

fn is_two_pair(card_counts: &[usize], joker_count: usize) -> bool {
    if joker_count > 1 {
        false
    } else {
        card_counts.iter().filter(|c| **c == 2).count() == 2
            || joker_count == 1 && card_counts.contains(&2)
    }
}

fn is_one_pair(card_counts: &[usize], joker_count: usize) -> bool {
    joker_count == 1 || card_counts.iter().filter(|c| **c == 2).count() == 1
}

fn main() {
    let data = fs::read_to_string("day7.txt").expect("Can't read input file");

    let mut hands: Vec<Hand> = data.lines().map(|h| Hand::from_str(h).unwrap()).collect();

    hands.sort();

    let part1_result: usize = hands
        .iter()
        .rev()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * hand.bid)
        .sum();

    println!("Day 7 Part 2: {}", part1_result);
}
