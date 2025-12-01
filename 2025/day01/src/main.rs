fn parse_steps(lines: &Vec<&str>) -> Vec<i32> {
    lines
        .iter()
        .map(|line| {
            let (dir, number) = line.split_at(1);
            let number = number.parse::<i32>().unwrap();

            if dir == "R" { number } else { number * -1 }
        })
        .collect()
}

fn part1(steps: &Vec<i32>) -> i32 {
    let mut position = 50;
    let mut zero_counter = 0;

    for step in steps {
        position += step;
        position %= 100;

        if position == 0 {
            zero_counter += 1;
        }
    }

    zero_counter
}

fn part2(steps: &Vec<i32>) -> i32 {
    let mut position = 50;
    let mut zero_counter = 0;

    for step in steps {
        let small_step = step % 100;
        zero_counter += ((step - small_step) / 100).abs();
        let new_position = position + small_step;

        if position != 0 {
            if new_position < 0 {
                zero_counter += 1;
            } else if new_position >= 100 {
                zero_counter += 1;
            }
        }

        if new_position == 0 {
            zero_counter += 1;
        }

        position = (new_position + 100) % 100;
    }

    zero_counter
}

fn main() {
    let data = include_str!("../day01.txt");

    let lines = data.lines().collect();
    let steps = parse_steps(&lines);

    let result_part1 = part1(&steps);

    println!("Result Day 1 Part 1: {}", result_part1);

    let result_part2 = part2(&steps);

    println!("Result Day 1 Part 2: {}", result_part2);
}
