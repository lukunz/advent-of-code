fn main() {
    let data = include_str!("../day22.txt");

    let init_secrets: Vec<usize> = data.lines().map(|line| line.parse().unwrap()).collect();

    let part1_result = init_secrets
        .iter()
        .map(|&s| nth_secret(s, 2000))
        .sum::<usize>();

    println!("Day 22 Part 1: {}", part1_result);
}

fn nth_secret(secret: usize, n: usize) -> usize {
    let mut secret = secret;
    for _ in 0..n {
        secret = next_secret(secret);
    }

    secret
}

fn next_secret(secret: usize) -> usize {
    let secret = (secret ^ (secret * 64)) % 16777216;
    let secret = (secret ^ (secret / 32)) % 16777216;
    (secret ^ (secret * 2048)) % 16777216
}
