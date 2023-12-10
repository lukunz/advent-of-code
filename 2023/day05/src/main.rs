use std::fs;
use std::str::Lines;

#[derive(Clone, PartialEq, Debug)]
struct Range {
    start: usize,
    len: usize,
}

impl Range {
    fn new(start: usize, len: usize) -> Self {
        Range { start, len }
    }

    fn from_endpoints(start: usize, end: usize) -> Self {
        Range {
            start,
            len: end - start,
        }
    }

    fn end(&self) -> usize {
        self.start + self.len
    }
    fn contains(&self, value: usize) -> bool {
        value >= self.start && value < self.start + self.len
    }

    fn intersect(&self, other: &Range) -> Option<Range> {
        if self.contains(other.start) {
            Some(Range::from_endpoints(other.start, self.end()))
        } else if self.contains(other.end()) {
            Some(Range::from_endpoints(self.start, other.end()))
        } else {
            None
        }
    }

    fn diff(&self, other: &Range) -> Vec<Range> {
        let mut result: Vec<Range> = Vec::new();

        if self.start > other.end() || other.start > self.end() {
            result.push(other.clone());

            return result;
        }

        if self.start > other.start {
            result.push(Range::from_endpoints(other.start, self.start - 1));
        }

        if self.end() < other.end() {
            result.push(Range::from_endpoints(self.end(), other.end()));
        }

        result
    }
}

struct Mapping {
    src: Range,
    dest: Range,
}

impl Mapping {
    fn map(&self, value: usize) -> usize {
        if self.src.contains(value) {
            self.dest.start + (value - self.src.start)
        } else {
            value
        }
    }

    fn map_range(&self, range: &Range) -> Option<(Range, Vec<Range>)> {
        if let Some(mut intersect) = self.src.intersect(range) {
            intersect.start = self.dest.start + (intersect.start - self.src.start);
            let new_ranges = self.src.diff(range);

            Some((intersect, new_ranges))
        } else {
            None
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
            src: Range {
                start: numbers[1],
                len: numbers[2],
            },
            dest: Range {
                start: numbers[0],
                len: numbers[2],
            },
        });
    }

    if inside_mapping && !mappings.is_empty() {
        almanac.push(mappings);
    }

    almanac
}

fn apply_mapping(x: usize, mappings: &Vec<Mapping>) -> usize {
    for m in mappings {
        if m.src.contains(x) {
            return m.map(x);
        }
    }

    x
}

fn apply_range_mapping(ranges: Vec<Range>, mappings: &Vec<Mapping>) -> Vec<Range> {
    let mut ranges = ranges;
    let mut mapped_ranges: Vec<Range> = Vec::new();

    for m in mappings {
        let mut old_ranges: Vec<Range> = Vec::new();

        while let Some(range) = ranges.pop() {
            if let Some((mapped_range, mut new_ranges)) = m.map_range(&range) {
                mapped_ranges.push(mapped_range);
                old_ranges.append(&mut new_ranges);
            } else {
                old_ranges.push(range);
            }
        }

        ranges = old_ranges;
    }

    ranges.append(&mut mapped_ranges);

    ranges
}

fn get_location(seed: usize, almanac: &Almanac) -> usize {
    almanac.iter().fold(seed, apply_mapping)
}

fn get_range_location(seed: Range, almanac: &Almanac) -> Vec<Range> {
    almanac.iter().fold(vec![seed], apply_range_mapping)
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
        .map(|range| Range::new(range[0], range[1]))
        .map(|seed| get_range_location(seed, &almanac))
        .map(|ranges| ranges.iter().map(|range| range.start).min().unwrap())
        .min()
        .unwrap();

    println!("Day 5 Part 1: {}", part1_result);
    println!("Day 5 Part 2: {}", part2_result);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_intersect() {
        let range1 = Range::from_endpoints(4, 8);
        let range2 = Range::from_endpoints(5, 10);
        let range3 = Range::from_endpoints(11, 12);

        assert_eq!(Some(Range::from_endpoints(5, 8)), range1.intersect(&range2));
        assert_eq!(None, range1.intersect(&range3));
    }

    #[test]
    fn test_range_diff() {
        // no intersection
        let range1 = Range::from_endpoints(1, 3);
        let range2 = Range::from_endpoints(5, 10);

        assert_eq!(1, range1.diff(&range2).len());
        assert!(range1.diff(&range2).contains(&Range::from_endpoints(5, 10)));

        // range1 inside range2
        let range1 = Range::from_endpoints(8, 9);
        let range2 = Range::from_endpoints(5, 10);

        assert_eq!(2, range1.diff(&range2).len());
        assert!(range1.diff(&range2).contains(&Range::from_endpoints(5, 7)));
        assert!(range1.diff(&range2).contains(&Range::from_endpoints(9, 10)));

        // range2 inside range1
        let range1 = Range::from_endpoints(5, 10);
        let range2 = Range::from_endpoints(8, 9);

        assert_eq!(0, range1.diff(&range2).len());

        // end of range2 inside range1
        let range1 = Range::from_endpoints(5, 10);
        let range2 = Range::from_endpoints(1, 9);

        assert_eq!(1, range1.diff(&range2).len());
        assert!(range1.diff(&range2).contains(&Range::from_endpoints(1, 4)));

        // start of range2 inside range1
        let range1 = Range::from_endpoints(5, 10);
        let range2 = Range::from_endpoints(7, 14);

        assert_eq!(1, range1.diff(&range2).len());
        assert!(range1
            .diff(&range2)
            .contains(&Range::from_endpoints(10, 14)));
    }
}
