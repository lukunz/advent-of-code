use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn main() {
    let data = fs::read_to_string("day8.txt").expect("Can't read input file");

    let (directions, map) = parse_input(&data);
    let mut current_location = "AAA";
    let mut step_count = 0;

    while current_location != "ZZZ" {
        current_location = run_directions(&directions, &map, current_location);
        step_count += directions.len();
    }

    println!("Day 8 Part 1: {}", step_count);
}

fn parse_input(data: &str) -> (Vec<char>, HashMap<&str, (&str, &str)>) {
    let mut lines = data.lines();
    let directions: Vec<char> = lines.next().unwrap().chars().collect();
    lines.next();

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();

    for line in lines {
        let r = Regex::new(r"([A-Z]*) = \(([A-Z]*), ([A-Z]*)\)").unwrap();
        let (_, [start, l_dest, r_dest]) = r.captures(line).unwrap().extract();
        map.insert(start, (l_dest, r_dest));
    }

    (directions, map)
}

fn run_directions<'a>(
    directions: &[char],
    map: &HashMap<&'a str, (&'a str, &'a str)>,
    start_location: &'a str,
) -> &'a str {
    let mut current_location = start_location;

    for d in directions {
        let (l_dest, r_dest) = map.get(current_location).unwrap();
        current_location = match d {
            'L' => l_dest,
            'R' => r_dest,
            _ => panic!("Unknown direction '{}'", d),
        };
    }

    current_location
}
