use std::fs;

#[derive(PartialEq)]
enum Tile {
    RoundRock,
    RegularRock,
    Empty,
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

fn shift_rocks(map: &mut Map) {
    let mut moved_rock = true;

    while moved_rock {
        moved_rock = false;
        for y in 1..map.len() {
            for x in 0..map[y].len() {
                if map[y][x] == Tile::RoundRock && map[y - 1][x] == Tile::Empty {
                    map[y - 1][x] = Tile::RoundRock;
                    map[y][x] = Tile::Empty;
                    moved_rock = true;
                }
            }
        }
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

fn main() {
    let data = fs::read_to_string("day14.txt").expect("Can't read input file");

    let mut map = parse_input(&data);
    shift_rocks(&mut map);
    let part1_result = calculate_load(&map);

    println!("Day 14 Part 1: {}", part1_result);
}
