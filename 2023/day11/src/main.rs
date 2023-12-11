use std::fs;

#[derive(Clone)]
struct Map {
    galaxies: Vec<(i64, i64)>,
    empty_cols: Vec<i64>,
    empty_rows: Vec<i64>,
}

impl Map {
    fn expand_space(&self, factor: i64) -> Map {
        let new_galaxies = self
            .galaxies
            .iter()
            .map(|g| {
                let col_count = self.empty_cols.iter().filter(|x| *x < &g.0).count() as i64;
                let row_count = self.empty_rows.iter().filter(|y| *y < &g.1).count() as i64;

                (
                    if col_count != 0 {
                        g.0 + col_count * (factor - 1)
                    } else {
                        g.0
                    },
                    if row_count != 0 {
                        g.1 + row_count * (factor - 1)
                    } else {
                        g.1
                    },
                )
            })
            .collect();

        Map {
            galaxies: new_galaxies,
            empty_rows: self.empty_rows.clone(),
            empty_cols: self.empty_cols.clone(),
        }
    }
}

fn main() {
    let data = fs::read_to_string("day11.txt").expect("Can't read input file");

    let map = parse_map(&data);

    let part1_result = calculate_paths(map.expand_space(2));
    let part2_result = calculate_paths(map.expand_space(1_000_000));

    println!("Day 11 Part 1: {}", part1_result);
    println!("Day 11 Part 2: {}", part2_result);
}

fn calculate_paths(mut map: Map) -> i64 {
    let mut paths_lengths: Vec<i64> = Vec::new();

    while let Some(galaxy) = &map.galaxies.pop() {
        for g in &map.galaxies {
            paths_lengths.push(shortest_path(galaxy, g));
        }
    }

    let part1_result: i64 = paths_lengths.iter().sum();
    part1_result
}

fn parse_map(data: &str) -> Map {
    let mut map: Vec<Vec<char>> = Vec::new();

    for line in data.lines() {
        map.push(line.chars().collect());
    }

    let galaxies = parse_galaxies(&map);
    let (empty_cols, empty_rows) = find_empty_space(&map);

    Map {
        galaxies,
        empty_cols,
        empty_rows,
    }
}

fn shortest_path(g1: &(i64, i64), g2: &(i64, i64)) -> i64 {
    (g2.0 - g1.0).abs() + (g2.1 - g1.1).abs()
}

fn find_empty_space(map: &[Vec<char>]) -> (Vec<i64>, Vec<i64>) {
    let width = map[0].len();
    let mut empty_cols: Vec<i64> = Vec::new();

    for x in 0..width {
        if map.iter().map(|row| row[x]).all(|c| c == '.') {
            empty_cols.push(x as i64);
        }
    }

    let mut empty_rows: Vec<i64> = Vec::new();

    for (y, row) in map.iter().enumerate() {
        if row.iter().all(|&c| c == '.') {
            empty_rows.push(y as i64);
        }
    }

    (empty_cols, empty_rows)
}

fn parse_galaxies(map: &[Vec<char>]) -> Vec<(i64, i64)> {
    let mut galaxies: Vec<(i64, i64)> = Vec::new();

    for (y, row) in map.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == '#' {
                galaxies.push((x as i64, y as i64));
            }
        }
    }
    galaxies
}
