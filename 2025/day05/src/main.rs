use std::{cmp::max, cmp::min, collections::HashSet, ops::RangeInclusive};

fn parse_input(data: &str) -> (Vec<RangeInclusive<usize>>, Vec<usize>) {
    let (range_data, id_data) = data.split_once("\n\n").unwrap();

    let ranges: Vec<RangeInclusive<usize>> = range_data
        .lines()
        .map(|line| {
            let (lo, hi) = line.split_once('-').unwrap();
            lo.parse::<usize>().unwrap()..=hi.parse::<usize>().unwrap()
        })
        .collect();

    let ids: Vec<usize> = id_data
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect();

    (ranges, ids)
}

fn intersect(
    r1: &RangeInclusive<usize>,
    r2: &RangeInclusive<usize>,
) -> Option<RangeInclusive<usize>> {
    if r1.contains(r2.start()) || r2.contains(r1.start()) {
        return Some(min(*r1.start(), *r2.start())..=max(*r1.end(), *r2.end()));
    }

    None
}

fn main() {
    let data = include_str!("../day05.txt");

    let (mut ranges, ids) = parse_input(data);

    let mut final_ranges: Vec<RangeInclusive<usize>> = Vec::new();

    let mut fresh_ids: HashSet<usize> = HashSet::new();

    let mut changed = true;

    while changed {
        changed = false;
        for range in ranges.drain(..) {
            let mut found = false;
            for i in 0..final_ranges.len() {
                if let Some(new_range) = intersect(&range, &final_ranges[i]) {
                    final_ranges[i] = new_range;
                    found = true;
                    changed = true;
                    break;
                }
            }
            if !found {
                final_ranges.push(range);
            }
        }

        let tmp = ranges;
        ranges = final_ranges;
        final_ranges = tmp;
    }

    for id in ids {
        for range in &ranges {
            if range.contains(&id) {
                fresh_ids.insert(id);
                break;
            }
        }
    }

    println!("Day 05 Part 1: {}", fresh_ids.len());

    let id_count = ranges
        .iter()
        .map(|range| range.end() - range.start() + 1)
        .sum::<usize>();

    println!("Day 05 Part 2: {}", id_count);
}
