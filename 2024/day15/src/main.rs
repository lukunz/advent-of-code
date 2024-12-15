#[derive(PartialEq)]
enum Tile {
    Wall,
    Box,
    Empty,
}

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

struct Map {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
    robot: Position,
}

impl Map {
    fn from_str(input: &str) -> Self {
        let tiles = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '#' => Tile::Wall,
                        'O' => Tile::Box,
                        '.' | '@' => Tile::Empty,
                        _ => panic!("Unknown tile"),
                    })
                    .collect()
            })
            .collect::<Vec<Vec<Tile>>>();

        let width = tiles[0].len();
        let height = tiles.len();

        let robot_position = input.find("@").expect("No robot found");
        let y = robot_position / (width + 1);
        let x = robot_position - (width + 1) * y;

        Self {
            tiles,
            width,
            height,
            robot: Position { x, y },
        }
    }

    fn step(&self, pos: &mut Position, dir: &Direction) -> Option<&Tile> {
        match dir {
            Direction::North => {
                if pos.y > 0 {
                    pos.y -= 1;
                    Some(&self.tiles[pos.y][pos.x])
                } else {
                    None
                }
            }
            Direction::East => {
                if pos.x < self.width - 1 {
                    pos.x += 1;
                    Some(&self.tiles[pos.y][pos.x])
                } else {
                    None
                }
            }
            Direction::South => {
                if pos.y < self.height - 1 {
                    pos.y += 1;
                    Some(&self.tiles[pos.y][pos.x])
                } else {
                    None
                }
            }
            Direction::West => {
                if pos.x > 0 {
                    pos.x -= 1;
                    Some(&self.tiles[pos.y][pos.x])
                } else {
                    None
                }
            }
        }
    }

    fn move_robot_and_boxes(&mut self, end_pos: &Position, dir: &Direction) {
        self.tiles[end_pos.y][end_pos.x] = Tile::Box;

        match dir {
            Direction::North => {
                self.robot.y -= 1;
            }
            Direction::East => {
                self.robot.x += 1;
            }
            Direction::South => {
                self.robot.y += 1;
            }
            Direction::West => {
                self.robot.x -= 1;
            }
        }

        self.tiles[self.robot.y][self.robot.x] = Tile::Empty;
    }
}

fn main() {
    let data = include_str!("../day15.txt");

    let (map_input, dir_input) = data.split_once("\n\n").expect("Input in wrong format");
    let mut map = Map::from_str(map_input);

    let dirs = dir_input
        .chars()
        .filter_map(|c| match c {
            '^' => Some(Direction::North),
            '>' => Some(Direction::East),
            'v' => Some(Direction::South),
            '<' => Some(Direction::West),
            _ => None,
        })
        .collect::<Vec<_>>();

    for dir in dirs {
        move_robot(&mut map, &dir);
    }

    let part1_result = map
        .tiles
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, tile)| {
                if *tile == Tile::Box {
                    Some(x + 100 * y)
                } else {
                    None
                }
            })
        })
        .sum::<usize>();

    println!("Day 15 Part 1: {}", part1_result);
}

fn move_robot(map: &mut Map, dir: &Direction) {
    let mut current_pos = Position {
        x: map.robot.x,
        y: map.robot.y,
    };

    while let Some(tile) = map.step(&mut current_pos, dir) {
        match tile {
            Tile::Empty => {
                map.move_robot_and_boxes(&current_pos, dir);
                return;
            }
            Tile::Wall => {
                return;
            }
            Tile::Box => {}
        }
    }
}
