use rayon::prelude::*;
use std::fs;
use std::str::Lines;

struct Mapping {
    src: usize,
    dest: usize,
    len: usize,
}

fn parse_number_list(number_str: &str) -> Vec<usize> {
    number_str
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect()
}

fn parse_seeds(seed_str: &str) -> Vec<usize> {
    let (_, number_str) = seed_str.split_once(": ").unwrap();

    parse_number_list(number_str)
}

type Almanac = Vec<Vec<Mapping>>;

fn parse_almanac(lines: &mut Lines) -> Almanac {
    let mut inside_mapping = false;
    let mut mappings: Vec<Mapping> = Vec::new();
    let mut almanac: Almanac = Vec::new();

    for line in lines {
        if line.is_empty() {
            if inside_mapping && !mappings.is_empty() {
                almanac.push(mappings);
                inside_mapping = false;
                mappings = Vec::new();
            }

            continue;
        }

        if line.ends_with(':') {
            inside_mapping = true;

            continue;
        }

        let numbers = parse_number_list(line);
        mappings.push(Mapping {
            src: numbers[1],
            dest: numbers[0],
            len: numbers[2],
        });
    }

    if inside_mapping && !mappings.is_empty() {
        almanac.push(mappings);
    }

    almanac
}

fn apply_mapping(x: usize, mappings: &Vec<Mapping>) -> usize {
    for m in mappings {
        if (m.src..m.src + m.len).contains(&x) {
            return m.dest + (x - m.src);
        }
    }

    x
}

fn get_location(seed: usize, almanac: &Almanac) -> usize {
    almanac.iter().fold(seed, apply_mapping)
}

fn main() {
    let data = fs::read_to_string("day5.txt").expect("Can't read input file");
    let mut lines = data.lines();

    let seeds = parse_seeds(lines.next().unwrap());
    let almanac = parse_almanac(&mut lines);

    let part1_result = seeds
        .iter()
        .map(|seed| get_location(*seed, &almanac))
        .min()
        .unwrap();

    let part2_result = seeds
        .chunks(2)
        .map(|range| {
            (range[0]..range[0] + range[1])
                .into_par_iter()
                .map(|seed| get_location(seed, &almanac))
                .min()
                .unwrap()
        })
        .min()
        .unwrap();

    println!("Day 5 Part 1: {}", part1_result);
    println!("Day 5 Part 2: {}", part2_result);
}
