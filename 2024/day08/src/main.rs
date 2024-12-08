use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn sub(&self, other: &Self) -> Self {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn add(&self, other: &Position) -> Self {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn in_bounds(&self, width: i64, height: i64) -> bool {
        self.x >= 0 && self.x < width && self.y >= 0 && self.y < height
    }
}

fn main() {
    let data = include_str!("../day08.txt");

    let width = data.find('\n').expect("Input is in wrong format") as i64;
    let height = data.lines().count() as i64;

    let mut antennas: BTreeMap<char, Vec<Position>> = BTreeMap::new();

    for (y, line) in data.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '0'..='9' | 'a'..='z' | 'A'..='Z' => {
                    antennas.entry(ch).or_default().push(Position {
                        x: x as i64,
                        y: y as i64,
                    });
                }
                _ => {}
            }
        }
    }

    let mut antinodes: BTreeSet<Position> = BTreeSet::new();

    for (_, positions) in antennas.iter() {
        for i in 0..positions.len() - 1 {
            let pos_a = &positions[i];
            for pos_b in &positions[i + 1..positions.len()] {
                let distance = pos_b.sub(pos_a);
                let possible_antinodes = [
                    pos_a.add(&distance),
                    pos_a.sub(&distance),
                    pos_b.add(&distance),
                    pos_b.sub(&distance),
                ]
                .into_iter()
                .filter(|position| position != pos_a && position != pos_b)
                .filter(|position| position.in_bounds(width, height));

                antinodes.extend(possible_antinodes);
            }
        }
    }

    println!("Day 8 Part 1: {}", antinodes.len());
}
