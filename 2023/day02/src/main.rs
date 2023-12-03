use std::fs;

struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game {
    id: u32,
    rounds: Vec<Round>,
}

fn parse_game(line: &str) -> Game {
    let split: Vec<&str> = line.split(": ").collect();

    let game_id = split[0].split(' ').last().unwrap().parse::<u32>().unwrap();
    let mut rounds = Vec::new();

    for round_str in split[1].split("; ") {
        let mut round = Round {
            red: 0,
            green: 0,
            blue: 0,
        };

        for cube_str in round_str.split(", ") {
            let cube_split: Vec<&str> = cube_str.split(' ').collect();
            match cube_split[1] {
                "red" => round.red = cube_split[0].parse::<u32>().unwrap(),
                "green" => round.green = cube_split[0].parse::<u32>().unwrap(),
                "blue" => round.blue = cube_split[0].parse::<u32>().unwrap(),
                _ => panic!("Unknown color"),
            }
        }

        rounds.push(round);
    }

    Game {
        id: game_id,
        rounds,
    }
}

fn check_game(game: &Game, max_round: &Round) -> bool {
    for round in &game.rounds {
        if round.red > max_round.red || round.green > max_round.green || round.blue > max_round.blue
        {
            return false;
        }
    }

    true
}

fn main() {
    let data = fs::read_to_string("day2.txt").expect("Can't read input file");
    let max_round = Round {
        red: 12,
        green: 13,
        blue: 14,
    };
    let sum: u32 = data
        .lines()
        .map(|line| parse_game(line))
        .filter(|game| check_game(game, &max_round))
        .map(|game| game.id)
        .sum();

    println!("Day 2 Part 1: {}", sum);
}
