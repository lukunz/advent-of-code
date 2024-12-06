#[derive(PartialEq, Copy, Clone)]
enum Tile {
    Empty,
    Blocked,
    Visited,
    Player(Direction),
}

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

struct Position {
    x: usize,
    y: usize,
}

fn main() {
    let data = include_str!("../day06.txt");

    let mut map = data
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Blocked,
                    '^' => Tile::Player(Direction::North),
                    '>' => Tile::Player(Direction::East),
                    'v' => Tile::Player(Direction::South),
                    '<' => Tile::Player(Direction::West),
                    _ => panic!("Invalid input"),
                })
                .collect::<Vec<Tile>>()
        })
        .collect::<Vec<Vec<Tile>>>();

    let map_height = map.len();
    let map_width = map[0].len();

    let (mut player_position, mut player_direction) = find_player(&map).expect("No player found");

    loop {
        map[player_position.y][player_position.x] = Tile::Visited;

        match player_direction {
            Direction::North => {
                if player_position.y == 0 {
                    break;
                }

                match map[player_position.y - 1][player_position.x] {
                    Tile::Empty | Tile::Visited => {
                        player_position.y -= 1;
                    }
                    Tile::Blocked => {
                        player_direction = Direction::East;
                    }
                    Tile::Player(_) => {
                        panic!("Player should not be here")
                    }
                }
            }
            Direction::East => {
                if player_position.x == map_width - 1 {
                    break;
                }

                match map[player_position.y][player_position.x + 1] {
                    Tile::Empty | Tile::Visited => {
                        player_position.x += 1;
                    }
                    Tile::Blocked => {
                        player_direction = Direction::South;
                    }
                    Tile::Player(_) => {
                        panic!("Player should not be here")
                    }
                }
            }
            Direction::South => {
                if player_position.y == map_height - 1 {
                    break;
                }

                match map[player_position.y + 1][player_position.x] {
                    Tile::Empty | Tile::Visited => {
                        player_position.y += 1;
                    }
                    Tile::Blocked => {
                        player_direction = Direction::West;
                    }
                    Tile::Player(_) => {
                        panic!("Player should not be here")
                    }
                }
            }
            Direction::West => {
                if player_position.x == 0 {
                    break;
                }

                match map[player_position.y][player_position.x - 1] {
                    Tile::Empty | Tile::Visited => {
                        player_position.x -= 1;
                    }
                    Tile::Blocked => {
                        player_direction = Direction::North;
                    }
                    Tile::Player(_) => {
                        panic!("Player should not be here")
                    }
                }
            }
        }
    }

    let visited_count = map
        .iter()
        .map(|row| row.iter().filter(|&tile| tile == &Tile::Visited).count())
        .sum::<usize>();

    println!("Day 6 Part 1: {}", visited_count);
}

fn find_player(map: &[Vec<Tile>]) -> Option<(Position, Direction)> {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if let Tile::Player(direction) = &map[y][x] {
                return Some((Position { x, y }, *direction));
            }
        }
    }

    None
}
