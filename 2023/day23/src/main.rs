use std::collections::HashSet;
use std::fs;

#[derive(PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            '^' => Self::North,
            '>' => Self::East,
            'v' => Self::South,
            '<' => Self::West,
            _ => panic!("Unknown direction '{}'", c),
        }
    }

    fn invert(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

#[derive(PartialEq)]
enum Tile {
    Path,
    Rock,
    Slope(Direction),
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Path,
            '#' => Self::Rock,
            d => Self::Slope(Direction::from_char(d)),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn from_xy(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Clone)]
struct Path {
    location: Point,
    visited: HashSet<Point>,
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl Map {
    fn find_start(&self) -> Point {
        let x = self.tiles[0]
            .iter()
            .position(|tile| tile == &Tile::Path)
            .unwrap();

        Point::from_xy(x, 0)
    }

    fn find_finish(&self) -> Point {
        let x = self.tiles[self.height - 1]
            .iter()
            .position(|tile| tile == &Tile::Path)
            .unwrap();

        Point::from_xy(x, self.height - 1)
    }

    fn get(&self, point: &Point) -> &Tile {
        &self.tiles[point.y][point.x]
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
        }?;

        let try_go = |point: Point| match self.get(&point) {
            Tile::Rock => None,
            Tile::Slope(d) => {
                if d != &direction.invert() {
                    Some(point)
                } else {
                    None
                }
            }
            _ => Some(point),
        };

        match self.get(point) {
            Tile::Path => try_go(new_point),
            Tile::Rock => None,
            Tile::Slope(d) => {
                if d == direction {
                    try_go(new_point)
                } else {
                    None
                }
            }
        }
    }

    fn find_paths(&self) -> Vec<Path> {
        let mut paths: Vec<Path> = Vec::new();
        let destination = self.find_finish();

        let mut queue = vec![Path {
            visited: HashSet::new(),
            location: self.find_start(),
        }];

        while let Some(path) = queue.pop() {
            if path.location == destination {
                paths.push(path);
            } else {
                [
                    Direction::North,
                    Direction::East,
                    Direction::South,
                    Direction::West,
                ]
                .iter()
                .filter_map(|direction| self.go(&path.location, direction))
                .filter(|new_point| !path.visited.contains(new_point))
                .for_each(|new_point| {
                    let mut new_path = path.clone();
                    new_path.visited.insert(new_path.location);
                    new_path.location = new_point;
                    queue.push(new_path);
                });
            }
        }

        paths
    }
}

fn parse_input(data: &str) -> Map {
    let tiles: Vec<Vec<Tile>> = data
        .lines()
        .map(|line| line.chars().map(Tile::from_char).collect())
        .collect();
    let width = tiles[0].len();
    let height = tiles.len();

    Map {
        tiles,
        width,
        height,
    }
}

fn main() {
    let data = fs::read_to_string("day23.txt").expect("Can't read input file");
    let map = parse_input(&data);
    let paths = map.find_paths();

    let part1_result: usize = paths.iter().map(|path| path.visited.len()).max().unwrap();

    println!("Day 23 Part 1: {}", part1_result);
}
