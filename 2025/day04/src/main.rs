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
        .map(|line| line.chars().map(Tile::from_char).collect())
        .collect()
}

fn find_removable_rolls(map: &[Vec<Tile>], removable_rolls: &mut Vec<Point>) {
    let width = map[0].len();
    let height = map.len();

    for y in 0..height {
        for x in 0..width {
            if map[y][x] == Tile::Empty {
                continue;
            }

            let min_x = if x > 0 { x - 1 } else { 0 };
            let max_x = min(x + 1, width - 1);

            let min_y = if y > 0 { y - 1 } else { 0 };
            let max_y = min(y + 1, height - 1);

            let neighbour_count = map[min_y..=max_y]
                .iter()
                .map(|row| {
                    row[min_x..=max_x]
                        .iter()
                        .filter(|&tile| *tile == Tile::Roll)
                        .count()
                })
                .sum::<usize>()
                - 1;

            if neighbour_count < 4 {
                removable_rolls.push(Point { x, y });
            }
        }
    }
}

fn main() {
    let data = include_str!("../day04.txt");

    let mut map = parse_map(data);

    let mut accessable_rolls: Vec<Point> = Vec::new();
    find_removable_rolls(&map, &mut accessable_rolls);

    let mut total_remove_count = accessable_rolls.len();

    println!("Day 04 Part 1: {}", total_remove_count);

    while !accessable_rolls.is_empty() {
        for p in accessable_rolls.drain(..) {
            map[p.y][p.x] = Tile::Empty;
        }

        find_removable_rolls(&map, &mut accessable_rolls);
        total_remove_count += accessable_rolls.len();
    }

    println!("Day 04 Part 2: {}", total_remove_count);
}
