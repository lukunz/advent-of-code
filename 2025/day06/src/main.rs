fn parse_input(input: &str) -> Vec<(Vec<u64>, char)> {
    let first_line = input.lines().next().unwrap();

    let col_count = first_line.split_whitespace().count();

    let mut data: Vec<(Vec<u64>, char)> = Vec::with_capacity(col_count);

    for _ in 0..col_count {
        data.push((Vec::new(), ' '));
    }

    for line in input.lines() {
        let cols = line.split_whitespace();
        for (index, col) in cols.enumerate() {
            if col == "+" || col == "*" {
                data[index].1 = col.chars().next().unwrap();
            } else {
                data[index].0.push(col.parse::<u64>().unwrap());
            }
        }
    }

    data
}

fn main() {
    let input = include_str!("../day06.txt");

    let data = parse_input(input);

    let result_part1: u64 = data
        .iter()
        .map(|(nums, op)| match op {
            '+' => nums.iter().sum(),
            '*' => nums.iter().product(),
            _ => 0,
        })
        .sum();

    println!("Day 06 Part 1: {}", result_part1);
}
