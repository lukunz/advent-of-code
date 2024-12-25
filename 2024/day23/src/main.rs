use bit_set::BitSet;
use std::collections::{HashMap, HashSet};

fn main() {
    let data = include_str!("../day23.txt");

    let graph = parse_input(data);

    println!("Day 23 Part 1: {}", part1(&graph));
    println!("Day 23 Part 2: {}", part2(&graph));
}

fn parse_input(data: &str) -> HashMap<&str, Vec<&str>> {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in data.lines() {
        let (node1, node2) = line.split_once("-").expect("Invalid input format");

        let node1_entry = graph.entry(node1).or_default();
        node1_entry.push(node2);
        let node2_entry = graph.entry(node2).or_default();
        node2_entry.push(node1);
    }

    graph
}

fn part1(graph: &HashMap<&str, Vec<&str>>) -> usize {
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

    groups.len()
}

fn part2(graph: &HashMap<&str, Vec<&str>>) -> String {
    let nodes = graph.keys().collect::<Vec<&&str>>();

    let graph: HashMap<usize, BitSet> = graph
        .iter()
        .map(|(node, connections)| {
            let node = nodes.iter().position(|&n| n == node).unwrap();
            let connections = connections
                .iter()
                .map(|&node| nodes.iter().position(|&n| *n == node).unwrap())
                .collect::<BitSet>();

            (node, connections)
        })
        .collect();

    let mut groups: Vec<BitSet> = graph
        .keys()
        .map(|&node| {
            let mut group = BitSet::new();
            group.insert(node);
            group
        })
        .collect();

    let mut visited_groups: HashMap<BitSet, usize> = HashMap::new();
    let mut max_group_size = 0;

    while let Some(group) = groups.pop() {
        if visited_groups.contains_key(&group) {
            continue;
        }
        visited_groups.insert(group.clone(), group.len());
        let new_groups = extend_group(&graph, &group);

        if new_groups.is_empty() {
            max_group_size = max_group_size.max(group.len());
        } else {
            groups.extend(new_groups);
        }
    }

    let big_groups = visited_groups
        .iter()
        .filter(|(_, len)| len >= &&max_group_size)
        .map(|(group, _)| {
            let mut result = group.iter().map(|node| *nodes[node]).collect::<Vec<&str>>();
            result.sort();
            result
        })
        .map(|group| group.join(","))
        .collect::<Vec<String>>();

    big_groups.first().unwrap().to_string()
}

fn extend_group(graph: &HashMap<usize, BitSet>, group: &BitSet) -> Vec<BitSet> {
    let mut candidates_it = group.iter().map(|node| graph.get(&node).unwrap());

    let first = candidates_it.next().unwrap().clone();

    let mut candidates = candidates_it.fold(first, |mut acc, b| {
        acc.intersect_with(b);
        acc
    });

    candidates.difference_with(group);

    candidates
        .iter()
        .map(|candidate| {
            let mut new_group = group.clone();
            new_group.insert(candidate);
            new_group
        })
        .collect()
}
