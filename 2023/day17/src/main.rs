use std::collections::{HashMap, VecDeque};
use std::fs;

#[derive(Clone)]
struct Map {
    tiles: Vec<Vec<u32>>,
    width: usize,
    height: usize,
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
    fn from_point_direction(point: Point, direction: Direction) -> Self {
        Self {
            point,
            direction,
            steps_in_direction: 1,
            path: Vec::new(),
        }
    }

    fn calculate_heat_load(&self, map: &Map) -> u32 {
        self.path.iter().map(|point| map.get(point)).sum()
    }
}

fn move_path(
    path: Path,
    map: &Map,
    queue: &mut VecDeque<Path>,
    visited: &mut HashMap<Point, u32>,
    directed_visited: &mut HashMap<(Point, Direction, usize), u32>,
    target: &Point,
    lowes_heat_load: u32,
) -> Option<Path> {
    let new_directions = if path.steps_in_direction >= 3 {
        match path.direction {
            Direction::North | Direction::South => vec![Direction::East, Direction::West],
            Direction::East | Direction::West => vec![Direction::North, Direction::South],
        }
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
    };

    let mut result = None;

    for direction in new_directions {
        if let Some(point) = path.point.move_in_direction(&direction, map) {
            let new_path = create_new_path(&path, direction, point);

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

            if let Some(&heat_load) = visited.get(&point) {
                if heat_load > new_heat_load {
                    visited.insert(point, new_heat_load);
                }
            } else {
                visited.insert(point, new_heat_load);
            }

            if &new_path.point != target {
                queue.push_back(new_path);
            } else {
                result = Some(new_path);
            }
        }
    }

    result
}

fn create_new_path(old_path: &Path, direction: Direction, point: Point) -> Path {
    let mut new_path_vec = old_path.path.clone();
    new_path_vec.push(point);

    Path {
        point,
        direction,
        steps_in_direction: if old_path.direction == direction {
            (old_path.steps_in_direction % 3) + 1
        } else {
            1
        },
        path: new_path_vec,
    }
}

fn explore_all_paths(map: &Map, start: Point, target: Point) -> u32 {
    let mut queue: VecDeque<Path> = VecDeque::new();
    queue.push_back(Path::from_point_direction(start, Direction::North));
    queue.push_back(Path::from_point_direction(start, Direction::East));
    queue.push_back(Path::from_point_direction(start, Direction::South));
    queue.push_back(Path::from_point_direction(start, Direction::West));

    let mut visited: HashMap<Point, u32> = HashMap::new();
    let mut directed_visited: HashMap<(Point, Direction, usize), u32> = HashMap::new();

    let mut lowest_heat_load = u32::MAX;

    while let Some(path) = queue.pop_back() {
        if let Some(path) = move_path(
            path,
            map,
            &mut queue,
            &mut visited,
            &mut directed_visited,
            &target,
            lowest_heat_load,
        ) {
            let new_heat_load = path.calculate_heat_load(map);
            lowest_heat_load = lowest_heat_load.min(new_heat_load);
        }
    }

    *visited.get(&target).unwrap()
}

fn main() {
    let data = fs::read_to_string("day17.txt").expect("Can't read input file");

    let map = Map::from_str(&data);

    let part1_result = explore_all_paths(
        &map,
        Point::from_xy(0, 0),
        Point::from_xy(map.width - 1, map.height - 1),
    );

    println!("Day 17 Part 1: {}", part1_result);
}
