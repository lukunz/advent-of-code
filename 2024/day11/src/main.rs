use std::collections::BTreeMap;

enum BlinkResult {
    One(u64),
    Two(u64, u64),
}

fn main() {
    let data = include_str!("../day10.txt");

    let stones = data
        .split_whitespace()
        .map(|value| value.parse::<u64>().expect("Invalid input"))
        .collect::<Vec<_>>();

    let mut stone_map: BTreeMap<(u64, u64), u64> = BTreeMap::new();

    println!(
        "Day 11 Part 1: {}",
        calculate_stone_count_sum(&stones, &mut stone_map, 25)
    );
    println!(
        "Day 11 Part 2: {}",
        calculate_stone_count_sum(&stones, &mut stone_map, 75)
    );
}

fn calculate_stone_count_sum(
    stones: &[u64],
    stone_map: &mut BTreeMap<(u64, u64), u64>,
    blinks: u64,
) -> u64 {
    stones
        .iter()
        .map(|stone| calculate_stone_count(*stone, stone_map, blinks))
        .sum::<u64>()
}

fn calculate_stone_count(
    stone: u64,
    stone_map: &mut BTreeMap<(u64, u64), u64>,
    blinks: u64,
) -> u64 {
    if blinks == 0 {
        return 1;
    }

    if let Some(count) = stone_map.get(&(stone, blinks)) {
        return *count;
    }

    let count = match blink(stone) {
        BlinkResult::One(new_stone) => calculate_stone_count(new_stone, stone_map, blinks - 1),
        BlinkResult::Two(new_stone_a, new_stone_b) => {
            calculate_stone_count(new_stone_a, stone_map, blinks - 1)
                + calculate_stone_count(new_stone_b, stone_map, blinks - 1)
        }
    };

    stone_map.insert((stone, blinks), count);

    count
}

fn blink(number: u64) -> BlinkResult {
    match (number, number_of_digits(number)) {
        (0, _) => BlinkResult::One(1),
        (s, digits) if digits % 2 == 0 => {
            let power = 10u64.pow(digits as u32 / 2);
            let upper = s / power;
            let lower = s - upper * power;

            BlinkResult::Two(upper, lower)
        }
        (s, _) => BlinkResult::One(s * 2024),
    }
}

fn number_of_digits(number: u64) -> u64 {
    (0..).take_while(|i| 10u64.pow(*i) <= number).count() as u64
}
