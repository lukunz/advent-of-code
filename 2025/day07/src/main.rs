use std::collections::{HashMap, HashSet, VecDeque};

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
    width: usize,
    height: usize,
}

impl Map {
    fn new(tiles: Vec<Vec<Tile>>, start: Point) -> Self {
        let width = tiles.len();
        let height = tiles[0].len();

        Self {
            tiles,
            start,
            width,
            height,
        }
    }

    fn contains(&self, point: &Point) -> bool {
        self.width > point.x && self.height > point.y
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: usize,
    y: usize,
}

fn parse_input(input: &str) -> Map {
    let tiles: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| line.chars().map(Tile::from_char).collect::<Vec<Tile>>())
        .collect();

    let mut start: Option<Point> = None;

    'outer: for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                start = Some(Point { x, y });
                break 'outer;
            }
        }
    }

    Map::new(tiles, start.unwrap())
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

        match map.tiles[point.y][point.x] {
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

fn collect_splits(map: &Map) -> Vec<Point> {
    let mut splits = Vec::new();

    for (y, row) in map.tiles.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if tile == &Tile::Splitter {
                splits.push(Point { x, y });
            }
        }
    }

    splits
}

fn count_splits(map: &Map, counted_splits: &HashMap<Point, u64>, split: &Point) -> u64 {
    let mut split_count = 0;

    'outer: for x in [split.x - 1, split.x + 1] {
        for y in split.y..map.height {
            let point = Point { x, y };
            if let Some(sub_count) = counted_splits.get(&point) {
                split_count += sub_count;
                continue 'outer;
            }
        }
        split_count += 1;
    }

    split_count
}

fn part2(map: &Map) -> u64 {
    let mut splits = collect_splits(map);
    let mut counted_splits: HashMap<Point, u64> = HashMap::new();

    while let Some(split) = splits.pop() {
        let count = count_splits(map, &counted_splits, &split);
        counted_splits.insert(split, count);
    }

    for y in map.start.y..map.height {
        if map.tiles[y][map.start.x] == Tile::Splitter {
            return *counted_splits.get(&Point { x: map.start.x, y }).unwrap();
        }
    }

    1
}

fn main() {
    let input = include_str!("../day07.txt");

    let map = parse_input(input);

    let part1_result = part1(&map);
    println!("Day 07 Part 1: {}", part1_result);

    let part2_result = part2(&map);
    println!("Day 07 Part 2: {}", part2_result);
}
