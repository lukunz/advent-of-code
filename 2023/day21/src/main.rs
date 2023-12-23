use std::collections::HashSet;
use std::fs;

#[derive(PartialEq)]
enum Tile {
    GardenPlot,
    Rock,
}

enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn from_xy(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl Map {
    fn get(&self, point: &Point) -> Option<&Tile> {
        if point.x < self.width && point.y < self.height {
            Some(&self.tiles[point.y][point.x])
        } else {
            None
        }
    }

    fn go(&self, point: &Point, direction: &Direction) -> Option<Point> {
        let new_point = match direction {
            Direction::North => {
                if point.y > 0 {
                    Some(Point::from_xy(point.x, point.y - 1))
                } else {
                    None
                }
            }
            Direction::East => {
                if point.x < self.width - 1 {
                    Some(Point::from_xy(point.x + 1, point.y))
                } else {
                    None
                }
            }
            Direction::South => {
                if point.y < self.height - 1 {
                    Some(Point::from_xy(point.x, point.y + 1))
                } else {
                    None
                }
            }
            Direction::West => {
                if point.x > 0 {
                    Some(Point::from_xy(point.x - 1, point.y))
                } else {
                    None
                }
            }
        };

        new_point.filter(|p| self.get(p) == Some(&Tile::GardenPlot))
    }
}

fn parse_input(data: &str) -> (Map, Point) {
    let mut tiles: Vec<Vec<Tile>> = Vec::new();
    let mut start_point = Point { x: 0, y: 0 };

    for (y, line) in data.lines().enumerate() {
        let mut row: Vec<Tile> = Vec::new();
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => row.push(Tile::GardenPlot),
                '#' => row.push(Tile::Rock),
                'S' => {
                    row.push(Tile::GardenPlot);
                    start_point.x = x;
                    start_point.y = y;
                }
                _ => panic!("Unknown tile {}", c),
            }
        }
        tiles.push(row);
    }

    let width = tiles[0].len();
    let height = tiles.len();

    (
        Map {
            tiles,
            width,
            height,
        },
        start_point,
    )
}

fn make_step(start_point: &Point, map: &Map, queue: &mut HashSet<Point>) {
    for direction in [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ] {
        if let Some(point) = map.go(start_point, &direction) {
            queue.insert(point);
        }
    }
}

fn main() {
    let data = fs::read_to_string("day21.txt").expect("Can't read input file");
    let (map, start_point) = parse_input(&data);

    let mut queue: HashSet<Point> = HashSet::new();
    queue.insert(start_point);

    for _ in 0..64 {
        let mut next_queue: HashSet<Point> = HashSet::new();

        for point in queue {
            make_step(&point, &map, &mut next_queue);
        }

        queue = next_queue;
    }

    let part1_result = queue.iter().copied().collect::<HashSet<Point>>().len();

    println!("Day 21 Part 1: {}", part1_result);
}
