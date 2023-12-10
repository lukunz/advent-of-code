use regex::Regex;
use std::cmp::max;
use std::collections::HashMap;
use std::fs;

fn prime_factors(mut n: usize) -> HashMap<usize, usize> {
    let mut factors: Vec<usize> = Vec::new();

    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }

    for i in 3..(n as f64).sqrt() as usize {
        while n % i == 0 {
            factors.push(i);
            n /= i;
        }
    }

    if n > 2 {
        factors.push(n);
    }

    let mut result: HashMap<usize, usize> = HashMap::new();

    for f in factors {
        if let Some(f_count) = result.get(&f) {
            result.insert(f, f_count + 1);
        } else {
            result.insert(f, 1);
        }
    }

    result
}

fn find_lcm(numbers: &[usize]) -> usize {
    let mut all_factors: HashMap<usize, usize> = HashMap::new();

    for factors in numbers.iter().copied().map(prime_factors) {
        for (factor, f_count) in factors {
            if let Some(f_count2) = all_factors.get(&factor) {
                all_factors.insert(factor, max(f_count, *f_count2));
            } else {
                all_factors.insert(factor, f_count);
            }
        }
    }

    all_factors
        .iter()
        .map(|(factor, count)| factor.pow(*count as u32))
        .product()
}

fn main() {
    let data = fs::read_to_string("day8.txt").expect("Can't read input file");

    let (directions, map) = parse_input(&data);
    println!(
        "Day 8 Part 1: {}",
        find_end_location("AAA", &directions, &map)
    );
    println!("Day 8 Part 2: {}", part2(&directions, &map));
}

fn find_end_location(
    start_location: &str,
    directions: &Vec<char>,
    map: &HashMap<&str, (&str, &str)>,
) -> usize {
    let mut current_location = start_location;
    let mut step_count = 0;

    while !current_location.ends_with('Z') {
        current_location = run_directions(directions, map, current_location);
        step_count += directions.len();
    }

    step_count
}

fn part2(directions: &Vec<char>, map: &HashMap<&str, (&str, &str)>) -> usize {
    let step_counts: Vec<usize> = map
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|start| find_end_location(start, directions, map))
        .collect();

    find_lcm(&step_counts)
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
