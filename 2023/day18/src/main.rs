use std::collections::HashSet;
use std::fs;

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct Instruction {
    direction: Direction,
    distance: usize,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct Point {
    x: i64,
    y: i64,
}

struct Map {
    wall: HashSet<Point>,
    top_left_point: Point,
    bottom_right_point: Point,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            'U' => Self::Up,
            'R' => Self::Right,
            'D' => Self::Down,
            'L' => Self::Left,
            _ => panic!("Unknown direction {}", c),
        }
    }
}

impl Instruction {
    fn from_str(data: &str) -> Self {
        let (direction_str, rest) = data.split_once(' ').unwrap();
        let (distance_str, _) = rest.split_once(' ').unwrap();

        Self {
            direction: Direction::from_char(direction_str.chars().next().unwrap()),
            distance: distance_str.parse().unwrap(),
        }
    }
}

impl Point {
    fn from_xy(x: i64, y: i64) -> Self {
        Point { x, y }
    }
}

impl Map {
    fn new() -> Self {
        Map {
            wall: HashSet::new(),
            top_left_point: Point::from_xy(0, 0),
            bottom_right_point: Point::from_xy(0, 0),
        }
    }

    fn add_wall(&mut self, point: Point) {
        self.top_left_point.x = self.top_left_point.x.min(point.x - 1);
        self.top_left_point.y = self.top_left_point.y.min(point.y - 1);
        self.bottom_right_point.x = self.bottom_right_point.x.max(point.x + 1);
        self.bottom_right_point.y = self.bottom_right_point.y.max(point.y + 1);
        self.wall.insert(point);
    }

    fn compute_filled_area(&self) -> usize {
        let mut visited: HashSet<Point> = HashSet::new();
        let mut queue: Vec<Point> = vec![self.top_left_point];

        while let Some(point) = queue.pop() {
            if point.x >= self.top_left_point.x
                && point.x <= self.bottom_right_point.x
                && point.y >= self.top_left_point.y
                && point.y <= self.bottom_right_point.y
                && !self.wall.contains(&point)
                && !visited.contains(&point)
            {
                queue.push(Point::from_xy(point.x, point.y - 1));
                queue.push(Point::from_xy(point.x + 1, point.y));
                queue.push(Point::from_xy(point.x, point.y + 1));
                queue.push(Point::from_xy(point.x - 1, point.y));

                visited.insert(point);
            }
        }

        ((self.bottom_right_point.x.abs_diff(self.top_left_point.x) + 1)
            * (self.bottom_right_point.y.abs_diff(self.top_left_point.y) + 1)) as usize
            - visited.len()
    }
}

fn parse_instructions(data: &str) -> Vec<Instruction> {
    data.lines().map(Instruction::from_str).collect()
}

fn execute_instruction(instruction: &Instruction, map: &mut Map, start_point: &Point) -> Point {
    match &instruction.direction {
        Direction::Up => {
            for delta_y in 1..=instruction.distance {
                map.add_wall(Point::from_xy(
                    start_point.x,
                    start_point.y - delta_y as i64,
                ));
            }
        }
        Direction::Right => {
            for delta_x in 1..=instruction.distance {
                map.add_wall(Point::from_xy(
                    start_point.x + delta_x as i64,
                    start_point.y,
                ));
            }
        }
        Direction::Down => {
            for delta_y in 1..=instruction.distance {
                map.add_wall(Point::from_xy(
                    start_point.x,
                    start_point.y + delta_y as i64,
                ));
            }
        }
        Direction::Left => {
            for delta_x in 1..=instruction.distance {
                map.add_wall(Point::from_xy(
                    start_point.x - delta_x as i64,
                    start_point.y,
                ));
            }
        }
    }

    match &instruction.direction {
        Direction::Up => Point::from_xy(start_point.x, start_point.y - instruction.distance as i64),
        Direction::Right => {
            Point::from_xy(start_point.x + instruction.distance as i64, start_point.y)
        }
        Direction::Down => {
            Point::from_xy(start_point.x, start_point.y + instruction.distance as i64)
        }
        Direction::Left => {
            Point::from_xy(start_point.x - instruction.distance as i64, start_point.y)
        }
    }
}

fn main() {
    let data = fs::read_to_string("day18.txt").expect("Can't read input file");

    let instructions = parse_instructions(&data);
    let mut map = Map::new();
    let mut start_point = Point::from_xy(0, 0);

    for instruction in &instructions {
        start_point = execute_instruction(instruction, &mut map, &start_point);
    }

    let part1_result = map.compute_filled_area();

    println!("Day 18 Part 1: {}", part1_result);
}
