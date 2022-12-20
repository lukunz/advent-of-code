use std::collections::{BTreeMap, BTreeSet};
use std::fs;

#[derive(PartialEq, Debug)]
struct Node {
    id: u32,
    flow_rate: u32,
    neighbours: BTreeSet<u32>,
}

fn parse_flow_rate(line_segment: &str) -> u32 {
    let (_, rate_string) = line_segment.split_once('=').expect("Invalid input");

    rate_string
        .trim_matches(';')
        .parse()
        .expect("Invalid input")
}

fn parse_neighbors(line_segments: &[&str]) -> BTreeSet<u32> {
    line_segments
        .iter()
        .map(|segment| parse_node_id(segment.trim_matches(',')))
        .collect()
}

fn parse_node_id(id: &str) -> u32 {
    u32::from_str_radix(id, 36).expect("Invalid input")
}

fn fmt_node_id(id: u32) -> String {
    let first_char = id / 36;
    let second_char = id - first_char * 36;

    let id_str = vec![
        char::from_digit(first_char, 36)
            .unwrap()
            .to_ascii_uppercase(),
        char::from_digit(second_char, 36)
            .unwrap()
            .to_ascii_uppercase(),
    ];

    id_str.iter().collect()
}

fn parse_line(line: &str) -> Node {
    let line_segments: Vec<&str> = line.split_whitespace().collect();

    if line_segments.len() < 10 {
        panic!("Invalid input");
    }

    Node {
        id: parse_node_id(line_segments[1]),
        flow_rate: parse_flow_rate(line_segments[4]),
        neighbours: parse_neighbors(&line_segments[9..]),
    }
}

fn parse_file(file: &str) -> Vec<Node> {
    let data = fs::read_to_string(file).expect("Can't read input file");

    data.lines().map(parse_line).collect()
}

#[derive(Debug)]
struct Graph {
    nodes: BTreeMap<u32, Node>,
    edges: BTreeMap<(u32, u32), u32>,
}

impl Graph {
    fn from_file(file: &str) -> Self {
        let node_list = parse_file(file);

        Self::from_nodes(node_list)
    }

    fn from_nodes(node_list: Vec<Node>) -> Self {
        let (nodes, edges) = Self::build_graph(node_list);

        Self { nodes, edges }
    }

    fn build_graph(node_list: Vec<Node>) -> (BTreeMap<u32, Node>, BTreeMap<(u32, u32), u32>) {
        let mut nodes = BTreeMap::new();
        let mut edges = BTreeMap::new();

        for node in node_list {
            for neighbour_id in &node.neighbours {
                edges.insert(Self::edge_id(node.id, *neighbour_id), 1);
            }

            nodes.insert(node.id, node);
        }

        (nodes, edges)
    }

    fn edge_id(node_a: u32, node_b: u32) -> (u32, u32) {
        if node_a < node_b {
            (node_a, node_b)
        } else {
            (node_b, node_a)
        }
    }

    fn to_dot(&self) -> String {
        let mut lines = Vec::new();

        lines.push(String::from("graph {"));

        for (_, node) in &self.nodes {
            lines.push(format!(
                "  {0} [ label = \"{0} {1}\" ];",
                fmt_node_id(node.id),
                node.flow_rate
            ));
        }

        lines.push(String::new());

        for ((node_a_id, node_b_id), cost) in &self.edges {
            lines.push(format!(
                "  {} -- {} [ label = \"{}\" ];",
                fmt_node_id(*node_a_id),
                fmt_node_id(*node_b_id),
                cost
            ));
        }

        lines.push(String::from("}"));

        lines.join("\n")
    }

    fn simplify(&mut self) {
        let root_id = parse_node_id("AA");
        let removable_node_ids: Vec<u32> = self
            .nodes
            .iter()
            .filter_map(|(_, node)| {
                if node.flow_rate == 0 && node.id != root_id {
                    Some(node.id)
                } else {
                    None
                }
            })
            .collect();

        for node_id in removable_node_ids {
            let node = self.nodes.remove(&node_id).unwrap();

            for neighbour_id in &node.neighbours {
                self.nodes
                    .get_mut(neighbour_id)
                    .unwrap()
                    .neighbours
                    .remove(&node_id);

                for new_neighbour_id in &node.neighbours {
                    if neighbour_id != new_neighbour_id {
                        self.nodes
                            .get_mut(neighbour_id)
                            .unwrap()
                            .neighbours
                            .insert(*new_neighbour_id);
                    }
                }

                if let Some(cost) = self.edges.remove(&Self::edge_id(*neighbour_id, node_id)) {
                    for new_neighbour_id in &node.neighbours {
                        if neighbour_id != new_neighbour_id {
                            let edge_id = Self::edge_id(*neighbour_id, *new_neighbour_id);
                            if self.edges.contains_key(&edge_id) {
                                self.edges.insert(edge_id, cost + self.edges[&edge_id] - 1);
                            } else {
                                self.edges.insert(edge_id, cost + 1);
                            }
                        }
                    }
                }
            }
        }
    }

    fn distance(&self, node_a_id: u32, node_b_id: u32) -> u32 {
        let mut queue = BTreeSet::new();
        let mut distances = BTreeMap::new();

        for node_id in self.nodes.keys() {
            distances.insert(node_id, u32::MAX);
            queue.insert(node_id);
        }

        distances.insert(&node_a_id, 0);

        while !queue.is_empty() {
            let node_id = queue
                .iter()
                .map(|node_id| (node_id, distances[node_id]))
                .reduce(|(nearest_node_id, nearest_distance), (node_id, distance)| {
                    if distance < nearest_distance {
                        (node_id, distance)
                    } else {
                        (nearest_node_id, nearest_distance)
                    }
                })
                .unwrap()
                .0
                .clone();

            if *node_id == node_b_id {
                return distances[node_id];
            }

            queue.remove(node_id);

            let to_visit: Vec<&u32> = self
                .nodes
                .get(node_id)
                .unwrap()
                .neighbours
                .iter()
                .filter(|neighbour_id| queue.contains(neighbour_id))
                .collect();

            for neighbour_id in to_visit {
                let new_distance =
                    distances[node_id] + self.edges[&Self::edge_id(*node_id, *neighbour_id)];

                if new_distance < distances[&neighbour_id] {
                    distances.insert(&neighbour_id, new_distance);
                }
            }
        }

        u32::MAX
    }
}

#[derive(Clone, Debug)]
struct GameState<'a> {
    rounds_left: u32,
    visited_nodes: BTreeMap<u32, bool>, // Node -> opened
    released_pressure: u32,
    graph: &'a Graph,
}

impl<'a> GameState<'a> {
    fn from(graph: &'a Graph) -> Self {
        GameState {
            rounds_left: 30,
            visited_nodes: BTreeMap::new(),
            released_pressure: 0,
            graph,
        }
    }

    fn current_flow_rate(&self) -> u32 {
        self.visited_nodes
            .iter()
            .map(|(node_id, opened)| {
                if *opened {
                    self.graph.nodes[node_id].flow_rate
                } else {
                    0
                }
            })
            .sum::<u32>()
    }

    fn tick(&mut self, rounds: u32) {
        self.rounds_left -= rounds;
        self.released_pressure += self.current_flow_rate() * rounds;
    }

    fn untick(&mut self, rounds: u32) {
        self.rounds_left += rounds;
        self.released_pressure -= self.current_flow_rate() * rounds;
    }

    fn score(&self) -> u32 {
        self.released_pressure + self.rounds_left * self.current_flow_rate()
    }
}

fn walk<'a>(
    node_id: u32,
    distances: &'a BTreeMap<(u32, u32), u32>,
    game_state: GameState<'a>,
    path: Vec<String>,
) -> GameState<'a> {
    let mut path = path;
    path.push(fmt_node_id(node_id));
    if game_state.rounds_left == 0 {
        // println!("{} times up", path.join(" -> "));
        return game_state;
    }

    let best_game_state = game_state.clone();
    let node = game_state.graph.nodes.get(&node_id).unwrap();

    let neighbour_ids: Vec<u32> = game_state
        .graph
        .nodes
        .iter()
        .filter(|(id, _)| {
            *id != &node_id
                && (!game_state.visited_nodes.contains_key(id))
        })
        .map(|(id, _)| *id)
        .collect();

    let mut right_game_state = game_state.clone();
    if node_id != parse_node_id("AA") {
        right_game_state.tick(1);
    }
    right_game_state.visited_nodes.insert(node_id, true);

    let best_game_state = walk_neighbours(
        node_id,
        distances,
        best_game_state,
        &neighbour_ids,
        right_game_state,
        &mut path,
    );

    // println!("{} graph done {} - {} - {}", path.join(" -> "), best_game_state.score(), best_game_state.current_flow_rate(), best_game_state.rounds_left);
    best_game_state
}

fn walk_neighbours<'a, 'b>(
    node_id: u32,
    distances: &'a BTreeMap<(u32, u32), u32>,
    best_game_state: GameState<'a>,
    neighbour_ids: &'b Vec<u32>,
    game_state: GameState<'a>,
    path: &mut Vec<String>,
) -> GameState<'a> {
    let mut new_best_game_state = if game_state.score() >= best_game_state.score() {
        game_state.clone()
    } else {
        best_game_state.clone()
    };

    for neighbour_id in neighbour_ids {
        let distance = distances[&Graph::edge_id(node_id, *neighbour_id)];
        if distance <= game_state.rounds_left {
            let mut new_game_state = game_state.clone();
            new_game_state.tick(distance);

            let new_game_state = walk(*neighbour_id, distances, new_game_state, path.clone());

            if new_game_state.score() > new_best_game_state.score() {
                new_best_game_state = new_game_state;
            }
        }
    }

    new_best_game_state
}

fn part_one() {
    let mut graph = Graph::from_file("day16/input.txt");
    graph.simplify();

    let distances = calculate_distances(&mut graph);

    let game_state = walk(parse_node_id("AA"), &distances, GameState::from(&graph), Vec::new());

    println!("{:?}", game_state.score());

    // println!("{}", graph.to_dot());
}

fn calculate_distances(mut graph: &mut Graph) -> BTreeMap<(u32, u32), u32> {
    let node_ids: Vec<u32> = graph.nodes.keys().map(|x| x.to_owned()).collect();
    let mut distances = BTreeMap::new();

    for i in 0..node_ids.len() {
        for j in i + 1..node_ids.len() {
            distances.insert(
                Graph::edge_id(node_ids[i], node_ids[j]),
                graph.distance(node_ids[i], node_ids[j]),
            );
        }
    }
    distances
}

fn main() {
    part_one();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_line() {
        let line = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB";
        let node_data = parse_line(line);
        let expected_node_data = Node {
            id: 370,
            flow_rate: 0,
            neighbours: BTreeSet::from_iter(vec![481, 666, 407].into_iter()),
        };

        assert_eq!(expected_node_data, node_data);
    }

    #[test]
    fn test_simplify() {
        let mut graph = Graph::from_file("input-small.txt");
        graph.simplify();

        for (_, node) in &graph.nodes {
            for neighbour_id in &node.neighbours {
                assert!(graph
                    .edges
                    .contains_key(&Graph::edge_id(node.id, *neighbour_id)));
            }
        }

        for (left, right) in graph.edges.keys() {
            assert!(graph.nodes.contains_key(left));
            assert!(graph.nodes.contains_key(right));
        }
    }

    #[test]
    fn test_distance_small() {
        let mut graph = Graph::from_file("input-small.txt");
        graph.simplify();

        assert_eq!(0, graph.distance(parse_node_id("AA"), parse_node_id("AA")));
        assert_eq!(2, graph.distance(parse_node_id("AA"), parse_node_id("JJ")));
        assert_eq!(5, graph.distance(parse_node_id("AA"), parse_node_id("HH")));
    }

    #[test]
    fn test_distance() {
        let mut graph = Graph::from_file("input.txt");
        graph.simplify();

        assert_eq!(0, graph.distance(parse_node_id("AA"), parse_node_id("AA")));
        assert_eq!(2, graph.distance(parse_node_id("AA"), parse_node_id("IF")));
        assert_eq!(3, graph.distance(parse_node_id("AA"), parse_node_id("MH")));
        assert_eq!(5, graph.distance(parse_node_id("AA"), parse_node_id("MU")));
    }

    #[test]
    fn test_walk() {
        let lines = vec![
            "Valve AA has flow rate=0; tunnels lead to valves BB",
            "Valve BB has flow rate=13; tunnels lead to valves AA, CC",
            "Valve CC has flow rate=2; tunnels lead to valves BB",
        ];
        let node_list: Vec<Node> = lines.into_iter().map(parse_line).collect();
        let mut graph = Graph::from_nodes(node_list);

        let distances = calculate_distances(&mut graph);

        let game_state = walk(parse_node_id("AA"), &distances, GameState::from(&graph), Vec::new());

        assert_eq!(28 * 13 + 26 * 2, game_state.score());
    }
}
