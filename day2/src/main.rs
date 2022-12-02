extern crate core;

use std::fs::File;
use std::io::{BufRead, BufReader};

enum Rps {
    Rock,
    Paper,
    Scissors,
}

fn parse_line(line: String) -> (Rps, Rps) {
    let mut split = line.split(' ');

    let opponent_move = match split.next().unwrap() {
        "A" => Rps::Rock,
        "B" => Rps::Paper,
        "C" => Rps::Scissors,
        _ => panic!("unknown move"),
    };

    let player_move = match split.next().unwrap() {
        "X" => Rps::Rock,
        "Y" => Rps::Paper,
        "Z" => Rps::Scissors,
        _ => panic!("unknown move"),
    };

    (opponent_move, player_move)
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut total = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let round = parse_line(line);

        total += match round {
            (Rps::Rock, Rps::Rock) => 3 + 1,
            (Rps::Rock, Rps::Paper) => 6 + 2,
            (Rps::Rock, Rps::Scissors) => 0 + 3,

            (Rps::Paper, Rps::Rock) => 0 + 1,
            (Rps::Paper, Rps::Paper) => 3 + 2,
            (Rps::Paper, Rps::Scissors) => 6 + 3,

            (Rps::Scissors, Rps::Rock) => 6 + 1,
            (Rps::Scissors, Rps::Paper) => 0 + 2,
            (Rps::Scissors, Rps::Scissors) => 3 + 3,
        };
    }

    println!("{total}");
}
