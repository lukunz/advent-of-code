use std::collections::{HashMap, HashSet};

fn main() {
    let data = include_str!("../day23.txt");

    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in data.lines() {
        let (node1, node2) = line.split_once("-").expect("Invalid input format");

        let node1_entry = graph.entry(node1).or_default();
        node1_entry.push(node2);
        let node2_entry = graph.entry(node2).or_default();
        node2_entry.push(node1);
    }

    let t_nodes = graph.keys().filter(|node| node.starts_with("t"));

    let mut groups: HashSet<Vec<&str>> = HashSet::new();

    for &node1 in t_nodes {
        for &node2 in graph.get(node1).unwrap() {
            for &node3 in graph.get(node2).unwrap() {
                if graph.get(node3).unwrap().contains(&node1) {
                    let mut group = vec![node1, node2, node3];
                    group.sort_unstable();
                    groups.insert(group);
                }
            }
        }
    }

    println!("Day 23 Part 1: {}", groups.len());
}
