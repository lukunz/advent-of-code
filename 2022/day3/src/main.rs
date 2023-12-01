use std::collections::HashSet;
use std::fs;

fn item_to_priority(item: char) -> u32 {
    match item {
        'a'..='z' => item as u32 - 96,
        'A'..='Z' => item as u32 - 64 + 26,
        _ => 0
    }
}

fn part1(data: &String) {
    let sum_of_priorities: u32 = data.lines()
        .map(|line| {
            let (left, right): (&str, &str) = line.split_at(line.len() / 2);
            let left: HashSet<char> = HashSet::from_iter(left.chars());
            let right: HashSet<char> = HashSet::from_iter(right.chars());

            let intersect = left.intersection(&right);

            intersect.map(|c| item_to_priority(c.clone())).sum::<u32>()
        }).sum();

    println!("{sum_of_priorities}");
}

fn part2(data: &String) {
    let sum_of_priorities: u32 = data.lines()
        .map(|line| HashSet::from_iter(line.chars()))
        .collect::<Vec<HashSet<char>>>()
        .chunks(3)
        .map(|group| {
            if group.is_empty() {
                return 0;
            }
            let mut group = Vec::from(group);
            let mut result_set = group.pop().unwrap();
            for line in group {
                result_set = result_set.intersection(&line)
                    .map(|c| c.clone())
                    .collect();
            }

            result_set.iter().map(|c| item_to_priority(*c)).sum()
        }).sum();

    println!("{sum_of_priorities}");
}

fn main() {
    let data = fs::read_to_string("../input.txt").expect("Can't read input file");

    part1(&data);
    part2(&data);
}