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

    let mut sum = 0_u64;

    for bank in &banks {
        assert!(bank.len() >= 2);
        let (first, index) = find_largest(&bank[..bank.len() - 1]);
        let (second, _) = find_largest(&bank[index + 1..]);

        sum += first.to_digit(10).unwrap() as u64 * 10 + second.to_digit(10).unwrap() as u64;
    }

    println!("Day 03 Part 1: {}", sum);

    sum = 0;

    let mut bats: Vec<char> = Vec::new();
    let mut current_index;

    for bank in &banks {
        bats.clear();
        current_index = 0;

        for offset in (0..12).rev() {
            let (bat, index) = find_largest(&bank[current_index..bank.len() - offset]);
            bats.push(bat);
            current_index += index + 1;
        }

        sum += bats.iter().collect::<String>().parse::<u64>().unwrap();
    }

    println!("Day 03 Part 2: {}", sum);
}
