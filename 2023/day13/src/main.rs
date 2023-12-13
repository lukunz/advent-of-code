use std::fs;

#[derive(PartialEq, Copy, Clone)]
enum Ground {
    Ash,
    Rock,
}

impl Ground {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Ash,
            '#' => Self::Rock,
            _ => panic!("Unknown ground: {}", c),
        }
    }
}

type Map = Vec<Vec<Ground>>;

fn parse_input(data: &str) -> Vec<Map> {
    let mut result: Vec<Map> = Vec::new();
    let mut map: Map = Vec::new();

    for line in data.lines() {
        if line.is_empty() {
            result.push(map);
            map = Vec::new();
        } else {
            map.push(line.chars().map(Ground::from_char).collect());
        }
    }

    if !map.is_empty() {
        result.push(map);
    }

    result
}

fn is_mirrored_at_row(map: &Map, row: usize) -> bool {
    (0..=row).all(|index| {
        let mirrored_index = 2 * row - index - 1;

        if mirrored_index >= map.len() {
            true
        } else {
            map[index]
                .iter()
                .zip(&map[mirrored_index])
                .all(|(a, b)| a == b)
        }
    })
}

fn is_mirrored_at_col(map: &Map, col: usize) -> bool {
    (0..=col).all(|index| {
        let mirrored_index = 2 * col - index - 1;

        if mirrored_index >= map[0].len() {
            true
        } else {
            map.iter()
                .map(|row| (row[index], row[mirrored_index]))
                .all(|(a, b)| a == b)
        }
    })
}

fn main() {
    let data = fs::read_to_string("day13.txt").expect("Can't read input file");

    let maps = parse_input(&data);

    let part1_result: usize = maps
        .iter()
        .map(|map| {
            for row in 1..map.len() {
                if is_mirrored_at_row(map, row) {
                    return row * 100;
                }
            }

            for col in 1..map[0].len() {
                if is_mirrored_at_col(map, col) {
                    return col;
                }
            }

            0
        })
        .sum();

    println!("Day 13 Part 1: {}", part1_result);
}
