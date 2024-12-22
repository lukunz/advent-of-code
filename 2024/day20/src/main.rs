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

    fn find_trail_with_deltas(
        &self,
        start: &(usize, usize),
        deltas: &[(isize, isize)],
    ) -> Vec<((usize, usize), usize)> {
        deltas
            .iter()
            .filter_map(|&(x, y)| {
                let target_x = start.0.checked_add_signed(x)?;
                let target_y = start.1.checked_add_signed(y)?;

                if target_x < self.width
                    && target_y < self.height
                    && self.tiles[target_y][target_x] == Tile::Trail
                {
                    Some(((target_x, target_y), (x.abs() + y.abs()) as usize))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }
}

fn main() {
    let data = include_str!("../day20.txt");

    let map = Map::new(data);
    let costs = walk_map(&map, map.start);

    let part1_result = count_shortcuts_gte_100(&map, &costs, 2);

    println!("Day 20 Part 1: {}", part1_result);

    let part2_result = count_shortcuts_gte_100(&map, &costs, 20);

    println!("Day 20 Part 2: {}", part2_result);
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

fn find_deltas_for_distance(distance: isize) -> Vec<(isize, isize)> {
    let mut positions = Vec::new();

    let diff = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

    for d in 2..=distance {
        for (x, y) in (0..=d).zip((0..=d).rev()) {
            for (diff_x, diff_y) in diff {
                positions.push((x * diff_x, y * diff_y));
            }
        }
    }

    positions.sort();
    positions.dedup();

    positions
}

fn find_shortcuts(
    map: &Map,
    costs: &HashMap<(usize, usize), usize>,
    pos: &(usize, usize),
    deltas: &[(isize, isize)],
) -> Vec<usize> {
    let start_costs: usize = *costs.get(pos).unwrap();

    map.find_trail_with_deltas(pos, deltas)
        .iter()
        .filter_map(|(end_pos, delta_cost)| {
            let end_cost = costs.get(end_pos)?;

            if *end_cost > start_costs + delta_cost {
                Some(*end_cost - start_costs - delta_cost)
            } else {
                None
            }
        })
        .collect::<Vec<usize>>()
}

fn count_shortcuts_gte_100(
    map: &Map,
    costs: &HashMap<(usize, usize), usize>,
    distance: isize,
) -> usize {
    let deltas = find_deltas_for_distance(distance);

    let shortcuts = costs
        .iter()
        .flat_map(|(pos, _)| find_shortcuts(map, costs, pos, &deltas))
        .collect::<Vec<usize>>();

    let mut grouped_shortcuts: HashMap<usize, usize> = HashMap::new();
    for savings in shortcuts {
        let entry = grouped_shortcuts.entry(savings).or_default();
        *entry += 1;
    }

    grouped_shortcuts
        .iter()
        .filter_map(|(savings, count)| if *savings >= 100 { Some(*count) } else { None })
        .sum()
}
