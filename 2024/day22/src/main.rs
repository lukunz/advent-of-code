use std::collections::HashMap;

type Seq = (isize, isize, isize, isize);

fn main() {
    let data = include_str!("../day22.txt");

    let number_of_secrets = 2000;
    let init_secrets: Vec<usize> = data.lines().map(|line| line.parse().unwrap()).collect();

    let part1_result = init_secrets
        .iter()
        .map(|&s| nth_secret(s, number_of_secrets))
        .sum::<usize>();

    println!("Day 22 Part 1: {}", part1_result);

    let mut scores: HashMap<Seq, usize> = HashMap::new();
    let mut secrets: Vec<usize> = Vec::with_capacity(number_of_secrets + 1);
    let mut changes: Vec<isize> = Vec::with_capacity(number_of_secrets);
    let mut found_seq: Vec<Seq> = Vec::with_capacity(number_of_secrets / 4);

    for &secret in &init_secrets {
        secrets.clear();
        changes.clear();
        found_seq.clear();

        secrets.push(secret % 10);
        let mut secret = secret;
        for _ in 0..number_of_secrets {
            secret = next_secret(secret);
            secrets.push(secret % 10);
        }

        for w in secrets.windows(2) {
            changes.push(w[1] as isize - w[0] as isize);
        }

        for (index, w) in changes.windows(4).enumerate() {
            let seq = (w[0], w[1], w[2], w[3]);

            if !found_seq.contains(&seq) {
                let entry = scores.entry(seq).or_default();
                *entry += secrets[index + 4];
                found_seq.push(seq);
            }
        }
    }

    let max_score = scores.values().max().unwrap();

    println!("Day 22 Part 2: {}", max_score);
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
