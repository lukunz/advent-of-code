use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

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
    J,
    T,
    C9,
    C8,
    C7,
    C6,
    C5,
    C4,
    C3,
    C2,
    Joker,
}

impl Card {
    fn from_char(c: char, with_joker: bool) -> Self {
        match c {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => {
                if with_joker {
                    Self::Joker
                } else {
                    Self::J
                }
            }
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

impl Hand {
    fn from_str(s: &str, with_joker: bool) -> Self {
        let (cards, bid) = s.split_once(' ').unwrap();
        let cards: Vec<Card> = cards
            .chars()
            .map(|c| Card::from_char(c, with_joker))
            .collect();
        let bid = bid.parse::<usize>().unwrap();

        let card_counts = count_cards(&cards, with_joker);
        let strength = calculate_strength(&card_counts);

        Hand {
            cards,
            bid,
            strength,
        }
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

fn count_cards(cards: &[Card], with_jokers: bool) -> Vec<usize> {
    let mut card_counts: HashMap<Card, usize> = HashMap::new();

    for card in cards {
        if let Some(count) = card_counts.get(card) {
            card_counts.insert(*card, count + 1);
        } else {
            card_counts.insert(*card, 1);
        }
    }

    if with_jokers {
        if let Some(joker_count) = card_counts.remove(&Card::Joker) {
            if let Some((high_card, count)) =
                card_counts
                    .iter()
                    .reduce(|(card_a, count_a), (card_b, count_b)| {
                        if count_a > count_b {
                            (card_a, count_a)
                        } else {
                            (card_b, count_b)
                        }
                    })
            {
                card_counts.insert(*high_card, count + joker_count);
            } else {
                card_counts.insert(Card::Joker, joker_count);
            }
        }
    }

    card_counts.into_values().collect()
}

fn calculate_strength(card_counts: &[usize]) -> Strength {
    if is_five_of_a_kind(card_counts) {
        Strength::FiveOfAKind
    } else if is_four_of_a_kind(card_counts) {
        Strength::FourOfAKind
    } else if is_full_house(card_counts) {
        Strength::FullHouse
    } else if is_three_of_a_kind(card_counts) {
        Strength::ThreeOfAKind
    } else if is_two_pair(card_counts) {
        Strength::TwoPair
    } else if is_one_pair(card_counts) {
        Strength::OnePair
    } else {
        Strength::HighCard
    }
}

fn is_five_of_a_kind(card_counts: &[usize]) -> bool {
    card_counts.contains(&5)
}

fn is_four_of_a_kind(card_counts: &[usize]) -> bool {
    card_counts.contains(&4)
}

fn is_full_house(card_counts: &[usize]) -> bool {
    card_counts.contains(&3) && card_counts.contains(&2)
}

fn is_three_of_a_kind(card_counts: &[usize]) -> bool {
    card_counts.contains(&3) && !card_counts.contains(&2)
}

fn is_two_pair(card_counts: &[usize]) -> bool {
    card_counts.iter().filter(|c| **c == 2).count() == 2
}

fn is_one_pair(card_counts: &[usize]) -> bool {
    card_counts.iter().filter(|c| **c == 2).count() == 1
}

fn calculate_result(data: String, with_joker: bool) -> usize {
    let mut hands: Vec<Hand> = data
        .lines()
        .map(|h| Hand::from_str(h, with_joker))
        .collect();

    hands.sort();

    hands
        .iter()
        .rev()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * hand.bid)
        .sum()
}
fn main() {
    let data = fs::read_to_string("day7.txt").expect("Can't read input file");

    let part1_result = calculate_result(data.clone(), false);
    let part2_result = calculate_result(data, true);

    println!("Day 7 Part 1: {}", part1_result);
    println!("Day 7 Part 2: {}", part2_result);
}
