use std::collections::HashMap;

fn main() {
    let data = include_str!("../day19.txt");

    let (towels_input, designs_input) = data.split_once("\n\n").expect("Input in wrong format");

    let towels = towels_input.split(", ").collect::<Vec<&str>>();
    let designs = designs_input.lines().collect::<Vec<&str>>();

    let part1_result = designs
        .iter()
        .filter(|d| is_design_valid(d, &towels))
        .count();

    println!("Day 19 Part 1: {}", part1_result);

    let mut cache: HashMap<&str, usize> = HashMap::new();

    let part2_result = designs
        .iter()
        .filter(|d| is_design_valid(d, &towels))
        .map(|d| count_possible_arrangements(d, &towels, &mut cache))
        .sum::<usize>();

    println!("Day 19 Part 2: {}", part2_result);
}

fn is_design_valid(design: &str, towels: &[&str]) -> bool {
    if design.is_empty() {
        return true;
    }

    for towel in towels {
        if let Some(sub_design) = design.strip_prefix(towel) {
            if is_design_valid(sub_design, towels) {
                return true;
            }
        }
    }

    false
}

fn count_possible_arrangements<'a>(
    design: &'a str,
    towels: &[&str],
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    }

    let mut count = 0;

    for towel in towels {
        if let Some(sub_design) = design.strip_prefix(towel) {
            if let Some(c) = cache.get(sub_design) {
                count += c;
            } else {
                let c = count_possible_arrangements(sub_design, towels, cache);
                cache.insert(sub_design, c);
                count += c;
            }
        }
    }

    count
}
