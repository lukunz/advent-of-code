use std::cmp::max;
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

fn part1(games: &[Game]) {
    let max_round = Round {
        red: 12,
        green: 13,
        blue: 14,
    };

    let sum: u32 = games
        .iter()
        .filter(|game| check_game(game, &max_round))
        .map(|game| game.id)
        .sum();

    println!("Day 2 Part 1: {}", sum);
}

fn find_max_round(game: &Game) -> Round {
    let mut max_round = Round {
        red: 0,
        green: 0,
        blue: 0,
    };

    for round in &game.rounds {
        max_round.red = max(max_round.red, round.red);
        max_round.green = max(max_round.green, round.green);
        max_round.blue = max(max_round.blue, round.blue);
    }

    max_round
}

fn part2(games: &[Game]) {
    let sum: u32 = games
        .iter()
        .map(find_max_round)
        .map(|round| round.red * round.green * round.blue)
        .sum();

    println!("Day 2 Part 2: {}", sum);
}

fn main() {
    let data = fs::read_to_string("day2.txt").expect("Can't read input file");
    let games: Vec<Game> = data.lines().map(parse_game).collect();

    part1(&games);
    part2(&games);
}
