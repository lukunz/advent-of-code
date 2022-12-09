use std::collections::HashSet;
use std::fs;

#[derive(Eq, Hash, PartialEq, Clone)]
struct Point {
    x: i32,
    y: i32,
}

enum Action {
    Right(i32),
    Left(i32),
    Down(i32),
    Up(i32),
}

fn main() {
    let actions = read_input_file("day9/input.txt");
    let part_one = part_one(&actions);
    let part_two = part_two(&actions);

    println!("Part one: {part_one}");
    println!("Part two: {part_two}");
}

fn part_one(actions: &Vec<Action>) -> usize {
    let mut head = Point { x: 0, y: 0 };
    let mut tail = Point { x: 0, y: 0 };
    let mut visited_points = HashSet::new();

    visited_points.insert(tail.clone());

    for action in actions {
        match action {
            Action::Right(distance) => {
                for _ in 0..*distance {
                    head.x += 1;
                    tail = move_tail(&head, &tail);
                    visited_points.insert(tail.clone());
                }
            }
            Action::Left(distance) => {
                for _ in 0..*distance {
                    head.x -= 1;
                    tail = move_tail(&head, &tail);
                    visited_points.insert(tail.clone());
                }
            }
            Action::Down(distance) => {
                for _ in 0..*distance {
                    head.y += 1;
                    tail = move_tail(&head, &tail);
                    visited_points.insert(tail.clone());
                }
            }
            Action::Up(distance) => {
                for _ in 0..*distance {
                    head.y -= 1;
                    tail = move_tail(&head, &tail);
                    visited_points.insert(tail.clone());
                }
            }
        }
    }

    visited_points.len()
}

fn part_two(actions: &Vec<Action>) -> usize {
    let mut rope: Vec<Point> = (0..10).map(|_| Point { x: 0, y: 0}).collect();
    let mut visited_points = HashSet::new();

    for action in actions {
        match action {
            Action::Right(distance) => {
                for _ in 0..*distance {
                    rope[0].x += 1;
                    for i in 0..9 {
                        rope[i + 1] = move_tail(&rope[i], &rope[i + 1]);
                    }
                    visited_points.insert(rope[9].clone());
                }
            }
            Action::Left(distance) => {
                for _ in 0..*distance {
                    rope[0].x -= 1;
                    for i in 0..9 {
                        rope[i + 1] = move_tail(&rope[i], &rope[i + 1]);
                    }
                    visited_points.insert(rope[9].clone());
                }
            }
            Action::Down(distance) => {
                for _ in 0..*distance {
                    rope[0].y += 1;
                    for i in 0..9 {
                        rope[i + 1] = move_tail(&rope[i], &rope[i + 1]);
                    }
                    visited_points.insert(rope[9].clone());
                }
            }
            Action::Up(distance) => {
                for _ in 0..*distance {
                    rope[0].y -= 1;
                    for i in 0..9 {
                        rope[i + 1] = move_tail(&rope[i], &rope[i + 1]);
                    }
                    visited_points.insert(rope[9].clone());
                }
            }
        }
    }

    visited_points.len()
}

fn move_tail(head: &Point, tail: &Point) -> Point {
    let mut new_tail = Point { x: tail.x, y: tail.y };
    let x_distance = head.x - tail.x;
    let y_distance = head.y - tail.y;

    if x_distance.abs() > 1 {
        new_tail.x += x_distance.signum();

        if y_distance.abs() > 0 {
            new_tail.y += y_distance.signum();
        }
    } else if y_distance.abs() > 1 {
        new_tail.y += y_distance.signum();

        if x_distance.abs() > 0 {
            new_tail.x += x_distance.signum();
        }
    }

    new_tail
}

fn read_input_file(file: &str) -> Vec<Action> {
    let data = fs::read_to_string(file).expect("Can't read input file");

    data.lines()
        .filter_map(|line| line.split_once(' '))
        .map(|(direction_str, distance_str)| {
            let distance: i32 = distance_str.parse().unwrap();

            match direction_str {
                "R" => Action::Right(distance),
                "L" => Action::Left(distance),
                "D" => Action::Down(distance),
                "U" => Action::Up(distance),
                &_ => panic!(),
            }
        })
        .collect()
}
