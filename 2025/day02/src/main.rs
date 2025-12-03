fn split_to_number(data: &str, split: usize) -> (u64, u64) {
    let (hi, lo) = data.split_at(split);

    (hi.parse::<u64>().unwrap(), lo.parse::<u64>().unwrap())
}

fn calculate_id(n: u64) -> u64 {
    10_u64.pow(n.checked_ilog10().unwrap_or(0) + 1) * n + n
}

fn main() {
    let data = include_str!("../day02.txt").trim();

    let ranges = data
        .split(',')
        .map(|range| range.split_once('-').unwrap())
        .collect::<Vec<(&str, &str)>>();

    let mut sum: u64 = 0;

    let mut valid_ranges: Vec<(String, String)> = Vec::new();

    for (start, end) in ranges {
        assert!(end.len() - start.len() < 2);
        let start_is_even = start.len() % 2 == 0;

        if start.len() % 2 != 0 && end.len() % 2 != 0 {
            continue;
        }

        if end.len() - start.len() == 0 {
            valid_ranges.push((start.to_string(), end.to_string()));
            continue;
        }

        if !start_is_even {
            let new_start = 10_u64.pow(end.len() as u32 - 1).to_string();
            valid_ranges.push((new_start, end.to_string()));
        } else {
            let new_end = (10_u64.pow(end.len() as u32 - 1) - 1).to_string();
            valid_ranges.push((start.to_string(), new_end));
        }
    }

    for (start, end) in valid_ranges {
        let split = start.len() / 2;

        let (start_hi, start_lo) = split_to_number(&start, split);
        let (end_hi, end_lo) = split_to_number(&end, split);

        let diff = end_hi - start_hi;

        if diff > 0 {
            for i in (start_hi + 1)..end_hi {
                sum += calculate_id(i);
            }

            if start_hi >= start_lo {
                sum += calculate_id(start_hi);
            }

            if end_hi <= end_lo {
                sum += calculate_id(end_hi);
            }
        } else if start_hi >= start_lo && start_hi <= end_lo {
            sum += calculate_id(start_hi);
        }
    }

    println!("Result Day 2 Part 1: {}", sum);
}
