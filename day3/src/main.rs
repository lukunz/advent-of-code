use std::collections::HashSet;
use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").expect("Can't read input file");
    let rucksacks: u32 = data.lines()
        .map(|line| {
            let (left, right): (&str, &str) = line.split_at(line.len() / 2);
            let left: HashSet<char> = HashSet::from_iter(left.chars());
            let right: HashSet<char> = HashSet::from_iter(right.chars());

            let intersect = left.intersection(&right);

            intersect.map(|c| match c {
                'a'..='z' => c.clone() as u32 - 96,
                'A'..='Z' => c.clone() as u32 - 64 + 26,
                _ => 0
            }).sum::<u32>()
        }).sum();

    println!("{rucksacks}");
}
