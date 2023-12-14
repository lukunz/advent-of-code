use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};

#[derive(PartialEq, Hash)]
enum Tile {
    RoundRock,
    RegularRock,
    Empty,
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            'O' => Self::RoundRock,
            '#' => Self::RegularRock,
            '.' => Self::Empty,
            _ => panic!("Unknown tile '{}'", c),
        }
    }
}

type Map = Vec<Vec<Tile>>;

fn parse_input(data: &str) -> Map {
    data.lines()
        .map(|line| line.chars().map(Tile::from_char).collect())
        .collect()
}

fn shift_rocks(map: &mut Map, direction: Direction) {
    while shift_rocks_step(map, &direction) {}
}

fn shift_rocks_step(map: &mut Map, direction: &Direction) -> bool {
    let mut moved_rock = false;

    match direction {
        Direction::North => {
            for y in 1..map.len() {
                for x in 0..map[y].len() {
                    moved_rock |= move_rock(map, (x, y), direction);
                }
            }
        }
        Direction::East => {
            for y in 0..map.len() {
                for x in (0..map[y].len() - 1).rev() {
                    moved_rock |= move_rock(map, (x, y), direction);
                }
            }
        }
        Direction::South => {
            for y in (0..map.len() - 1).rev() {
                for x in 0..map[y].len() {
                    moved_rock |= move_rock(map, (x, y), direction);
                }
            }
        }
        Direction::West => {
            for y in 0..map.len() {
                for x in 1..map[y].len() {
                    moved_rock |= move_rock(map, (x, y), direction);
                }
            }
        }
    }

    moved_rock
}

fn move_rock(map: &mut Map, point: (usize, usize), direction: &Direction) -> bool {
    let target_point = match direction {
        Direction::North => (point.0, point.1 - 1),
        Direction::East => (point.0 + 1, point.1),
        Direction::South => (point.0, point.1 + 1),
        Direction::West => (point.0 - 1, point.1),
    };

    if map[point.1][point.0] == Tile::RoundRock
        && map[target_point.1][target_point.0] == Tile::Empty
    {
        map[target_point.1][target_point.0] = Tile::RoundRock;
        map[point.1][point.0] = Tile::Empty;

        true
    } else {
        false
    }
}

fn calculate_load(map: &Map) -> usize {
    map.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .map(|tile| {
                    if tile == &Tile::RoundRock {
                        map.len() - y
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

fn run_spin_cycle(map: &mut Map) {
    for direction in [
        Direction::North,
        Direction::West,
        Direction::South,
        Direction::East,
    ] {
        shift_rocks(map, direction);
    }
}

fn calculate_hash(map: &Map) -> u64 {
    let mut hasher = DefaultHasher::new();
    map.hash(&mut hasher);
    hasher.finish()
}

fn main() {
    let data = fs::read_to_string("day14.txt").expect("Can't read input file");

    let mut map = parse_input(&data);
    shift_rocks(&mut map, Direction::North);
    let part1_result = calculate_load(&map);
    println!("Day 14 Part 1: {}", part1_result);

    let mut map = parse_input(&data);
    let mut cache: Vec<(u64, usize)> = vec![(calculate_hash(&map), calculate_load(&map))];

    for n in 0..1_000_000_000 {
        run_spin_cycle(&mut map);
        let hash = calculate_hash(&map);
        let load = calculate_load(&map);

        if let Some(start) = cache.iter().position(|&item| item == (hash, load)) {
            let rest = (1_000_000_000 - start) % (n - start + 1);
            println!("Day 14 Part 2: {}", cache[rest + start].1);
            break;
        }

        cache.push((hash, load));
    }
}
