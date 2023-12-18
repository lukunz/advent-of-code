use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::ops::RangeInclusive;

#[derive(Eq, PartialEq, Debug)]
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

struct Wall {
    direction: Direction,
    start: Point,
    end: Point,
}

struct Map {
    walls: Vec<Wall>,
    top_left_point: Point,
    bottom_right_point: Point,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            'U' | '3' => Self::Up,
            'R' | '0' => Self::Right,
            'D' | '1' => Self::Down,
            'L' | '2' => Self::Left,
            _ => panic!("Unknown direction {}", c),
        }
    }
}

impl Instruction {
    fn from_str(data: &str, color_mode: bool) -> Self {
        let (direction_str, rest) = data.split_once(' ').unwrap();
        let (distance_str, rest) = rest.split_once(' ').unwrap();

        if color_mode {
            let distance_str = &rest[2..rest.len() - 2];
            let direction_char = rest.chars().nth(7).unwrap();

            Self {
                direction: Direction::from_char(direction_char),
                distance: usize::from_str_radix(distance_str, 16).unwrap(),
            }
        } else {
            Self {
                direction: Direction::from_char(direction_str.chars().next().unwrap()),
                distance: distance_str.parse().unwrap(),
            }
        }
    }
}

impl Point {
    fn from_xy(x: i64, y: i64) -> Self {
        Point { x, y }
    }

    fn min(&self, other: &Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    fn max(&self, other: &Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    fn shift_up(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn shift_right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn shift_down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn shift_left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }
}

impl Wall {
    fn is_horizontal(&self) -> bool {
        self.direction == Direction::Left || self.direction == Direction::Right
    }

    fn intersects_y(&self, y: i64) -> Option<RangeInclusive<i64>> {
        if self.is_horizontal() && self.start.y == y {
            Some(self.start.x.min(self.end.x)..=self.start.x.max(self.end.x))
        } else if self.start.y <= y && y <= self.end.y || self.end.y <= y && y <= self.start.y {
            Some(self.start.x..=self.end.x)
        } else {
            None
        }
    }
}

impl Map {
    fn new() -> Self {
        Map {
            walls: Vec::new(),
            top_left_point: Point::from_xy(0, 0),
            bottom_right_point: Point::from_xy(0, 0),
        }
    }

    fn add_wall(&mut self, wall: Wall) {
        self.top_left_point = self.top_left_point.min(&wall.start);
        self.top_left_point = self.top_left_point.min(&wall.end);
        self.bottom_right_point = self.bottom_right_point.max(&wall.start);
        self.bottom_right_point = self.bottom_right_point.max(&wall.end);

        self.walls.push(wall);
    }

    fn compute_filled_area(&self) -> usize {
        let wall_by_start = self
            .walls
            .iter()
            .map(|wall| (wall.start, wall))
            .collect::<HashMap<Point, &Wall>>();

        let wall_by_end = self
            .walls
            .iter()
            .map(|wall| (wall.end, wall))
            .collect::<HashMap<Point, &Wall>>();

        (self.top_left_point.y..=self.bottom_right_point.y)
            .into_par_iter()
            .map(|y| self.calculate_line_area(&wall_by_start, &wall_by_end, y))
            .sum()
    }

    fn calculate_line_area(
        &self,
        wall_by_start: &HashMap<Point, &Wall>,
        wall_by_end: &HashMap<Point, &Wall>,
        y: i64,
    ) -> usize {
        let mut walls: Vec<&Wall> = self
            .walls
            .iter()
            .filter(|wall| wall.intersects_y(y).is_some())
            .collect();

        let mut state_changes: Vec<i64> = Vec::new();

        walls.sort_by(|a, b| a.start.x.min(a.end.x).cmp(&b.start.x.min(b.end.x)));

        let mut skip = false;
        for wall in &walls {
            if skip {
                skip = false;
                continue;
            }
            if wall.is_horizontal() {
                let is_inside = state_changes.len() % 2 == 1;
                if wall.direction == Direction::Right {
                    let left_wall = wall_by_end.get(&wall.start.shift_left()).unwrap();
                    let right_wall = find_next_wall_after_horizontal(wall_by_start, wall);

                    if left_wall.direction == right_wall.direction {
                        if !is_inside {
                            state_changes.pop();
                            state_changes.push(wall.end.x);
                        }
                    } else if is_inside {
                        state_changes.push(wall.end.x);
                    } else {
                        state_changes.pop();
                    }
                } else {
                    // wall.direction == Direction::Left
                    let right_wall = wall_by_end.get(&wall.start.shift_right()).unwrap();
                    let left_wall = find_next_wall_after_horizontal(wall_by_start, wall);

                    if left_wall.direction == right_wall.direction {
                        if !is_inside {
                            state_changes.push(wall.end.x);
                            skip = true;
                        }
                    } else if !is_inside {
                        state_changes.push(wall.end.x);
                    } else {
                        skip = true;
                    }
                }
            } else {
                state_changes.push(wall.start.x);
            }
        }

        let line_area = state_changes
            .chunks(2)
            .map(|chunk| chunk[0].abs_diff(chunk[1]) + 1)
            .sum::<u64>() as usize;
        line_area
    }

    fn print(&self) {
        let mut grid: Vec<Vec<char>> = Vec::new();

        for _ in self.top_left_point.y..=self.bottom_right_point.y {
            grid.push(
                ['.']
                    .repeat(self.top_left_point.x.abs_diff(self.bottom_right_point.x) as usize + 1),
            );
        }

        for wall in &self.walls {
            let start_point = wall.start.min(&wall.end);
            for y in 0..=wall.start.y.abs_diff(wall.end.y) {
                for x in 0..=wall.start.x.abs_diff(wall.end.x) {
                    grid[(y as i64 + start_point.y - self.top_left_point.y) as usize]
                        [(x as i64 + start_point.x - self.top_left_point.x) as usize] = '#';
                }
            }
        }

        for line in grid {
            for c in line {
                print!("{}", c);
            }
            println!();
        }
    }
}

fn parse_instructions(data: &str, color_mode: bool) -> Vec<Instruction> {
    data.lines()
        .map(|line| Instruction::from_str(line, color_mode))
        .collect()
}

fn execute_instruction(instruction: &Instruction, map: &mut Map, start_point: &Point) -> Point {
    match &instruction.direction {
        Direction::Up => {
            let end = Point::from_xy(start_point.x, start_point.y - instruction.distance as i64);
            map.add_wall(Wall {
                start: Point::from_xy(start_point.x, start_point.y - 1),
                end,
                direction: Direction::Up,
            });

            end
        }
        Direction::Right => {
            let end = Point::from_xy(start_point.x + instruction.distance as i64, start_point.y);
            map.add_wall(Wall {
                start: Point::from_xy(start_point.x + 1, start_point.y),
                end,
                direction: Direction::Right,
            });
            end
        }
        Direction::Down => {
            let end = Point::from_xy(start_point.x, start_point.y + instruction.distance as i64);
            map.add_wall(Wall {
                start: Point::from_xy(start_point.x, start_point.y + 1),
                end,
                direction: Direction::Down,
            });

            end
        }
        Direction::Left => {
            let end = Point::from_xy(start_point.x - instruction.distance as i64, start_point.y);
            map.add_wall(Wall {
                start: Point::from_xy(start_point.x - 1, start_point.y),
                end,
                direction: Direction::Left,
            });

            end
        }
    }
}

fn find_next_wall_after_horizontal<'a>(
    wall_by_start: &HashMap<Point, &'a Wall>,
    wall: &Wall,
) -> &'a Wall {
    if !wall.is_horizontal() {
        panic!("Wall not horizontal!");
    }
    if let Some(w) = wall_by_start.get(&wall.end.shift_up()) {
        w
    } else if let Some(w) = wall_by_start.get(&wall.end.shift_down()) {
        w
    } else {
        panic!("Hole in wall!");
    }
}

fn main() {
    let data = fs::read_to_string("day18.txt").expect("Can't read input file");

    let instructions = parse_instructions(&data, false);
    let mut map = Map::new();
    let mut start_point = Point::from_xy(0, 0);

    for instruction in &instructions {
        start_point = execute_instruction(instruction, &mut map, &start_point);
    }

    let part1_result = map.compute_filled_area();

    println!("Day 18 Part 1: {}", part1_result);

    // map.print();

    let instructions = parse_instructions(&data, true);
    let mut map = Map::new();
    let mut start_point = Point::from_xy(0, 0);

    for instruction in &instructions {
        start_point = execute_instruction(instruction, &mut map, &start_point);
    }

    let part1_result = map.compute_filled_area();

    println!("Day 18 Part 2: {}", part1_result);
}
