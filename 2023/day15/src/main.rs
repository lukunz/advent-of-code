use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Lens<'a> {
    label: &'a str,
    focal_length: u32,
}

enum Operation {
    Add,
    Remove,
}

fn calculate_hash(step: &str) -> u8 {
    step.chars()
        .map(|c| c as u64)
        .fold(0, |hash, c| ((hash + c) * 17) % 256) as u8
}

fn parse_step(step: &str) -> (&str, Operation, Option<u32>) {
    if let Some((label, focal_length)) = step.split_once('=') {
        (label, Operation::Add, focal_length.parse::<u32>().ok())
    } else if let Some((label, _)) = step.split_once('-') {
        (label, Operation::Remove, None)
    } else {
        panic!("Invalid input")
    }
}

fn sort_into_boxes(steps: Vec<&str>) -> HashMap<u8, Vec<Lens>> {
    let mut boxes: HashMap<u8, Vec<Lens>> = HashMap::new();

    for (label, operation, focal_length) in steps.iter().map(|step| parse_step(step)) {
        let hash = calculate_hash(label);
        match operation {
            Operation::Add => {
                let lens = Lens {
                    label,
                    focal_length: focal_length.unwrap(),
                };

                boxes.entry(hash).or_default();

                if let Some(lenses) = boxes.get_mut(&hash) {
                    if let Some(index) = lenses.iter().position(|l| l.label == lens.label) {
                        lenses[index].focal_length = lens.focal_length;
                    } else {
                        lenses.push(lens);
                    }
                }
            }
            Operation::Remove => {
                if let Some(lenses) = boxes.get_mut(&hash) {
                    if let Some(index) = lenses.iter().position(|l| l.label == label) {
                        lenses.remove(index);
                    }
                }
            }
        }
    }
    boxes
}

fn main() {
    let data = fs::read_to_string("day15.txt").expect("Can't read input file");

    let steps: Vec<&str> = data.split(',').map(|step| step.trim()).collect();

    let part1_result: u64 = steps.iter().map(|step| calculate_hash(step) as u64).sum();

    println!("Day 15 Part 1: {}", part1_result);

    let boxes = sort_into_boxes(steps);

    let part2_result: usize = boxes
        .iter()
        .map(|(&hash, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(index, lens)| (hash as usize + 1) * (index + 1) * lens.focal_length as usize)
                .sum::<usize>()
        })
        .sum();

    println!("Day 15 Part 1: {}", part2_result);
}
