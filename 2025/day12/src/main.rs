#[derive(Debug)]
struct Area {
    size: usize,
    required_shapes: Vec<usize>,
}

fn parse_areas(input: &str) -> Vec<Area> {
    let data = input
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(area_str, shape_str)| (area_str.split_once("x").unwrap(), shape_str));

    let mut areas = Vec::new();

    for ((width, height), shapes) in data {
        let width = width.parse::<usize>().unwrap();
        let height = height.parse::<usize>().unwrap();
        let shapes = shapes
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        areas.push(Area {
            size: width * height,
            required_shapes: shapes,
        });
    }

    areas
}

fn parse_shapes(input: &[&str]) -> Vec<usize> {
    let mut shapes = Vec::new();

    for segment in input {
        let used_area = segment
            .lines()
            .skip(1)
            .map(|line| line.chars().filter(|c| *c == '#').count())
            .sum();

        shapes.push(used_area);
    }

    shapes
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<Area>) {
    let mut segments: Vec<&str> = input.split("\n\n").collect();
    let area_input = segments.pop().unwrap();

    (parse_shapes(&segments), parse_areas(area_input))
}

fn main() {
    let input = include_str!("../day12.txt");

    let (shapes, areas) = parse_input(input);

    let mut count = 0;

    for area in &areas {
        let shape_area: usize = area
            .required_shapes
            .iter()
            .enumerate()
            .map(|(index, mul)| shapes[index] * mul)
            .sum();

        if shape_area <= area.size {
            count += 1;
        }
    }

    println!("Day 12 Part 1: {}", count);
}
