use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Rule {
    before: u32,
    after: Vec<u32>,
}

impl PartialOrd for Rule {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.before == other.before {
            return Some(Ordering::Equal);
        }

        if self.after.contains(&other.before) {
            return Some(Ordering::Less);
        }

        if other.after.contains(&self.before) {
            return Some(Ordering::Greater);
        }

        None
    }
}

fn sum_middles(updates: &[Vec<u32>]) -> u32 {
    updates
        .iter()
        .map(|update| {
            assert_eq!(update.len() % 2, 1);

            update[update.len() / 2]
        })
        .sum::<u32>()
}

fn main() {
    let data = include_str!("../day05.txt");

    let (rules, updates) = data.split_once("\n\n").expect("Input has wrong format");

    let single_rules = rules
        .lines()
        .map(|line| {
            let (a, b) = line.split_once("|").expect("Line has wrong format");

            (
                a.parse::<u32>().expect("Before is not a number"),
                b.parse::<u32>().expect("After is not a number"),
            )
        })
        .collect::<Vec<_>>();

    let mut rule_book: HashMap<u32, Rule> = HashMap::new();

    for (before, after) in single_rules {
        if let Some(rule) = rule_book.get_mut(&before) {
            rule.after.push(after);
        } else {
            rule_book.insert(
                before,
                Rule {
                    before,
                    after: vec![after],
                },
            );
        }

        rule_book.entry(after).or_insert(Rule {
            before: after,
            after: Vec::new(),
        });
    }

    let updates = updates
        .lines()
        .map(|line| {
            line.split(',')
                .map(|item| item.parse::<u32>().expect("Update item is not a number"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let (valid_updates, invalid_updates): (Vec<_>, Vec<_>) = updates
        .into_iter()
        .partition(|update| update.iter().map(|&number| &rule_book[&number]).is_sorted());

    let part1_result = sum_middles(&valid_updates);

    let fixed_updates = invalid_updates
        .iter()
        .map(|update| {
            let mut rules = update
                .iter()
                .map(|&number| &rule_book[&number])
                .collect::<Vec<_>>();
            rules.sort_by(|a, b| a.partial_cmp(b).unwrap());

            rules.iter().map(|r| r.before).collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let part2_result = sum_middles(&fixed_updates);

    println!("Day 05 Part 1: {:?}", part1_result);
    println!("Day 05 Part 2: {:?}", part2_result);
}
