fn main() {
    let data = include_str!("../day10.txt");

    let mut stones = data
        .split_whitespace()
        .map(|value| value.parse::<u64>().expect("Invalid input"))
        .collect::<Vec<_>>();

    for _ in 0..25 {
        let mut new_stones = Vec::new();

        for stone in stones.iter() {
            match stone {
                0 => new_stones.push(1),
                s => {
                    let digits = number_of_digits(*s);
                    if digits % 2 == 0 {
                        let power = 10u64.pow(digits as u32 / 2);
                        let upper = s / power;
                        let lower = s - upper * power;

                        new_stones.push(upper);
                        new_stones.push(lower);
                    } else {
                        new_stones.push(*s * 2024);
                    }
                }
            }
        }

        stones = new_stones;
    }

    println!("Day 11 Part 1: {}", stones.len());
}

fn number_of_digits(number: u64) -> u64 {
    (0..).take_while(|i| 10u64.pow(*i) <= number).count() as u64
}
