use std::collections::HashSet;
use std::hash::RandomState;

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
struct Point {
    x: u32,
    y: u32,
}

struct Map {
    tiles: Vec<Vec<u32>>,
    width: u32,
    height: u32,
}

impl Map {
    fn new(tiles: Vec<Vec<u32>>) -> Self {
        let width = tiles[0].len() as u32;
        let height = tiles.len() as u32;

        Self {
            tiles,
            width,
            height,
        }
    }

    fn get(&self, point: &Point) -> Option<u32> {
        if point.x < self.width && point.y < self.height {
            Some(self.tiles[point.y as usize][point.x as usize])
        } else {
            None
        }
    }

    fn neighbors(&self, point: &Point, height: u32) -> Vec<Point> {
        let mut neighbors = Vec::new();

        for (diff_x, diff_y) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let x = point.x.checked_add_signed(diff_x);
            let y = point.y.checked_add_signed(diff_y);

            if x.is_some() && y.is_some() {
                let x = x.unwrap();
                let y = y.unwrap();

                if x < self.width && y < self.height && self.tiles[y as usize][x as usize] == height
                {
                    neighbors.push(Point { x, y });
                }
            }
        }

        neighbors
    }
}

fn main() {
    let data = include_str!("../day10.txt");

    let tiles = data
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Invalid input"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let map = Map::new(tiles);

    let start_points = map
        .tiles
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, tile)| match tile {
                    0 => Some(Point {
                        x: x as u32,
                        y: y as u32,
                    }),
                    _ => None,
                })
        })
        .collect::<Vec<_>>();

    let trails = start_points
        .into_iter()
        .map(|start_point| {
            let mut nines = Vec::new();
            walk(&map, start_point, &mut nines);

            nines
        })
        .collect::<Vec<_>>();

    let part1_result = trails
        .iter()
        .map(|nines| HashSet::<_, RandomState>::from_iter(nines.iter()).len())
        .sum::<usize>();

    let part2_result = trails.iter().map(|nines| nines.len()).sum::<usize>();

    println!("Day 10 Part 1: {}", part1_result);
    println!("Day 10 Part 2: {}", part2_result);
}

fn walk(map: &Map, point: Point, nines: &mut Vec<Point>) {
    let height = map.get(&point).unwrap();

    if height == 9 {
        nines.push(point);
    } else {
        for neighbor in map.neighbors(&point, height + 1) {
            walk(map, neighbor, nines)
        }
    }
}
