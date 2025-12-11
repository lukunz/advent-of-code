fn parse_input_lights(input: &str) -> Vec<(u64, Vec<u64>)> {
    let mut machines = Vec::new();

    for line in input.lines() {
        let mut pattern = 0;
        let mut buttons: Vec<u64> = Vec::new();

        for object in line.split_whitespace() {
            match object.chars().next().unwrap() {
                '[' => {
                    for c in object.chars().rev() {
                        match c {
                            '.' => pattern <<= 1,
                            '#' => pattern = (pattern << 1) + 1,
                            _ => {}
                        }
                    }
                }
                '(' => {
                    let button = object
                        .trim_matches(|c| c == '(' || c == ')')
                        .split(',')
                        .map(|c| c.parse::<u32>().unwrap())
                        .map(|b| 1 << b)
                        .sum::<u64>();

                    buttons.push(button);
                }
                _ => {}
            }
        }

        machines.push((pattern, buttons));
    }

    machines
}

fn push_button_lights(pattern: u64, buttons: &[u64], count: u64) -> bool {
    if count == 0 {
        return false;
    }

    for button in buttons {
        if pattern == *button {
            return true;
        }

        if push_button_lights(pattern ^ button, buttons, count - 1) {
            return true;
        }
    }

    false
}

fn part1(input: &str) -> u64 {
    let machines = parse_input_lights(input);
    let mut total_pushes = 0;

    for (pattern, buttons) in &machines {
        if *pattern == 0 {
            continue;
        }

        let mut push_count = 1;

        loop {
            if push_button_lights(*pattern, buttons, push_count) {
                total_pushes += push_count;
                break;
            }
            push_count += 1;
        }
    }

    total_pushes
}

fn main() {
    let input = include_str!("../day10.txt");

    println!("Day 10 Part 1: {}", part1(input));
}
