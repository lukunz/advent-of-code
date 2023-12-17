use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

#[derive(Clone)]
struct Map {
    tiles: Vec<Vec<u32>>,
    width: usize,
    height: usize,
    start: Point,
    target: Point,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone, Debug)]
struct Path {
    point: Point,
    direction: Direction,
    steps_in_direction: usize,
    path: Vec<Point>,
    distance_to_target: usize,
}

struct Limits {
    min: usize,
    max: usize,
}

impl Map {
    fn from_str(data: &str) -> Map {
        let tiles: Vec<Vec<u32>> = data
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();

        let width = tiles[0].len();
        let height = tiles.len();

        Map {
            tiles,
            width,
            height,
            start: Point::from_xy(0, 0),
            target: Point::from_xy(width - 1, height - 1),
        }
    }

    fn get(&self, point: &Point) -> u32 {
        self.tiles[point.y][point.x]
    }
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }
}

impl Point {
    fn from_xy(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn distance(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    fn move_in_direction(&self, direction: &Direction, map: &Map) -> Option<Self> {
        match direction {
            Direction::North => {
                if self.y == 0 {
                    None
                } else {
                    Some(Self::from_xy(self.x, self.y - 1))
                }
            }
            Direction::East => {
                if self.x >= map.width - 1 {
                    None
                } else {
                    Some(Self::from_xy(self.x + 1, self.y))
                }
            }
            Direction::South => {
                if self.y >= map.height - 1 {
                    None
                } else {
                    Some(Self::from_xy(self.x, self.y + 1))
                }
            }
            Direction::West => {
                if self.x == 0 {
                    None
                } else {
                    Some(Self::from_xy(self.x - 1, self.y))
                }
            }
        }
    }
}

impl Path {
    fn from_point_direction(point: Point, direction: Direction, target: &Point) -> Self {
        Self {
            point,
            direction,
            steps_in_direction: 1,
            path: Vec::new(),
            distance_to_target: point.distance(target),
        }
    }

    fn calculate_heat_load(&self, map: &Map) -> u32 {
        self.path.iter().map(|point| map.get(point)).sum()
    }
}

impl Eq for Path {}

impl PartialEq<Self> for Path {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd<Self> for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance_to_target.cmp(&other.distance_to_target)
    }
}

fn calculate_new_directions(path: &Path, limits: &Limits) -> Vec<Direction> {
    if path.steps_in_direction >= limits.max {
        match path.direction {
            Direction::North | Direction::South => vec![Direction::East, Direction::West],
            Direction::East | Direction::West => vec![Direction::North, Direction::South],
        }
    } else if path.steps_in_direction < limits.min {
        vec![path.direction]
    } else {
        let opposite = path.direction.opposite();
        vec![
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
        .into_iter()
        .filter(|&d| d != opposite)
        .collect()
    }
}

fn move_path(
    path: Path,
    map: &Map,
    queue: &mut Vec<Path>,
    visited: &mut HashMap<Point, u32>,
    directed_visited: &mut HashMap<(Point, Direction, usize), u32>,
    lowes_heat_load: u32,
    limits: &Limits,
) -> Option<Path> {
    let mut result = None;

    for direction in calculate_new_directions(&path, limits) {
        if let Some(point) = path.point.move_in_direction(&direction, map) {
            let new_path = create_new_path(&path, direction, point, limits, &map.target);

            let new_heat_load = new_path.calculate_heat_load(map);

            if new_heat_load >= lowes_heat_load {
                continue;
            }

            if let Some(&heat_load) =
                directed_visited.get(&(point, direction, new_path.steps_in_direction))
            {
                if heat_load <= new_heat_load {
                    continue;
                }
            }

            directed_visited.insert(
                (point, direction, new_path.steps_in_direction),
                new_heat_load,
            );

            if new_path.steps_in_direction >= limits.min {
                if let Some(&heat_load) = visited.get(&point) {
                    if heat_load > new_heat_load {
                        visited.insert(point, new_heat_load);
                    }
                } else {
                    visited.insert(point, new_heat_load);
                }
            }

            if new_path.point != map.target {
                queue.push(new_path);
            } else if new_path.steps_in_direction >= limits.min {
                result = Some(new_path);
            }
        }
    }

    result
}

fn create_new_path(
    old_path: &Path,
    direction: Direction,
    point: Point,
    limits: &Limits,
    target: &Point,
) -> Path {
    let mut new_path_vec = old_path.path.clone();
    new_path_vec.push(point);

    Path {
        point,
        direction,
        steps_in_direction: if old_path.direction == direction {
            (old_path.steps_in_direction % limits.max) + 1
        } else {
            1
        },
        path: new_path_vec,
        distance_to_target: point.distance(target),
    }
}

fn explore_all_paths(map: &Map, limits: &Limits) -> u32 {
    let mut queue: Vec<Path> = vec![
        Path::from_point_direction(map.start, Direction::North, &map.target),
        Path::from_point_direction(map.start, Direction::East, &map.target),
        Path::from_point_direction(map.start, Direction::South, &map.target),
        Path::from_point_direction(map.start, Direction::West, &map.target),
    ];

    let mut visited: HashMap<Point, u32> = HashMap::new();
    let mut directed_visited: HashMap<(Point, Direction, usize), u32> = HashMap::new();

    let mut lowest_heat_load = u32::MAX;

    while let Some(path) = queue.pop() {
        if let Some(path) = move_path(
            path,
            map,
            &mut queue,
            &mut visited,
            &mut directed_visited,
            lowest_heat_load,
            limits,
        ) {
            let new_heat_load = path.calculate_heat_load(map);
            lowest_heat_load = lowest_heat_load.min(new_heat_load);
        }

        queue.sort();
    }

    *visited.get(&map.target).unwrap()
}

fn main() {
    let data = fs::read_to_string("day17.txt").expect("Can't read input file");

    let map = Map::from_str(&data);

    let part1_result = explore_all_paths(&map, &Limits { min: 1, max: 3 });
    println!("Day 17 Part 1: {}", part1_result);

    let part2_result = explore_all_paths(&map, &Limits { min: 4, max: 10 });
    println!("Day 17 Part 2: {}", part2_result);
}
