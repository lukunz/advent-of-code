use std::collections::HashMap;

#[derive(PartialEq, Debug)]
enum Tile {
    Wall,
    Trail,
}

#[derive(PartialEq, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
    start: (usize, usize),
}

impl Map {
    fn new(input: &str) -> Self {
        let tiles = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '#' => Tile::Wall,
                        '.' | 'S' | 'E' => Tile::Trail,
                        _ => panic!("Unknown tile '{}'", c),
                    })
                    .collect::<Vec<Tile>>()
            })
            .collect::<Vec<Vec<Tile>>>();

        let width = tiles[0].len();
        let height = tiles.len();

        let start = input
            .lines()
            .enumerate()
            .find_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .find_map(move |(x, c)| if c == 'S' { Some((x, y)) } else { None })
            })
            .expect("No start found");

        Self {
            tiles,
            width,
            height,
            start,
        }
    }

    fn neighbors(&self, (x, y): (usize, usize)) -> Vec<((usize, usize), Direction)> {
        let mut neighbors = Vec::new();

        for (diff_x, diff_y, dir) in [
            (1, 0, Direction::East),
            (0, 1, Direction::South),
            (-1, 0, Direction::West),
            (0, -1, Direction::North),
        ] {
            let x = x.checked_add_signed(diff_x);
            let y = y.checked_add_signed(diff_y);

            if x.is_some() && y.is_some() {
                let x = x.unwrap();
                let y = y.unwrap();

                if x < self.width && y < self.height {
                    neighbors.push(((x, y), dir));
                }
            }
        }

        neighbors
    }

    fn step(&self, (x, y): (usize, usize), dir: &Direction) -> Option<(usize, usize)> {
        match dir {
            Direction::North if y > 0 => Some((x, y - 1)),
            Direction::East if x < self.width - 1 => Some((x + 1, y)),
            Direction::South if y < self.height - 1 => Some((x, y + 1)),
            Direction::West if x > 0 => Some((x - 1, y)),
            _ => None,
        }
    }
}

fn main() {
    let data = include_str!("../day20.txt");

    let map = Map::new(data);

    let costs = walk_map(&map, map.start);

    let shortcuts = costs
        .keys()
        .flat_map(|pos| find_shortcuts(&map, &costs, pos))
        .collect::<Vec<_>>();

    let mut grouped_shortcuts: HashMap<usize, usize> = HashMap::new();
    for savings in shortcuts {
        let entry = grouped_shortcuts.entry(savings).or_default();
        *entry += 1;
    }

    let part1_result: usize = grouped_shortcuts
        .iter()
        .filter_map(|(savings, count)| if *savings >= 100 { Some(*count) } else { None })
        .sum();

    println!("Day 20 Part 1: {}", part1_result);
}

fn walk_map(map: &Map, start: (usize, usize)) -> HashMap<(usize, usize), usize> {
    let mut costs = HashMap::new();

    let mut next_pos = Some(start);

    let mut current_cost = 0;
    while let Some(pos) = next_pos {
        costs.insert(pos, current_cost);
        current_cost += 1;

        let neighbors = map
            .neighbors(pos)
            .into_iter()
            .filter_map(|((x, y), _)| {
                if map.tiles[y][x] == Tile::Trail && !costs.contains_key(&(x, y)) {
                    Some((x, y))
                } else {
                    None
                }
            })
            .collect::<Vec<(usize, usize)>>();

        assert!(neighbors.len() < 2);

        next_pos = neighbors.first().cloned();
    }

    costs
}

fn find_shortcuts(
    map: &Map,
    costs: &HashMap<(usize, usize), usize>,
    pos: &(usize, usize),
) -> Vec<usize> {
    let start_costs: usize = *costs.get(pos).unwrap();

    map.neighbors(*pos)
        .iter()
        .filter(|((x, y), _)| map.tiles[*y][*x] == Tile::Wall)
        .filter_map(|((x, y), dir)| {
            let (new_x, new_y) = map.step((*x, *y), dir)?;

            match map.tiles[new_y][new_x] {
                Tile::Wall => None,
                Tile::Trail => {
                    let max_costs = costs[&(new_x, new_y)];
                    if max_costs > 1 {
                        Some(max_costs - 2)
                    } else {
                        None
                    }
                }
            }
        })
        .filter(|&max_costs| max_costs > start_costs)
        .map(|max_costs| max_costs - start_costs)
        .collect()
}
