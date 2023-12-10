use std::fs;

fn parse_number_list(number_str: &str) -> Vec<i64> {
    number_str
        .split_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect()
}

fn diff_list(numbers: &[i64]) -> Vec<i64> {
    numbers.windows(2).map(|n| n[1] - n[0]).collect()
}

fn is_all_zero(numbers: &[i64]) -> bool {
    numbers.iter().all(|n| *n == 0)
}

fn find_next_number(numbers: Vec<i64>) -> i64 {
    let mut numbers_stack: Vec<Vec<i64>> = Vec::new();
    let mut index = 0;
    numbers_stack.push(numbers);

    while !is_all_zero(&numbers_stack[index]) {
        let diff_list = diff_list(&numbers_stack[index]);
        numbers_stack.push(diff_list);
        index += 1;
    }

    numbers_stack
        .iter()
        .rev()
        .fold(0, |acc, n| n.last().unwrap() + acc)
}

fn main() {
    let data = fs::read_to_string("day9.txt").expect("Can't read input file");

    let part1_result: i64 = data
        .lines()
        .map(parse_number_list)
        .map(find_next_number)
        .sum();

    println!("Day 9 Part 1: {}", part1_result);
}
