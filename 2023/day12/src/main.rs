use std::collections::HashMap;
use std::fs;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Spring {
    Working,
    Defective,
    Unknown,
}

impl Spring {
    fn from_char(c: char) -> Self {
        match c {
            '#' => Self::Working,
            '.' => Self::Defective,
            '?' => Self::Unknown,
            _ => panic!("Unknown spring state {}", c),
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Entry {
    pattern: Vec<Spring>,
    numbers: Vec<usize>,
}

impl Entry {
    fn from_str(line: &str) -> Self {
        let (pattern, numbers_str) = line.split_once(' ').unwrap();

        let numbers: Vec<usize> = numbers_str
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        Entry {
            pattern: pattern.chars().map(Spring::from_char).collect(),
            numbers,
        }
    }

    fn head(&self) -> Spring {
        self.pattern[0]
    }

    fn tail(&self) -> Vec<Spring> {
        let mut new_pattern = self.pattern.clone();
        new_pattern.remove(0);

        new_pattern
    }
}

fn parse_input(data: &str) -> Vec<Entry> {
    data.lines().map(Entry::from_str).collect()
}

fn count_arrangements(entry: Entry, memo: &mut HashMap<Entry, usize>) -> usize {
    if let Some(&count) = memo.get(&entry) {
        return count;
    }

    if entry.pattern.is_empty() {
        return if entry.numbers.is_empty() { 1 } else { 0 };
    }

    let current_spring = entry.head();

    match current_spring {
        Spring::Working => {
            if entry.numbers.is_empty() {
                memo.insert(entry, 0);
                0
            } else {
                let spring_count = entry.numbers[0];
                if entry.pattern.len() >= spring_count
                    && entry
                        .pattern
                        .iter()
                        .take(spring_count)
                        .all(|&s| s != Spring::Defective)
                {
                    let mut new_pattern: Vec<Spring> = entry
                        .pattern
                        .clone()
                        .into_iter()
                        .skip(spring_count)
                        .collect();
                    if new_pattern.is_empty() || new_pattern[0] != Spring::Working {
                        if !new_pattern.is_empty() {
                            new_pattern[0] = Spring::Defective;
                        }

                        count_arrangements(
                            Entry {
                                pattern: new_pattern,
                                numbers: entry.numbers.into_iter().skip(1).collect(),
                            },
                            memo,
                        )
                    } else {
                        memo.insert(entry, 0);
                        0
                    }
                } else {
                    memo.insert(entry, 0);
                    0
                }
            }
        }
        Spring::Defective => {
            let result = count_arrangements(
                Entry {
                    pattern: entry.tail(),
                    numbers: entry.numbers.clone(),
                },
                memo,
            );

            memo.insert(entry, result);

            result
        }
        Spring::Unknown => {
            if entry.pattern.is_empty() {
                memo.insert(entry, 1);
                1
            } else {
                let mut new_pattern1 = entry.pattern.clone();
                let mut new_pattern2 = entry.pattern.clone();

                new_pattern1[0] = Spring::Working;
                new_pattern2[0] = Spring::Defective;

                let result = count_arrangements(
                    Entry {
                        pattern: new_pattern1,
                        numbers: entry.numbers.clone(),
                    },
                    memo,
                ) + count_arrangements(
                    Entry {
                        pattern: new_pattern2,
                        numbers: entry.numbers.clone(),
                    },
                    memo,
                );

                memo.insert(entry, result);

                result
            }
        }
    }
}

fn main() {
    let data = fs::read_to_string("day12.txt").expect("Can't read input file");

    let entries = parse_input(&data);
    let mut memo: HashMap<Entry, usize> = HashMap::new();

    let part1_result: usize = entries
        .into_iter()
        .map(|entry| count_arrangements(entry, &mut memo))
        .sum();

    println!("Day 12 Part 1: {}", part1_result);
}
