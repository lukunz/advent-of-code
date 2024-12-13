#[derive(Debug)]
struct Machine {
    a_x: usize,
    a_y: usize,

    b_x: usize,
    b_y: usize,

    prize_x: usize,
    prize_y: usize,
}

impl Machine {
    fn from_string(input: &str) -> Self {
        let mut machine = Self {
            a_x: 0,
            a_y: 0,
            b_x: 0,
            b_y: 0,
            prize_x: 0,
            prize_y: 0,
        };

        for line in input.lines() {
            match line {
                line if line.starts_with("Button A") => {
                    let (x, y) = parse_button(line);

                    machine.a_x = x;
                    machine.a_y = y;
                }
                line if line.starts_with("Button B") => {
                    let (x, y) = parse_button(line);

                    machine.b_x = x;
                    machine.b_y = y;
                }
                line if line.starts_with("Prize") => {
                    let (x, y) = parse_price(line);

                    machine.prize_x = x;
                    machine.prize_y = y;
                }
                _ => unreachable!(),
            }
        }

        machine
    }

    fn solve(&self) -> Option<(usize, usize)> {
        let b1 = self.b_x * self.a_y;
        let b2 = self.b_y * self.a_x;

        let p1 = self.prize_x * self.a_y;
        let p2 = self.prize_y * self.a_x;

        let (b, p) = if b1 > b2 && p1 > p2 {
            Some((b1 - b2, p1 - p2))
        } else if b1 < b2 && p1 < p2 {
            Some((b2 - b1, p2 - p1))
        } else {
            None
        }?;

        let b = if p % b == 0 { Some(p / b) } else { None }?;

        let sub_b = b * self.b_x;

        if self.prize_x > sub_b {
            let p2 = self.prize_x - sub_b;

            if p2 % self.a_x == 0 {
                return Some((p2 / self.a_x, b));
            }
        }

        None
    }
}

fn parse_button(input: &str) -> (usize, usize) {
    let (_, data) = input.split_once(": ").expect("Line has no ':'");
    let (x_data, y_data) = data.split_once(", ").expect("Line has no ','");
    let (_, x) = x_data.split_once("X+").expect("x has no '+'");
    let (_, y) = y_data.split_once("Y+").expect("y has no '+'");

    (
        x.parse().expect("X is not a number"),
        y.parse().expect("Y is not a number"),
    )
}

fn parse_price(input: &str) -> (usize, usize) {
    let (_, data) = input.split_once(": ").expect("Line has no ':'");
    let (x_data, y_data) = data.split_once(", ").expect("Line has no ','");
    let (_, x) = x_data.split_once("X=").expect("x has no '='");
    let (_, y) = y_data.split_once("Y=").expect("y has no '='");

    (
        x.parse().expect("X is not a number"),
        y.parse().expect("Y is not a number"),
    )
}

fn main() {
    let data = include_str!("../day13.txt");

    let part1_result = data
        .split("\n\n")
        .map(Machine::from_string)
        .filter_map(|machine| machine.solve())
        .map(|(a, b)| a * 3 + b)
        .sum::<usize>();

    println!("Day 13 Part 1: {}", part1_result);
}
