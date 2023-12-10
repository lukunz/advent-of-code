use std::cmp::min;
use std::collections::HashMap;
use std::fs;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Tile {
    NToS,
    EToW,
    NToE,
    NToW,
    SToW,
    SToE,
    Ground,
    Start,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Tile {
    fn from_char(c: char) -> Result<Self, String> {
        match c {
            '|' => Ok(Self::NToS),
            '-' => Ok(Self::EToW),
            'L' => Ok(Self::NToE),
            'J' => Ok(Self::NToW),
            '7' => Ok(Self::SToW),
            'F' => Ok(Self::SToE),
            '.' => Ok(Self::Ground),
            'S' => Ok(Self::Start),
            _ => Err(String::from("Unknown tile")),
        }
    }

    fn has_north(&self) -> bool {
        self.walk(&Direction::North).is_some()
    }

    fn has_east(&self) -> bool {
        self.walk(&Direction::East).is_some()
    }

    fn has_south(&self) -> bool {
        self.walk(&Direction::South).is_some()
    }

    fn has_west(&self) -> bool {
        self.walk(&Direction::West).is_some()
    }

    fn walk(&self, dir: &Direction) -> Option<Direction> {
        match dir {
            Direction::North => match self {
                Self::NToS => Some(Direction::South),
                Self::NToE => Some(Direction::East),
                Self::NToW => Some(Direction::West),
                _ => None,
            },
            Direction::East => match self {
                Self::EToW => Some(Direction::West),
                Self::NToE => Some(Direction::North),
                Self::SToE => Some(Direction::South),
                _ => None,
            },
            Direction::South => match self {
                Self::NToS => Some(Direction::North),
                Self::SToW => Some(Direction::West),
                Self::SToE => Some(Direction::East),
                _ => None,
            },
            Direction::West => match self {
                Self::EToW => Some(Direction::East),
                Self::NToW => Some(Direction::North),
                Self::SToW => Some(Direction::South),
                _ => None,
            },
        }
    }
}

struct Map {
    map: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

type Point = (usize, usize);

impl Map {
    fn get(&self, point: &Point) -> Tile {
        self.map[point.1][point.0]
    }

    fn walk(&self, start: &Point, dir: &Direction) -> Option<(Point, Direction)> {
        let new_dir = self.get(start).walk(dir)?;
        match new_dir {
            Direction::North => {
                if start.1 == 0 {
                    None
                } else {
                    let dest = (start.0, start.1 - 1);

                    Some((dest, Direction::South))
                }
            }
            Direction::East => {
                if start.0 + 1 == self.width {
                    None
                } else {
                    let dest = (start.0 + 1, start.1);

                    Some((dest, Direction::West))
                }
            }
            Direction::South => {
                if start.1 + 1 == self.height {
                    None
                } else {
                    let dest = (start.0, start.1 + 1);

                    Some((dest, Direction::North))
                }
            }
            Direction::West => {
                if start.0 == 0 {
                    None
                } else {
                    let dest = (start.0 - 1, start.1);

                    Some((dest, Direction::East))
                }
            }
        }
    }
}

fn parse_map(data: &str) -> Result<Map, String> {
    let map: Vec<Vec<Tile>> = data
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| Tile::from_char(c).ok())
                .collect()
        })
        .collect();

    let height = map.len();
    let width = map[0].len();

    Ok(Map { map, width, height })
}

fn find_start(map: &Map) -> Option<(usize, usize)> {
    for (y, row) in map.map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if *tile == Tile::Start {
                return Some((x, y));
            }
        }
    }

    None
}

fn find_starting_directions(start: &Point, map: &Map) -> Vec<(Point, Direction)> {
    let mut result: Vec<(Point, Direction)> = Vec::new();

    // check north
    if start.1 > 0 && map.get(&(start.0, start.1 - 1)).has_south() {
        result.push(((start.0, start.1 - 1), Direction::South));
    }

    // check east
    if start.0 + 1 < map.width && map.get(&(start.0 + 1, start.1)).has_west() {
        result.push(((start.0 + 1, start.1), Direction::West));
    }

    // check south
    if start.1 + 1 < map.height && map.get(&(start.0, start.1 + 1)).has_north() {
        result.push(((start.0, start.1 + 1), Direction::North));
    }

    // check west
    if start.0 > 0 && map.get(&(start.0 - 1, start.1)).has_east() {
        result.push(((start.0 - 1, start.1), Direction::East));
    }

    result
}

fn trace_walk(start: Point, entering_direction: Direction, map: &Map) -> HashMap<Point, usize> {
    let mut curent_location = start;
    let mut current_direction = entering_direction;
    let mut trace: HashMap<Point, usize> = HashMap::new();
    let mut distance = 1;

    while map.get(&curent_location) != Tile::Start {
        trace.insert(curent_location, distance);
        let (location, direction) = map.walk(&curent_location, &current_direction).unwrap();
        distance += 1;
        curent_location = location;
        current_direction = direction;
    }

    trace
}

fn main() -> Result<(), String> {
    let data = fs::read_to_string("day10.txt").expect("Can't read input file");
    let map = parse_map(&data)?;
    let start = find_start(&map).unwrap();
    let starting_directions = find_starting_directions(&start, &map);
    let traces: Vec<HashMap<Point, usize>> = starting_directions
        .iter()
        .copied()
        .map(|(p, d)| trace_walk(p, d, &map))
        .collect();

    let mut merged_trace: HashMap<Point, usize> = HashMap::new();

    for trace in traces {
        for (point, distance) in trace {
            if let Some(d) = merged_trace.get(&point) {
                merged_trace.insert(point, min(distance, *d));
            } else {
                merged_trace.insert(point, distance);
            }
        }
    }

    let part1_result = merged_trace.values().max().unwrap();

    println!("Day 10 Part 1: {}", part1_result);

    Ok(())
}
