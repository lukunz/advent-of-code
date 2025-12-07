use std::collections::{HashSet, VecDeque};

#[derive(PartialEq, Clone, Debug)]
enum Tile {
    Empty,
    Splitter,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' | 'S' => Self::Empty,
            '^' => Self::Splitter,
            _ => unreachable!(),
        }
    }
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    start: Point,
}

impl Map {
    fn contains(&self, point: &Point) -> bool {
        self.tiles.len() > point.x && self.tiles[0].len() > point.y
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: usize,
    y: usize,
}

fn parse_input(input: &str) -> Map {
    let data: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let mut tiles: Vec<Vec<Tile>> = Vec::with_capacity(data[0].len());
    for _ in 0..data[0].len() {
        tiles.push(Vec::new());
    }

    let mut start: Option<Point> = None;

    for (y, row) in data.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == 'S' {
                start = Some(Point { x, y });
                tiles[x].push(Tile::Empty);
            } else {
                tiles[x].push(Tile::from_char(c));
            }
        }
    }

    Map {
        tiles,
        start: start.unwrap(),
    }
}

fn part1(map: &Map) -> u64 {
    let mut next: VecDeque<Point> = VecDeque::new();
    let mut visited: HashSet<Point> = HashSet::new();
    let mut split_count = 0;

    next.push_back(map.start.clone());

    while let Some(point) = next.pop_front() {
        if visited.contains(&point) {
            continue;
        }

        if !map.contains(&point) {
            continue;
        }

        visited.insert(point.clone());

        match map.tiles[point.x][point.y] {
            Tile::Empty => {
                next.push_back(Point {
                    x: point.x,
                    y: point.y + 1,
                });
            }
            Tile::Splitter => {
                split_count += 1;
                if point.x > 0 {
                    next.push_back(Point {
                        x: point.x - 1,
                        y: point.y,
                    });
                }
                next.push_back(Point {
                    x: point.x + 1,
                    y: point.y,
                });
            }
        }
    }

    split_count
}

fn main() {
    let input = include_str!("../day07.txt");

    let map = parse_input(input);

    let part1_result = part1(&map);

    println!("Day 07 Part 1: {}", part1_result);
}
