use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Copy, Clone)]
enum Tile {
    Empty,
    Blocked,
}

#[derive(PartialEq, Clone, Copy, Hash, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Hash, Eq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Eq, PartialEq)]
enum MapWalkResult {
    LeftMap,
    Looped,
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(tiles: Vec<Vec<Tile>>) -> Self {
        let height = tiles.len();
        let width = tiles[0].len();

        Self {
            tiles,
            width,
            height,
        }
    }

    fn get(&self, position: &Position) -> Tile {
        self.tiles[position.y][position.x]
    }

    fn set(&mut self, position: &Position, tile: Tile) {
        self.tiles[position.y][position.x] = tile;
    }

    fn north(&self, position: Position) -> Option<Position> {
        if position.y > 0 {
            Some(Position {
                x: position.x,
                y: position.y - 1,
            })
        } else {
            None
        }
    }

    fn east(&self, position: Position) -> Option<Position> {
        if position.x < self.width - 1 {
            Some(Position {
                x: position.x + 1,
                y: position.y,
            })
        } else {
            None
        }
    }

    fn south(&self, position: Position) -> Option<Position> {
        if position.y < self.height - 1 {
            Some(Position {
                x: position.x,
                y: position.y + 1,
            })
        } else {
            None
        }
    }

    fn west(&self, position: Position) -> Option<Position> {
        if position.x > 0 {
            Some(Position {
                x: position.x - 1,
                y: position.y,
            })
        } else {
            None
        }
    }
}

fn main() {
    let data = include_str!("../day06.txt");

    let tiles = data
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' | '^' | '>' | 'v' | '<' => Tile::Empty,
                    '#' => Tile::Blocked,
                    _ => panic!("Invalid input"),
                })
                .collect::<Vec<Tile>>()
        })
        .collect::<Vec<Vec<Tile>>>();

    let mut map = Map::new(tiles);
    let (player_position, player_direction) = find_player(data).expect("No player found");

    let (result, visited, possible_obstacles) =
        walk_map(&mut map, player_position, player_direction, true);

    assert_eq!(result, MapWalkResult::LeftMap);

    println!("Day 6 Part 1: {}", visited.len());
    println!("Day 6 Part 2: {}", possible_obstacles.len());
}

fn find_player(data: &str) -> Option<(Position, Direction)> {
    for (y, line) in data.lines().enumerate() {
        for (x, tile) in line.chars().enumerate() {
            match tile {
                '^' => return Some((Position { x, y }, Direction::North)),
                '>' => return Some((Position { x, y }, Direction::East)),
                'v' => return Some((Position { x, y }, Direction::South)),
                '<' => return Some((Position { x, y }, Direction::West)),
                _ => {}
            }
        }
    }

    None
}

fn has_visited(
    visited: &HashMap<Position, HashSet<Direction>>,
    position: &Position,
    direction: &Direction,
) -> bool {
    if let Some(directions) = visited.get(position) {
        if directions.contains(direction) {
            return true;
        }
    }

    false
}

fn walk_map(
    map: &mut Map,
    start_position: Position,
    start_direction: Direction,
    find_possible_obstacles: bool,
) -> (
    MapWalkResult,
    HashMap<Position, HashSet<Direction>>,
    HashSet<Position>,
) {
    let mut visited: HashMap<Position, HashSet<Direction>> = HashMap::new();
    let mut possible_obstacles: HashSet<Position> = HashSet::new();

    let mut player_position = start_position;
    let mut player_direction = start_direction;

    loop {
        if has_visited(&visited, &player_position, &player_direction) {
            return (MapWalkResult::Looped, visited, possible_obstacles);
        }

        visited
            .entry(player_position)
            .or_default()
            .insert(player_direction);

        let new_position = match player_direction {
            Direction::North => map.north(player_position),
            Direction::East => map.east(player_position),
            Direction::South => map.south(player_position),
            Direction::West => map.west(player_position),
        };

        if let Some(new_position) = new_position {
            match map.get(&new_position) {
                Tile::Empty => {
                    if find_possible_obstacles {
                        map.set(&new_position, Tile::Blocked);
                        let (result, _, _) = walk_map(map, start_position, start_direction, false);
                        map.set(&new_position, Tile::Empty);

                        if result == MapWalkResult::Looped {
                            possible_obstacles.insert(new_position);
                        }
                    }

                    player_position = new_position;
                }
                Tile::Blocked => {
                    player_direction = player_direction.turn();
                }
            }
        } else {
            return (MapWalkResult::LeftMap, visited, possible_obstacles);
        }
    }
}
