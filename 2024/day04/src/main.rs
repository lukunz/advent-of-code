fn main() {
    let data = include_str!("../day04.txt");

    let width = data.find('\n').unwrap() + 1;
    let mut part1_count = 0;
    let mut part2_count = 0;

    for index in 0..data.len() {
        match &data[index..index + 1] {
            "X" => {
                if match_horizontal(data, index, "XMAS") {
                    part1_count += 1;
                }
                if match_vertical(data, index, width, "XMAS") {
                    part1_count += 1;
                }
                if match_vertical(data, index, width + 1, "XMAS") {
                    part1_count += 1;
                }
                if match_vertical(data, index, width - 1, "XMAS") {
                    part1_count += 1;
                }
            }
            "S" => {
                if match_horizontal(data, index, "SAMX") {
                    part1_count += 1;
                }
                if match_vertical(data, index, width, "SAMX") {
                    part1_count += 1;
                }
                if match_vertical(data, index, width + 1, "SAMX") {
                    part1_count += 1;
                }
                if match_vertical(data, index, width - 1, "SAMX") {
                    part1_count += 1;
                }
            }
            "A" => {
                if index > width + 1
                    && (match_vertical(data, index - width - 1, width + 1, "MAS")
                        || match_vertical(data, index - width - 1, width + 1, "SAM"))
                    && (match_vertical(data, index - width + 1, width - 1, "MAS")
                        || match_vertical(data, index - width + 1, width - 1, "SAM"))
                {
                    part2_count += 1;
                }
            }
            _ => {}
        }
    }

    println!("Day 04 Part 1: {}", part1_count);
    println!("Day 04 Part 2: {}", part2_count);
}

fn match_horizontal(input: &str, index: usize, word: &str) -> bool {
    if word.len() + index > input.len() {
        return false;
    }

    input[index..index + word.len()].eq(word)
}

fn match_vertical(input: &str, index: usize, width: usize, word: &str) -> bool {
    let mut start = index;

    for char in word.chars() {
        if start + 1 > input.len() {
            return false;
        }

        if input[start..start + 1] != char.to_string() {
            return false;
        }

        start += width;
    }

    true
}
