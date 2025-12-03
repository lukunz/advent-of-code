fn find_largest(bank: &[char]) -> (char, usize) {
    assert!(!bank.is_empty());
    let mut largest = bank[0];
    let mut index = 0;

    for (i, &bat) in bank.iter().enumerate() {
        if bat > largest {
            largest = bat;
            index = i;
        }
    }

    (largest, index)
}

fn main() {
    let data = include_str!("../day03.txt");

    let banks: Vec<Vec<char>> = data
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let mut sum = 0;

    for bank in &banks {
        assert!(bank.len() >= 2);
        let (first, index) = find_largest(&bank[..bank.len() - 1]);
        let (second, _) = find_largest(&bank[index + 1..]);

        sum += first.to_digit(10).unwrap() * 10 + second.to_digit(10).unwrap();
    }

    println!("Day 03 Part 1: {}", sum);
}
