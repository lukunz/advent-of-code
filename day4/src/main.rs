use std::fs;

type Pair = (u32, u32);
type Group = (Pair, Pair);

fn parse_item(item: &str) -> Option<Pair> {
    let parts: Vec<&str> = item.split("-").collect();

    if parts.len() < 2 {
        return None;
    }

    Some((
        parts[0].parse().ok()?,
        parts[1].parse().ok()?)
    )
}

fn parse_line(line: &str) -> Option<Group> {
    let items: Vec<(u32, u32)> = line.split(",")
        .filter_map(|item| parse_item(item))
        .collect();

    if items.len() < 2 {
        return None;
    }

    Some((items[0], items[1]))
}

fn check_contains(group: &Group) -> bool {
    group.0.0 >= group.1.0 && group.0.1 <= group.1.1
    || group.0.0 <= group.1.0 && group.0.1 >= group.1.1
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("Can't read input file");

    let number_of_fully_contained_groups = data.lines()
        .filter_map(|line| parse_line(line))
        .filter(|&group| check_contains(&group))
        .collect::<Vec<Group>>()
        .len();



    println!("{number_of_fully_contained_groups}");
}
