use std::iter::repeat;

fn main() {
    let data = include_str!("../day09.txt");

    let input = data
        .trim()
        .chars()
        .map(|c| {
            c.to_digit(10)
                .unwrap_or_else(|| panic!("Unknown input character '{}'", c)) as u8
        })
        .collect::<Vec<u8>>();

    let mut expanded: Vec<Option<usize>> = Vec::new();
    let mut id: usize = 0;

    for (index, item) in input.iter().enumerate() {
        if index % 2 == 0 {
            expanded.extend(repeat(Some(id)).take(*item as usize));
            id += 1;
        } else {
            expanded.extend(repeat(None).take(*item as usize));
        }
    }

    let mut r_index = expanded.len() - 1;

    for i in 0..expanded.len() {
        if i == r_index {
            break;
        }

        if expanded[i].is_none() {
            while expanded[r_index].is_none() {
                r_index -= 1;

                if r_index == i {
                    break;
                }
            }

            expanded[i] = expanded[r_index];
            expanded[r_index] = None;
        }
    }

    let part1_result = expanded
        .iter()
        .filter_map(|item| *item)
        .enumerate()
        .map(|(index, item)| item * index)
        .sum::<usize>();

    println!("Day 9 Part 1: {}", part1_result);
}
