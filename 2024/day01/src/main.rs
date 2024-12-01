use std::collections::HashMap;

fn main() {
    let data = include_str!("../day01.txt");

    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in data.lines() {
        let split = line.split("   ").collect::<Vec<&str>>();
        assert_eq!(split.len(), 2);

        list1.push(split[0].parse::<i32>().unwrap());
        list2.push(split[1].parse::<i32>().unwrap());
    }

    list1.sort();
    list2.sort();

    let part1_result = list1
        .iter()
        .zip(list2.iter())
        .map(|(v1, v2)| (v1 - v2).abs())
        .sum::<i32>();

    println!("Day 01 Part 1: {}", part1_result);

    let mut number_counts: HashMap<i32, i32> = HashMap::new();

    for value in list2 {
        let count = number_counts.get(&value).unwrap_or(&0) + 1;
        number_counts.insert(value, count);
    }

    let mut sum = 0;

    for value in list1 {
        let count = number_counts.get(&value).unwrap_or(&0);
        sum += value * count;
    }

    println!("Day 01 Part 2: {}", sum);
}
