use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap};

#[derive(PartialEq)]
enum Tile {
    Wall,
    Empty,
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn is_rev(&self, other: &Direction) -> bool {
        match self {
            Self::North => *other == Self::South,
            Self::East => *other == Self::West,
            Self::South => *other == Self::North,
            Self::West => *other == Self::East,
        }
    }

    fn is_turn(&self, other: &Direction) -> bool {
        match other {
            Direction::North => *self == Direction::East || *self == Direction::West,
            Direction::East => *self == Direction::North || *self == Direction::South,
            Direction::South => *self == Direction::East || *self == Direction::West,
            Direction::West => *self == Direction::South || *self == Direction::North,
        }
    }
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn step(&self, dir: &Direction) -> Self {
        match dir {
            Direction::North => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::East => Position {
                x: self.x + 1,
                y: self.y,
            },
            Direction::South => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::West => Position {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
    start: Position,
    end: Position,
}

impl Map {
    fn from_str(input: &str) -> Self {
        let tiles = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '#' => Tile::Wall,
                        '.' | 'S' | 'E' => Tile::Empty,
                        _ => panic!("Unrecognized tile: {}", c),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let width = tiles[0].len();
        let height = tiles.len();

        let start_position = input.find("S").expect("No start tile found");
        let start_y = start_position / (width + 1);
        let start_x = start_position - (width + 1) * start_y;

        let end_position = input.find("E").expect("No end tile found");
        let end_y = end_position / (width + 1);
        let end_x = end_position - (width + 1) * end_y;

        Self {
            tiles,
            width,
            height,
            start: Position {
                x: start_x,
                y: start_y,
            },
            end: Position { x: end_x, y: end_y },
        }
    }
}

struct Node {
    pos: Position,
    edges: Vec<Direction>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.pos.cmp(&other.pos)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Node {}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State<'a> {
    cost: usize,
    node: &'a Node,
    entered_from: Direction,
}

impl Ord for State<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(other.node))
    }
}

impl PartialOrd for State<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let data = include_str!("../day16.txt");

    let map = Map::from_str(data);
    let nodes = find_nodes(&map);
    let mut heap = BinaryHeap::new();

    let mut dist: BTreeMap<&Position, usize> =
        nodes.values().map(|node| (&node.pos, usize::MAX)).collect();

    let start = &nodes[&map.start];

    heap.push(State {
        cost: 0,
        node: start,
        entered_from: Direction::East,
    });

    dist.insert(&map.start, 0);

    while let Some(State {
        cost,
        node,
        entered_from,
    }) = heap.pop()
    {
        if cost > dist[&node.pos] {
            continue;
        }

        for dir in &node.edges {
            if entered_from.is_rev(dir) {
                continue;
            }

            let next = State {
                cost: if entered_from.is_turn(dir) { 1001 } else { 1 } + cost,
                node: &nodes[&node.pos.step(dir)],
                entered_from: *dir,
            };

            if next.cost < dist[&next.node.pos] {
                dist.insert(&next.node.pos, next.cost);
                heap.push(next);
            }
        }
    }

    println!("Day 16 Part 1: {}", dist[&map.end]);
}

fn find_nodes(map: &Map) -> BTreeMap<Position, Node> {
    assert!(map.width > 2 && map.height > 2);

    let mut nodes = BTreeMap::new();

    for y in 1..map.height - 1 {
        for x in 1..map.width - 1 {
            match map.tiles[y][x] {
                Tile::Wall => {}
                Tile::Empty => {
                    let mut edges = Vec::with_capacity(4);

                    if map.tiles[y - 1][x] == Tile::Empty {
                        edges.push(Direction::North);
                    }
                    if map.tiles[y][x + 1] == Tile::Empty {
                        edges.push(Direction::East);
                    }
                    if map.tiles[y + 1][x] == Tile::Empty {
                        edges.push(Direction::South);
                    }
                    if map.tiles[y][x - 1] == Tile::Empty {
                        edges.push(Direction::West);
                    }

                    nodes.insert(
                        Position { x, y },
                        Node {
                            pos: Position { x, y },
                            edges,
                        },
                    );
                }
            }
        }
    }

    nodes
}
