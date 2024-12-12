use std::collections::{BTreeMap, BTreeSet};

struct Map {
    plots: Vec<Vec<Plot>>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(plots: Vec<Vec<Plot>>) -> Self {
        let width = plots[0].len();
        let height = plots.len();

        Self {
            plots,
            width,
            height,
        }
    }

    fn neighbors(&self, plot: &Plot) -> Vec<(&Plot, Direction)> {
        let mut neighbors = Vec::new();

        for (diff_x, diff_y, dir) in [
            (1, 0, Direction::East),
            (0, 1, Direction::South),
            (-1, 0, Direction::West),
            (0, -1, Direction::North),
        ] {
            let x = plot.x.checked_add_signed(diff_x);
            let y = plot.y.checked_add_signed(diff_y);

            if x.is_some() && y.is_some() {
                let x = x.unwrap();
                let y = y.unwrap();

                if x < self.width && y < self.height {
                    neighbors.push((&self.plots[y][x], dir));
                }
            }
        }

        neighbors
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Plot {
    label: char,
    x: usize,
    y: usize,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Fence {
    dir: Direction,
    x: usize,
    y: usize,
}

impl Fence {
    fn new(dir: Direction, x: usize, y: usize) -> Self {
        Self { dir, x, y }
    }
}

struct Region<'a> {
    plots: Vec<&'a Plot>,
    fence_count: usize,
    sides_count: usize,
}

fn main() {
    let data = include_str!("../day12.txt");

    let map = data
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, char)| Plot { label: char, x, y })
                .collect::<Vec<Plot>>()
        })
        .collect::<Vec<_>>();

    let map = Map::new(map);

    let mut regions: Vec<Region> = Vec::new();
    let mut used_plots: BTreeSet<&Plot> = BTreeSet::new();

    for plot in map.plots.iter().flatten() {
        if !used_plots.contains(plot) {
            let region = find_region(&map, plot);
            used_plots.extend(&region.plots);
            regions.push(region);
        }
    }

    let part1_result = regions
        .iter()
        .map(|region| region.plots.len() * region.fence_count)
        .sum::<usize>();

    let part2_result = regions
        .iter()
        .map(|region| region.plots.len() * region.sides_count)
        .sum::<usize>();

    println!("Day 12 Part 1: {}", part1_result);
    println!("Day 12 Part 2: {}", part2_result);
}

fn find_region<'a>(map: &'a Map, plot: &'a Plot) -> Region<'a> {
    let mut plots_to_check = vec![plot];
    let mut region_plots = BTreeSet::new();
    let mut fences = Vec::new();

    while let Some(p) = plots_to_check.pop() {
        region_plots.insert(p);

        for (neighbor, dir) in map.neighbors(p) {
            if neighbor.label == p.label {
                if !region_plots.contains(&neighbor) && !plots_to_check.contains(&neighbor) {
                    plots_to_check.push(neighbor);
                }
            } else {
                fences.push(Fence::new(dir, p.x, p.y));
            }
        }
    }

    for p in region_plots.iter() {
        if p.x == 0 {
            fences.push(Fence::new(Direction::West, p.x, p.y));
        } else if p.x == map.width - 1 {
            fences.push(Fence::new(Direction::East, p.x, p.y));
        }

        if p.y == 0 {
            fences.push(Fence::new(Direction::North, p.x, p.y));
        } else if p.y == map.height - 1 {
            fences.push(Fence::new(Direction::South, p.x, p.y));
        }
    }

    let sides = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ]
    .into_iter()
    .map(|dir| count_sides(&fences, dir))
    .sum::<usize>();

    Region {
        plots: Vec::from_iter(region_plots),
        fence_count: fences.len(),
        sides_count: sides,
    }
}

fn count_sides(fences: &[Fence], dir: Direction) -> usize {
    let dir_fences = fences.iter().filter_map(|fence| {
        if fence.dir == dir {
            match fence.dir {
                Direction::East | Direction::West => Some((fence.x, fence.y)),
                Direction::North | Direction::South => Some((fence.y, fence.x)),
            }
        } else {
            None
        }
    });

    let mut grouped_fences: BTreeMap<usize, Vec<usize>> = BTreeMap::new();

    for (group, index) in dir_fences {
        grouped_fences.entry(group).or_default().push(index);
    }

    grouped_fences
        .values_mut()
        .map(|indexes| {
            indexes.sort_unstable();
            indexes
                .windows(2)
                .map(|w| (w[1] - w[0] - 1).min(1))
                .sum::<usize>()
                + 1
        })
        .sum::<usize>()
}
