use std::cmp::min;

#[derive(PartialEq)]
enum Tile {
    Empty,
    Roll,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '@' => Self::Roll,
            c => panic!("Unknown tile '{}'", c),
        }
    }
}

struct Point {
    x: usize,
    y: usize,
}

fn parse_map(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| Tile::from_char(c)).collect())
        .collect()
}

fn main() {
    let data = include_str!("../day04.txt");

    let map = parse_map(data);
    let width = map[0].len();
    let height = map.len();

    let mut accessable_rolls: Vec<Point> = Vec::new();

    for y in 0..height {
        for x in 0..width {
            if map[y][x] == Tile::Empty {
                continue;
            }

            let min_x = if x > 0 { x - 1 } else { 0 };
            let max_x = min(x + 1, width - 1);

            let min_y = if y > 0 { y - 1 } else { 0 };
            let max_y = min(y + 1, height - 1);

            let mut neighbour_count = 0;

            for test_y in min_y..=max_y {
                for test_x in min_x..=max_x {
                    if map[test_y][test_x] == Tile::Roll {
                        neighbour_count += 1;
                    }
                }
            }

            neighbour_count -= 1;

            if neighbour_count < 4 {
                accessable_rolls.push(Point { x, y });
            }
        }
    }

    println!("Day 04 Part 1: {}", accessable_rolls.len());
}
