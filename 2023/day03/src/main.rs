use std::fs;

#[derive(Clone, Copy)]
struct NumberLocation {
    line_number: usize,
    start: usize,
    end: usize,
}

fn find_number_locations(data: String) -> Vec<NumberLocation> {
    let mut locations: Vec<NumberLocation> = Vec::new();

    for (line_number, line) in data.lines().enumerate() {
        let mut location: (Option<usize>, Option<usize>) = (None, None);

        for (col_number, ch) in line.chars().enumerate() {
            if ch.is_numeric() {
                if location.0.is_some() {
                    location.1 = Some(col_number);
                } else {
                    location.0 = Some(col_number);
                }
            } else if location.0.is_some() && location.1.is_none() {
                locations.push(NumberLocation {
                    line_number,
                    start: location.0.unwrap(),
                    end: location.0.unwrap(),
                });
                location = (None, None);
            } else if location.0.is_some() && location.1.is_some() {
                locations.push(NumberLocation {
                    line_number,
                    start: location.0.unwrap(),
                    end: location.1.unwrap(),
                });
                location = (None, None);
            }
        }

        if location.0.is_some() {
            if location.1.is_some() {
                locations.push(NumberLocation {
                    line_number,
                    start: location.0.unwrap(),
                    end: location.1.unwrap(),
                });
            } else {
                locations.push(NumberLocation {
                    line_number,
                    start: location.0.unwrap(),
                    end: location.0.unwrap(),
                });
            }
        }
    }

    locations
}

fn is_symbol(ch: char) -> bool {
    !ch.is_numeric() && ch != '.'
}

fn find_gears(map: &[Vec<char>], col_count: usize) -> Vec<(usize, usize)> {
    let mut locations: Vec<(usize, usize)> = Vec::new();

    for (y, _) in map.iter().enumerate() {
        for x in 0..col_count {
            if map[y][x] == '*' {
                locations.push((x, y));
            }
        }
    }

    locations
}

fn bound_inc(a: usize, bound: usize) -> usize {
    if a < bound {
        a + 1
    } else {
        a
    }
}

fn bound_dec(a: usize, bound: usize) -> usize {
    if bound < a {
        a - 1
    } else {
        a
    }
}

fn get_number(map: &[Vec<char>], location: &NumberLocation) -> usize {
    let number_string =
        String::from_iter(map[location.line_number][location.start..=location.end].iter());

    number_string.parse::<usize>().unwrap()
}

fn main() {
    let data = fs::read_to_string("day3.txt").expect("Can't read input file");

    let map: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    let row_count = map.len();
    let col_count = map[0].len();

    let number_locations = find_number_locations(data);
    let mut valid_number_locations: Vec<NumberLocation> = Vec::new();

    for location in number_locations.iter() {
        if location.line_number > 0 {
            // check top row
            for x in location.start..=location.end {
                if is_symbol(map[location.line_number - 1][x]) {
                    valid_number_locations.push(*location);
                }
            }
        }

        if location.line_number < row_count - 1 {
            // check bottom row
            for x in location.start..=location.end {
                if is_symbol(map[location.line_number + 1][x]) {
                    valid_number_locations.push(*location);
                }
            }
        }

        if location.start > 0 {
            // check left symbol
            if is_symbol(map[location.line_number][location.start - 1]) {
                valid_number_locations.push(*location);
            }
        }

        if location.end < col_count - 1 {
            // check right symbol
            if is_symbol(map[location.line_number][location.end + 1]) {
                valid_number_locations.push(*location);
            }
        }

        if location.line_number > 0 && location.start > 0 {
            // check top left symbol
            if is_symbol(map[location.line_number - 1][location.start - 1]) {
                valid_number_locations.push(*location);
            }
        }

        if location.line_number > 0 && location.end < col_count - 1 {
            // check top right symbol
            if is_symbol(map[location.line_number - 1][location.end + 1]) {
                valid_number_locations.push(*location);
            }
        }

        if location.line_number < row_count - 1 && location.start > 0 {
            // check bottom left
            if is_symbol(map[location.line_number + 1][location.start - 1]) {
                valid_number_locations.push(*location);
            }
        }

        if location.line_number < row_count - 1 && location.end < col_count - 1 {
            // check bottom right
            if is_symbol(map[location.line_number + 1][location.end + 1]) {
                valid_number_locations.push(*location);
            }
        }
    }

    let mut sum: usize = 0;

    for location in valid_number_locations {
        sum += get_number(&map, &location);
    }

    println!("Day 3 Part 1 {}", sum);

    let gear_locations = find_gears(&map, col_count);
    let mut gear_numbers: Vec<Vec<NumberLocation>> = Vec::new();

    for location in gear_locations {
        let numbers: Vec<NumberLocation> = number_locations
            .iter()
            .filter(|num_loc| {
                (bound_dec(num_loc.line_number, 0)..=bound_inc(num_loc.line_number, row_count))
                    .contains(&location.1)
                    && (bound_dec(num_loc.start, 0)..=bound_inc(num_loc.end, row_count))
                        .contains(&location.0)
            })
            .copied()
            .collect();

        if numbers.len() == 2 {
            gear_numbers.push(numbers);
        }
    }

    let part2_solution: usize = gear_numbers
        .iter()
        .map(|numbers| {
            numbers
                .iter()
                .map(|number| get_number(&map, number))
                .product::<usize>()
        })
        .sum();

    println!("Day 3 Part 2: {}", part2_solution);
}
