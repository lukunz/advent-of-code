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
}
