use std::fs;
use std::str::{FromStr, Lines};

struct Mapping {
    src: usize,
    dest: usize,
    len: usize,
}

#[derive(PartialEq, Eq, Hash)]
enum Resource {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl FromStr for Resource {
    type Err = String;

    fn from_str<'a>(s: &str) -> Result<Self, Self::Err> {
        match s {
            "seed" => Ok(Self::Seed),
            "soil" => Ok(Self::Soil),
            "fertilizer" => Ok(Self::Fertilizer),
            "water" => Ok(Self::Water),
            "light" => Ok(Self::Light),
            "temperature" => Ok(Self::Temperature),
            "humidity" => Ok(Self::Humidity),
            "location" => Ok(Self::Location),
            _ => Err(format!("Unknown resource '{}'", s)),
        }
    }
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
    let mut to: Option<Resource> = None;
    let mut mappings: Vec<Mapping> = Vec::new();
    let mut almanac: Almanac = Vec::new();

    for line in lines {
        if line.is_empty() {
            if to.is_some() && !mappings.is_empty() {
                almanac.push(mappings);
                to = None;
                mappings = Vec::new();
            }

            continue;
        }

        if line.ends_with(':') {
            let (mapping_str, _) = line.split_once(' ').unwrap();
            let (_, to_str) = mapping_str.split_once("-to-").unwrap();

            to = Resource::from_str(to_str).ok();

            continue;
        }

        let numbers = parse_number_list(line);
        mappings.push(Mapping {
            src: numbers[1],
            dest: numbers[0],
            len: numbers[2],
        });
    }

    if to.is_some() && !mappings.is_empty() {
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

    println!("Day 5 Part 1: {}", part1_result);
}
