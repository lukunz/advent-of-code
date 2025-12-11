use std::collections::{HashMap, VecDeque};

fn parse_input(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(start, end)| (start, end.split_whitespace().collect::<Vec<&str>>()))
        .collect()
}

fn main() {
    let input = include_str!("../day11.txt");

    let graph = parse_input(input);

    let mut queue: VecDeque<&str> = VecDeque::new();
    queue.push_front("you");

    let mut out_count = 0;

    while let Some(node) = queue.pop_front() {
        if node == "out" {
            out_count += 1;
            continue;
        }

        if let Some(next_nodes) = graph.get(node) {
            for next_node in next_nodes {
                queue.push_back(next_node);
            }
        }
    }

    println!("Day 11 Part 1: {}", out_count);
}
