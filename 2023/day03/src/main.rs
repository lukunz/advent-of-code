use std::fs;

#[derive(Clone)]
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

fn main() {
    let data = fs::read_to_string("day3.txt").expect("Can't read input file");

    let map: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    let row_count = map.len();
    let col_count = map[0].len();

    let number_locations = find_number_locations(data);
    let mut valid_number_locations: Vec<NumberLocation> = Vec::new();

    for location in number_locations {
        if location.line_number > 0 {
            // check top row
            for x in location.start..=location.end {
                if is_symbol(map[location.line_number - 1][x]) {
                    valid_number_locations.push(location.clone());
                }
            }
        }

        if location.line_number < row_count - 1 {
            // check bottom row
            for x in location.start..=location.end {
                if is_symbol(map[location.line_number + 1][x]) {
                    valid_number_locations.push(location.clone());
                }
            }
        }

        if location.start > 0 {
            // check left symbol
            if is_symbol(map[location.line_number][location.start - 1]) {
                valid_number_locations.push(location.clone());
            }
        }

        if location.end < col_count - 1 {
            // check right symbol
            if is_symbol(map[location.line_number][location.end + 1]) {
                valid_number_locations.push(location.clone());
            }
        }

        if location.line_number > 0 && location.start > 0 {
            // check top left symbol
            if is_symbol(map[location.line_number - 1][location.start - 1]) {
                valid_number_locations.push(location.clone());
            }
        }

        if location.line_number > 0 && location.end < col_count - 1 {
            // check top right symbol
            if is_symbol(map[location.line_number - 1][location.end + 1]) {
                valid_number_locations.push(location.clone());
            }
        }

        if location.line_number < row_count - 1 && location.start > 0 {
            // check bottom left
            if is_symbol(map[location.line_number + 1][location.start - 1]) {
                valid_number_locations.push(location.clone());
            }
        }

        if location.line_number < row_count - 1 && location.end < col_count - 1 {
            // check bottom right
            if is_symbol(map[location.line_number + 1][location.end + 1]) {
                valid_number_locations.push(location.clone());
            }
        }
    }

    let mut sum: usize = 0;

    for location in valid_number_locations {
        let number_string =
            String::from_iter(map[location.line_number][location.start..=location.end].iter());

        // println!("{}", number_string);
        sum += number_string.parse::<usize>().unwrap();
    }

    println!("Day 3 Part 1 {}", sum);
}
