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

fn count_differences(a: &[Ground], b: &[Ground]) -> usize {
    a.iter()
        .zip(b)
        .map(|(a, b)| if a == b { 0 } else { 1 })
        .sum()
}

fn count_mirror_defects_at_row(map: &Map, row: usize) -> usize {
    (0..row)
        .map(|index| {
            let mirrored_index = 2 * row - index - 1;

            if mirrored_index >= map.len() {
                0
            } else {
                count_differences(&map[index], &map[mirrored_index])
            }
        })
        .sum()
}

fn count_mirror_defects_at_col(map: &Map, col: usize) -> usize {
    (0..col)
        .map(|index| {
            let mirrored_index = 2 * col - index - 1;

            if mirrored_index >= map[0].len() {
                0
            } else {
                let col_a: Vec<Ground> = map.iter().map(|row| row[index]).collect();
                let col_b: Vec<Ground> = map.iter().map(|row| row[mirrored_index]).collect();

                count_differences(&col_a, &col_b)
            }
        })
        .sum()
}

fn calculate_result(maps: &[Map], defects: usize) -> usize {
    maps.iter()
        .map(|map| {
            for row in 1..map.len() {
                if count_mirror_defects_at_row(map, row) == defects {
                    return row * 100;
                }
            }

            for col in 1..map[0].len() {
                if count_mirror_defects_at_col(map, col) == defects {
                    return col;
                }
            }

            0
        })
        .sum()
}
fn main() {
    let data = fs::read_to_string("day13.txt").expect("Can't read input file");

    let maps = parse_input(&data);

    let part1_result = calculate_result(&maps, 0);
    let part2_result = calculate_result(&maps, 1);

    println!("Day 13 Part 1: {}", part1_result);
    println!("Day 13 Part 2: {}", part2_result);
}
