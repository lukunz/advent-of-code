use std::collections::HashSet;
use std::fs;

enum Tile {
    Empty,
    HorizontalSplitter,
    VerticalSplitter,
    TopLeftBottomRightMirror,
    TopRightBottomLeftMirror,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '-' => Self::HorizontalSplitter,
            '|' => Self::VerticalSplitter,
            '\\' => Self::TopLeftBottomRightMirror,
            '/' => Self::TopRightBottomLeftMirror,
            _ => panic!("Unknown tile {}", c),
        }
    }
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    width: i64,
    height: i64,
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Map {
    fn contains(&self, point: &Point) -> bool {
        point.x >= 0 && point.x < self.width && point.y >= 0 && point.y < self.height
    }

    fn get(&self, point: &Point) -> &Tile {
        &self.tiles[point.y as usize][point.x as usize]
    }
}

impl Point {
    fn move_to(&self, d: &Direction) -> Point {
        match d {
            Direction::Up => Point {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Right => Point {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Down => Point {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Point {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn parse_map(data: &str) -> Map {
    let tiles: Vec<Vec<Tile>> = data
        .lines()
        .map(|line| line.chars().map(Tile::from_char).collect())
        .collect();

    let width = tiles[0].len() as i64;
    let height = tiles.len() as i64;

    Map {
        tiles,
        width,
        height,
    }
}

fn move_beam(point: &Point, direction: Direction, result: &mut Vec<(Point, Direction)>) {
    result.push((point.move_to(&direction), direction))
}

fn step_beam(
    map: &Map,
    point: Point,
    direction: Direction,
    energized_tiles: &mut HashSet<(Point, Direction)>,
    queue: &mut Vec<(Point, Direction)>,
) {
    if !map.contains(&point) || energized_tiles.contains(&(point, direction)) {
        return;
    }

    match map.get(&point) {
        Tile::Empty => move_beam(&point, direction, queue),
        Tile::HorizontalSplitter => match &direction {
            Direction::Up | Direction::Down => {
                move_beam(&point, Direction::Left, queue);
                move_beam(&point, Direction::Right, queue);
            }
            _ => move_beam(&point, direction, queue),
        },
        Tile::VerticalSplitter => match &direction {
            Direction::Left | Direction::Right => {
                move_beam(&point, Direction::Up, queue);
                move_beam(&point, Direction::Down, queue);
            }
            _ => move_beam(&point, direction, queue),
        },
        Tile::TopLeftBottomRightMirror => {
            let new_direction = match &direction {
                Direction::Up => Direction::Left,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
            };
            move_beam(&point, new_direction, queue);
        }
        Tile::TopRightBottomLeftMirror => {
            let new_direction = match &direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Up,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
            };
            move_beam(&point, new_direction, queue);
        }
    }

    energized_tiles.insert((point, direction));
}

fn fire_beam(map: &Map, start: (Point, Direction)) -> usize {
    let mut queue = vec![start];
    let mut energized_tiles: HashSet<(Point, Direction)> = HashSet::new();

    while let Some((point, direction)) = queue.pop() {
        step_beam(map, point, direction, &mut energized_tiles, &mut queue);
    }

    energized_tiles
        .iter()
        .map(|(point, _)| point)
        .collect::<HashSet<&Point>>()
        .len()
}

fn main() {
    let data = fs::read_to_string("day16.txt").expect("Can't read input file");

    let map = parse_map(&data);

    let part1_result = fire_beam(&map, (Point { x: 0, y: 0 }, Direction::Right));

    println!("Day 16 Part 1: {}", part1_result);

    let mut part2_result = 0;

    part2_result = (0..map.width).fold(part2_result, |result, x| {
        result.max(fire_beam(&map, (Point { x, y: 0 }, Direction::Down)))
    });
    part2_result = (0..map.width).fold(part2_result, |result, x| {
        result.max(fire_beam(
            &map,
            (
                Point {
                    x,
                    y: map.height - 1,
                },
                Direction::Up,
            ),
        ))
    });
    part2_result = (0..map.height).fold(part2_result, |result, y| {
        result.max(fire_beam(&map, (Point { x: 0, y }, Direction::Right)))
    });
    part2_result = (0..map.height).fold(part2_result, |result, y| {
        result.max(fire_beam(
            &map,
            (
                Point {
                    x: map.width - 1,
                    y,
                },
                Direction::Left,
            ),
        ))
    });

    println!("Day 16 Part 2: {}", part2_result);
}
