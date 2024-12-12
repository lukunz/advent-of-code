use std::collections::BTreeSet;

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

    fn neighbors(&self, plot: &Plot) -> Vec<Plot> {
        let mut neighbors = Vec::new();

        for (diff_x, diff_y) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let x = plot.x.checked_add_signed(diff_x);
            let y = plot.y.checked_add_signed(diff_y);

            if x.is_some() && y.is_some() {
                let x = x.unwrap();
                let y = y.unwrap();

                if x < self.width && y < self.height {
                    neighbors.push(self.plots[y][x]);
                }
            }
        }

        neighbors
    }
}

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Plot {
    label: char,
    x: usize,
    y: usize,
}

struct Region {
    plots: Vec<Plot>,
    neighbour_plots: Vec<Plot>,
}

impl Region {
    fn perimeter_len(&self, map: &Map) -> usize {
        self.neighbour_plots.len()
            + self
                .plots
                .iter()
                .map(|p| match (p.x, p.y) {
                    (0, 0) => 2,
                    (x, 0) if x == map.width - 1 => 2,
                    (0, y) if y == map.height - 1 => 2,
                    (0, _) => 1,
                    (_, 0) => 1,
                    (x, y) if x == map.width - 1 && y == map.height - 1 => 2,
                    (x, _) if x == map.width - 1 => 1,
                    (_, y) if y == map.height - 1 => 1,
                    _ => 0,
                })
                .sum::<usize>()
    }
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
    let mut used_plots: BTreeSet<Plot> = BTreeSet::new();

    for plot in map.plots.iter().flatten() {
        if !used_plots.contains(plot) {
            let region = find_region(&map, plot);
            used_plots.extend(&region.plots);
            regions.push(region);
        }
    }

    let part1_result = regions
        .iter()
        .map(|region| region.plots.len() * region.perimeter_len(&map))
        .sum::<usize>();

    println!("Day 12 Part 1: {}", part1_result);
}

fn find_region(map: &Map, plot: &Plot) -> Region {
    let mut plots_to_check = vec![*plot];
    let mut region_plots = BTreeSet::new();
    let mut region_perimeter = Vec::new();

    while let Some(p) = plots_to_check.pop() {
        region_plots.insert(p);

        for neighbor in map.neighbors(&p) {
            match neighbor.label {
                l if l == p.label => {
                    if !region_plots.contains(&neighbor) && !plots_to_check.contains(&neighbor) {
                        plots_to_check.push(neighbor);
                    }
                }
                _ => {
                    region_perimeter.push(neighbor);
                }
            }
        }
    }

    Region {
        plots: Vec::from_iter(region_plots),
        neighbour_plots: region_perimeter,
    }
}
