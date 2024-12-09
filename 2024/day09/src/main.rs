use std::iter::repeat;

#[derive(Clone, Copy)]
enum Block {
    File(usize, u32),
    Empty(u32),
}

fn main() {
    let data = include_str!("../day09.txt");

    let input = data
        .trim()
        .chars()
        .map(|c| {
            c.to_digit(10)
                .unwrap_or_else(|| panic!("Unknown input character '{}'", c))
        })
        .collect::<Vec<u32>>();

    let part1_result = part1(&input);
    let part2_result = part2(&input);

    println!("Day 9 Part 1: {}", part1_result);
    println!("Day 9 Part 2: {}", part2_result);
}

fn part1(input: &[u32]) -> usize {
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

    calculate_checksum(&expanded)
}

fn calculate_checksum(expanded: &[Option<usize>]) -> usize {
    expanded
        .iter()
        .enumerate()
        .filter_map(|(index, item)| item.as_ref().map(|val| (index, val)))
        .map(|(index, item)| item * index)
        .sum::<usize>()
}

fn part2(input: &[u32]) -> usize {
    let mut disk = input
        .iter()
        .enumerate()
        .map(|(index, item)| {
            if index % 2 == 0 {
                Block::File(index / 2, *item)
            } else {
                Block::Empty(*item)
            }
        })
        .collect::<Vec<Block>>();

    let files = {
        let mut files = disk
            .iter()
            .filter_map(|block| match block {
                Block::File(id, size) => Some((*id, *size)),
                _ => None,
            })
            .collect::<Vec<(usize, u32)>>();

        files.reverse();

        files
    };

    for (id, file_size) in files {
        let file_position = disk
            .iter()
            .position(|block| matches!(block, Block::File(i, _) if id == *i))
            .expect("Could not find file position");

        let empty_position = disk.iter().position(
            |block| matches!(block, Block::Empty(empty_size) if *empty_size >= file_size),
        );

        if let Some(empty_position) = empty_position {
            if empty_position < file_position {
                if let Block::Empty(empty_size) = disk[empty_position] {
                    disk[file_position] = Block::Empty(file_size);

                    if file_size == empty_size {
                        disk[empty_position] = Block::File(id, file_size);
                    } else {
                        disk[empty_position] = Block::Empty(empty_size - file_size);
                        disk.insert(empty_position, Block::File(id, file_size));
                    }

                    disk = remove_consecutive_empties(&mut disk);
                }
            }
        }
    }

    let expanded_disk = disk
        .iter()
        .flat_map(|block| match block {
            Block::File(id, size) => repeat(Some(*id)).take(*size as usize),
            Block::Empty(size) => repeat(None).take(*size as usize),
        })
        .collect::<Vec<Option<usize>>>();

    calculate_checksum(&expanded_disk)
}

fn remove_consecutive_empties(disk: &mut [Block]) -> Vec<Block> {
    let (mut disk, last_empty) = disk.iter().fold(
        (Vec::new(), None),
        |(mut new_disk, last_empty), block| match block {
            Block::File(_, _) => {
                if let Some(empty) = last_empty {
                    new_disk.push(empty);
                }
                new_disk.push(*block);

                (new_disk, None)
            }
            Block::Empty(size) => {
                if let Some(Block::Empty(last_size)) = last_empty {
                    (new_disk, Some(Block::Empty(size + last_size)))
                } else {
                    (new_disk, Some(Block::Empty(*size)))
                }
            }
        },
    );

    if let Some(empty) = last_empty {
        disk.push(empty);
    }

    disk
}
