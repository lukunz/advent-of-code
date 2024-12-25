use std::collections::HashMap;

enum Op {
    And(String, String),
    Xor(String, String),
    Or(String, String),
}

fn main() {
    let data = include_str!("../day24.txt");

    let (value_lines, ops_lines) = data.split_once("\n\n").unwrap();

    let mut values = value_lines
        .lines()
        .map(|line| {
            let (name, value) = line.split_once(": ").unwrap();
            (name.to_string(), value.parse::<u8>().unwrap())
        })
        .collect::<HashMap<String, u8>>();

    let ops = ops_lines
        .lines()
        .map(|line| {
            let parts = line.split(' ').collect::<Vec<&str>>();
            assert_eq!(parts.len(), 5);

            match parts[1] {
                "AND" => (
                    parts[4].to_string(),
                    Op::And(parts[0].to_string(), parts[2].to_string()),
                ),
                "XOR" => (
                    parts[4].to_string(),
                    Op::Xor(parts[0].to_string(), parts[2].to_string()),
                ),
                "OR" => (
                    parts[4].to_string(),
                    Op::Or(parts[0].to_string(), parts[2].to_string()),
                ),
                _ => panic!("Unknown op: {}", parts[1]),
            }
        })
        .collect::<HashMap<String, Op>>();

    let part1_result = ops
        .keys()
        .filter_map(|name| {
            if !name.starts_with("z") {
                return None;
            }

            let value = resolve(name, &mut values, &ops);

            let shift = name.trim_start_matches('z').parse::<usize>().unwrap();

            Some((value as usize) << shift)
        })
        .sum::<usize>();

    println!("Day 24 Part 1: {}", part1_result);
}

fn resolve(name: &str, values: &mut HashMap<String, u8>, ops: &HashMap<String, Op>) -> u8 {
    if values.contains_key(name) {
        return values[name];
    }

    match ops.get(name).unwrap() {
        Op::And(lhs, rhs) => {
            let l_value = resolve(lhs, values, ops);
            let r_value = resolve(rhs, values, ops);
            let value = l_value & r_value;
            values.insert(name.to_string(), value);
            value
        }
        Op::Xor(lhs, rhs) => {
            let l_value = resolve(lhs, values, ops);
            let r_value = resolve(rhs, values, ops);
            let value = l_value ^ r_value;
            values.insert(name.to_string(), value);
            value
        }
        Op::Or(lhs, rhs) => {
            let l_value = resolve(lhs, values, ops);
            let r_value = resolve(rhs, values, ops);
            let value = l_value | r_value;
            values.insert(name.to_string(), value);
            value
        }
    }
}
