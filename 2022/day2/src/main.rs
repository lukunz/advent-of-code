extern crate core;

use std::fs::File;
use std::io::{BufRead, BufReader};

enum Rps {
    Rock,
    Paper,
    Scissors,
}

enum RoundResult {
    Loose,
    Draw,
    Win,
}

fn parse_line(line: String) -> (Rps, RoundResult) {
    let mut split = line.split(' ');

    let opponent_move = match split.next().unwrap() {
        "A" => Rps::Rock,
        "B" => Rps::Paper,
        "C" => Rps::Scissors,
        _ => panic!("unknown move"),
    };

    let expected_result = match split.next().unwrap() {
        "X" => RoundResult::Loose,
        "Y" => RoundResult::Draw,
        "Z" => RoundResult::Win,
        _ => panic!("unknown round result"),
    };

    (opponent_move, expected_result)
}

fn points_for_move(player_move: Rps) -> u32 {
    match player_move {
        Rps::Rock => 1,
        Rps::Paper => 2,
        Rps::Scissors => 3,
    }
}

fn main() {
    let file = File::open("../input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut total = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let round = parse_line(line);

        total += match round {
            (Rps::Rock, RoundResult::Loose) => 0 + points_for_move(Rps::Scissors),
            (Rps::Rock, RoundResult::Draw) => 3 + points_for_move(Rps::Rock),
            (Rps::Rock, RoundResult::Win) => 6 + points_for_move(Rps::Paper),

            (Rps::Paper, RoundResult::Loose) => 0 + points_for_move(Rps::Rock),
            (Rps::Paper, RoundResult::Draw) => 3 + points_for_move(Rps::Paper),
            (Rps::Paper, RoundResult::Win) => 6 + points_for_move(Rps::Scissors),

            (Rps::Scissors, RoundResult::Loose) => 0 + points_for_move(Rps::Paper),
            (Rps::Scissors, RoundResult::Draw) => 3 + points_for_move(Rps::Scissors),
            (Rps::Scissors, RoundResult::Win) => 6 + points_for_move(Rps::Rock),
        };
    }

    println!("{total}");
}
