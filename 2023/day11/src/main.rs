use std::fs;

fn main() {
    let data = fs::read_to_string("day11.txt").expect("Can't read input file");

    let map = parse_map(&data);
    let mut galaxies = parse_galaxies(&map);
    let (empty_cols, empty_rows) = find_empty_space(&map);

    expand_space(&mut galaxies, empty_cols, empty_rows);

    let mut paths_lengths: Vec<i32> = Vec::new();

    while let Some(galaxy) = &galaxies.pop() {
        for g in &galaxies {
            paths_lengths.push(shortest_path(galaxy, g));
        }
    }

    let part1_result: i32 = paths_lengths.iter().sum();

    println!("Day 11 Part 1: {}", part1_result);
}

fn parse_map(data: &str) -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = Vec::new();

    for line in data.lines() {
        map.push(line.chars().collect());
    }

    map
}

fn shortest_path(g1: &(i32, i32), g2: &(i32, i32)) -> i32 {
    (g2.0 - g1.0).abs() + (g2.1 - g1.1).abs()
}

fn expand_space(galaxies: &mut Vec<(i32, i32)>, empty_cols: Vec<i32>, empty_rows: Vec<i32>) {
    let mut offset = 0;
    for x in empty_cols {
        for index in 0..galaxies.len() {
            if galaxies[index].0 > x + offset {
                galaxies[index].0 += 1;
            }
        }
        offset += 1;
    }

    offset = 0;
    for y in empty_rows {
        for index in 0..galaxies.len() {
            if galaxies[index].1 > y + offset {
                galaxies[index].1 += 1;
            }
        }
        offset += 1;
    }
}

fn find_empty_space(map: &[Vec<char>]) -> (Vec<i32>, Vec<i32>) {
    let width = map[0].len();
    let mut empty_cols: Vec<i32> = Vec::new();

    for x in 0..width {
        let mut is_empty = true;
        for row in map.iter() {
            is_empty &= row[x] == '.'
        }
        if is_empty {
            empty_cols.push(x as i32);
        }
    }

    let mut empty_rows: Vec<i32> = Vec::new();

    for (y, row) in map.iter().enumerate() {
        if row.iter().all(|c| *c == '.') {
            empty_rows.push(y as i32);
        }
    }

    (empty_cols, empty_rows)
}

fn parse_galaxies(map: &[Vec<char>]) -> Vec<(i32, i32)> {
    let mut galaxies: Vec<(i32, i32)> = Vec::new();

    for (y, row) in map.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == '#' {
                galaxies.push((x as i32, y as i32));
            }
        }
    }
    galaxies
}
