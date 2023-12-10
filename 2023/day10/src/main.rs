use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
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

    fn to_char(self) -> char {
        match self {
            Tile::NToS => '|',
            Tile::EToW => '-',
            Tile::NToE => 'L',
            Tile::NToW => 'J',
            Tile::SToW => '7',
            Tile::SToE => 'F',
            Tile::Ground => '.',
            Tile::Start => 'S',
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

    fn sweep(&mut self, keep: &HashSet<Point>) {
        for y in 0..self.height {
            for x in 0..self.width {
                if !keep.contains(&(x, y)) {
                    self.map[y][x] = Tile::Ground;
                }
            }
        }
    }

    fn expand(&self) -> Map {
        let width = self.width * 3;
        let height = self.height * 3;
        let mut map: Vec<Vec<Tile>> = Vec::new();

        for _ in 0..height {
            map.push(vec![Tile::Ground; width]);
        }

        for (y, row) in self.map.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                let x = x * 3 + 1;
                let y = y * 3 + 1;

                map[y][x] = *tile;
                map[y - 1][x - 1] = Tile::Ground;
                map[y - 1][x + 1] = Tile::Ground;
                map[y + 1][x - 1] = Tile::Ground;
                map[y + 1][x + 1] = Tile::Ground;

                match tile {
                    Tile::NToS => {
                        map[y][x - 1] = Tile::Ground;
                        map[y][x + 1] = Tile::Ground;
                        map[y - 1][x] = Tile::NToS;
                        map[y + 1][x] = Tile::NToS;
                    }
                    Tile::EToW => {
                        map[y][x - 1] = Tile::EToW;
                        map[y][x + 1] = Tile::EToW;
                        map[y - 1][x] = Tile::Ground;
                        map[y + 1][x] = Tile::Ground;
                    }
                    Tile::NToE => {
                        map[y][x - 1] = Tile::Ground;
                        map[y][x + 1] = Tile::EToW;
                        map[y - 1][x] = Tile::NToS;
                        map[y + 1][x] = Tile::Ground;
                    }
                    Tile::NToW => {
                        map[y][x - 1] = Tile::EToW;
                        map[y][x + 1] = Tile::Ground;
                        map[y - 1][x] = Tile::NToS;
                        map[y + 1][x] = Tile::Ground;
                    }
                    Tile::SToW => {
                        map[y][x - 1] = Tile::EToW;
                        map[y][x + 1] = Tile::Ground;
                        map[y - 1][x] = Tile::Ground;
                        map[y + 1][x] = Tile::NToS;
                    }
                    Tile::SToE => {
                        map[y][x - 1] = Tile::Ground;
                        map[y][x + 1] = Tile::EToW;
                        map[y - 1][x] = Tile::Ground;
                        map[y + 1][x] = Tile::NToS;
                    }
                    Tile::Ground => {
                        map[y][x - 1] = Tile::Ground;
                        map[y][x + 1] = Tile::Ground;
                        map[y - 1][x] = Tile::Ground;
                        map[y + 1][x] = Tile::Ground;
                    }
                    Tile::Start => {
                        let starting_directions =
                            find_starting_directions(&((x - 1) / 3, (y - 1) / 3), self);
                        map[y][x - 1] = Tile::Ground;
                        map[y][x + 1] = Tile::Ground;
                        map[y - 1][x] = Tile::Ground;
                        map[y + 1][x] = Tile::Ground;
                        for (_, dir) in starting_directions {
                            match dir {
                                Direction::East => map[y][x - 1] = Tile::EToW,
                                Direction::West => map[y][x + 1] = Tile::EToW,
                                Direction::South => map[y - 1][x] = Tile::NToS,
                                Direction::North => map[y + 1][x] = Tile::NToS,
                            }
                        }
                    }
                }
            }
        }

        Map { map, width, height }
    }

    fn fill(&self) -> HashSet<Point> {
        let mut filled_points: HashSet<Point> = HashSet::new();
        let mut queue: Vec<Point> = vec![(0, 0)];

        while let Some(point) = queue.pop() {
            if self.get(&point) == Tile::Ground {
                filled_points.insert(point);
                let (x, y) = point;
                if x > 0 {
                    let p = (x - 1, y);
                    if !filled_points.contains(&p) {
                        queue.push(p);
                    }
                }

                if x < self.width - 1 {
                    let p = (x + 1, y);
                    if !filled_points.contains(&p) {
                        queue.push(p);
                    }
                }

                if y > 0 {
                    let p = (x, y - 1);
                    if !filled_points.contains(&p) {
                        queue.push(p);
                    }
                }

                if y < self.height - 1 {
                    let p = (x, y + 1);
                    if !filled_points.contains(&p) {
                        queue.push(p);
                    }
                }
            }
        }

        filled_points
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let result: Vec<String> = self
            .map
            .iter()
            .map(|row| String::from_iter(row.iter().map(|tile| tile.to_char())))
            .collect();

        write!(f, "{}", result.join("\n"))
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
    let mut current_location = start;
    let mut current_direction = entering_direction;
    let mut trace: HashMap<Point, usize> = HashMap::new();
    let mut distance = 1;

    while map.get(&current_location) != Tile::Start {
        trace.insert(current_location, distance);
        let (location, direction) = map.walk(&current_location, &current_direction).unwrap();
        distance += 1;
        current_location = location;
        current_direction = direction;
    }

    trace
}

fn main() -> Result<(), String> {
    let data = fs::read_to_string("day10.txt").expect("Can't read input file");
    let mut map = parse_map(&data)?;
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

    let mut loop_track: HashSet<Point> = merged_trace.keys().copied().collect();
    loop_track.insert(start);

    map.sweep(&loop_track);
    let expanded_map = map.expand();
    let filled_points = expanded_map.fill();

    let filled_count: Vec<Point> = filled_points
        .iter()
        .filter_map(|(x, y)| {
            if x > &0 && (x - 1) % 3 == 0 && y > &0 && (y - 1) % 3 == 0 {
                Some(((x - 1) / 3, (y - 1) / 3))
            } else {
                None
            }
        })
        .collect();

    let part2_result = (map.width * map.height) - filled_count.len() - loop_track.len();

    println!("Day 10 Part 2: {}", part2_result);

    Ok(())
}
