use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap};

#[derive(Clone, PartialEq)]
enum Tile {
    Empty,
    Blocked,
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    width: i32,
    height: i32,
}

impl Map {
    fn new(blocks: &[(i32, i32)], width: usize, height: usize) -> Self {
        let mut tiles = vec![vec![Tile::Empty; width]; height];

        for (x, y) in blocks.iter().cloned() {
            assert!(
                x >= 0 && y >= 0 && (x as usize) < width && (y as usize) < height,
                "Block is out of bounds"
            );
            tiles[y as usize][x as usize] = Tile::Blocked;
        }

        Self {
            tiles,
            width: width as i32,
            height: height as i32,
        }
    }

    fn neighbors(&self, (x, y): (i32, i32)) -> Vec<(i32, i32)> {
        let mut neighbors = Vec::with_capacity(4);

        for (delta_x, delta_y) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (x, y) = (x + delta_x, y + delta_y);

            if x >= 0
                && x < self.width
                && y >= 0
                && y < self.height
                && self.tiles[y as usize][x as usize] == Tile::Empty
            {
                neighbors.push((x, y));
            }
        }

        neighbors
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Node {
    position: (i32, i32),
    cost: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let data = include_str!("../day18.txt");
    let map_width = 71;
    let map_height = 71;
    let number_of_blocks = 1024;

    let blocks = parse_input(data);
    let map = Map::new(&blocks[..number_of_blocks], map_width, map_height);
    let graph = build_graph(&map);

    if let Some(path) = find_shortest_path((0, 0), (map.width - 1, map.height - 1), &graph) {
        println!("Day 18 Part 1: {}", path.len() - 1);
    }
}

fn build_graph(map: &Map) -> BTreeMap<(i32, i32), Vec<(i32, i32)>> {
    let mut graph = BTreeMap::new();

    for (y, row) in map.tiles.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if tile == &Tile::Empty {
                let position = (x as i32, y as i32);
                graph.insert(position, map.neighbors(position));
            }
        }
    }

    graph
}

fn heuristic(start: (i32, i32), goal: (i32, i32)) -> i32 {
    (start.0 - goal.0).abs() + (start.1 - goal.1).abs()
}

fn find_shortest_path(
    start: (i32, i32),
    goal: (i32, i32),
    graph: &BTreeMap<(i32, i32), Vec<(i32, i32)>>,
) -> Option<Vec<(i32, i32)>> {
    let mut nodes_to_check = BinaryHeap::new();
    let mut costs = BTreeMap::new();
    let mut prev = BTreeMap::new();

    costs.insert(start, 0);
    nodes_to_check.push(Node {
        position: start,
        cost: heuristic(start, goal),
    });

    while let Some(Node { position, .. }) = nodes_to_check.pop() {
        if position == goal {
            let mut position = position;
            let mut path = vec![position];
            while let Some(&prev) = prev.get(&position) {
                position = prev;
                path.push(position);
            }
            path.reverse();

            return Some(path);
        }

        if let Some(neighbors) = graph.get(&position) {
            let cost = costs[&position] + 1;
            for &neighbor in neighbors {
                if cost < *costs.get(&neighbor).unwrap_or(&i32::MAX) {
                    costs.insert(neighbor, cost);
                    prev.insert(neighbor, position);
                    nodes_to_check.push(Node {
                        position: neighbor,
                        cost: cost + heuristic(neighbor, goal),
                    });
                }
            }
        }
    }

    None
}

fn parse_input(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .map(|line| line.split_once(",").expect("Line in wrong format"))
        .map(|(x, y)| {
            (
                x.parse().expect("Can't parse x as integer"),
                y.parse().expect("Can't parse y as integer"),
            )
        })
        .collect()
}
