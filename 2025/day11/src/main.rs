use std::collections::HashMap;

type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;

#[derive(Debug, Clone)]
struct Counter {
    outs: u64,
    fft: u64,
    dac: u64,
    fft_dac: u64,
}

fn parse_input(input: &str) -> Graph<'_> {
    input
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(start, end)| (start, end.split_whitespace().collect::<Vec<&str>>()))
        .collect()
}

fn reverse_graph<'a>(graph: &'a Graph) -> Graph<'a> {
    let mut reversed_graph: Graph<'a> = HashMap::new();

    for (node, outs) in graph {
        for out in outs {
            let targets = reversed_graph.entry(out).or_default();
            targets.push(node);
        }
    }

    reversed_graph
}

fn count_outs<'a>(
    graph: &'a Graph,
    mut cache: HashMap<&'a str, Counter>,
    start: &'a str,
    end: &'a str,
) -> HashMap<&'a str, Counter> {
    if !cache.contains_key(start) {
        let is_fft = start == "fft";
        let is_dac = start == "dac";

        let mut counter = Counter {
            outs: 0,
            fft: 0,
            dac: 0,
            fft_dac: 0,
        };

        if let Some(nodes) = graph.get(start) {
            for node in nodes {
                if node == &end {
                    counter.outs += 1;
                } else {
                    cache = count_outs(graph, cache, node, end);
                    let child_counter = cache.get(node).unwrap();
                    counter.outs += child_counter.outs;
                    counter.fft += child_counter.fft;
                    counter.dac += child_counter.dac;
                    counter.fft_dac += child_counter.fft_dac;
                }
            }
        }

        if is_fft {
            counter.fft += counter.outs;

            if counter.dac > 0 {
                counter.fft_dac += counter.dac;
            }
        }

        if is_dac {
            counter.dac += counter.outs;

            if counter.fft > 0 {
                counter.fft_dac += counter.fft;
            }
        }

        cache.insert(start, counter);
    }

    cache
}

fn part1(graph: &HashMap<&str, Vec<&str>>) -> u64 {
    let mut queue: Vec<&str> = Vec::new();
    queue.push("you");

    let mut out_count = 0;

    while let Some(node) = queue.pop() {
        if node == "out" {
            out_count += 1;
            continue;
        }

        if let Some(next_nodes) = graph.get(node) {
            for next_node in next_nodes {
                queue.push(next_node);
            }
        }
    }

    out_count
}

fn part2(graph: &Graph, rgraph: &Graph) -> u64 {
    let mut queue: Vec<&str> = Vec::new();
    queue.push("out");

    let mut cache: HashMap<&str, Counter> = HashMap::new();

    while let Some(node) = queue.pop() {
        cache = count_outs(graph, cache, node, "out");
        if let Some(next_nodes) = rgraph.get(node) {
            let new_nodes = next_nodes.iter().filter(|n| !cache.contains_key(*n));
            queue.extend(new_nodes);
        }
    }

    cache.get("svr").unwrap().fft_dac
}

fn main() {
    let input = include_str!("../day11.txt");

    let graph = parse_input(input);
    let rgraph = reverse_graph(&graph);

    println!("Day 11 Part 1: {}", part1(&graph));
    println!("Day 11 Part 2: {:?}", part2(&graph, &rgraph));
}
