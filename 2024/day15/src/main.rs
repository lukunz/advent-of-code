#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
enum Tile {
    Wall,
    Box,
    BoxL,
    BoxR,
    Empty,
}

enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
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

    fn step(&self, pos: &Position, dir: &Direction, amount: usize) -> Option<Position> {
        match dir {
            Direction::North if pos.y >= amount => Some(Position::new(pos.x, pos.y - amount)),
            Direction::East if pos.x < self.width - amount => {
                Some(Position::new(pos.x + amount, pos.y))
            }
            Direction::South if pos.y < self.height - amount => {
                Some(Position::new(pos.x, pos.y + amount))
            }
            Direction::West if pos.x >= amount => Some(Position::new(pos.x - amount, pos.y)),
            _ => None,
        }
    }

    fn double_width(&self) -> Self {
        let tiles = self
            .tiles
            .iter()
            .map(|row| {
                row.iter()
                    .flat_map(|tile| match tile {
                        Tile::Box => vec![Tile::BoxL, Tile::BoxR],
                        t => vec![t.clone(), t.clone()],
                    })
                    .collect::<Vec<Tile>>()
            })
            .collect::<Vec<_>>();

        Self {
            tiles,
            width: self.width * 2,
            height: self.height,
            robot: Position {
                x: self.robot.x * 2,
                y: self.robot.y,
            },
        }
    }
}

fn main() {
    let data = include_str!("../day15.txt");

    let (map_input, dir_input) = data.split_once("\n\n").expect("Input in wrong format");

    let map = Map::from_str(map_input);
    let wide_map = map.double_width();

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

    let part1_result = run(map, &dirs);
    let part2_result = run(wide_map, &dirs);

    println!("Day 15 Part 1: {}", part1_result);
    println!("Day 15 Part 2: {}", part2_result);
}

fn run(mut map: Map, dirs: &[Direction]) -> usize {
    let mut movable_boxes: Vec<(Position, Tile)> = Vec::new();
    let mut pos_to_check: Vec<Position> = Vec::new();

    for dir in dirs {
        let mut blocked = false;

        if let Some(pos) = map.step(&map.robot, dir, 1) {
            pos_to_check.push(pos);
        }

        while let Some(pos) = pos_to_check.pop() {
            match map.tiles[pos.y][pos.x] {
                Tile::Empty => {}
                Tile::Wall => {
                    blocked = true;
                    pos_to_check.clear();
                }
                Tile::Box => {
                    let new_pos = map
                        .step(&pos, dir, 1)
                        .expect("Boxes can't be on the edge of the map");

                    pos_to_check.push(new_pos);
                    movable_boxes.push((pos, Tile::Box));
                }
                Tile::BoxL => match dir {
                    Direction::North | Direction::South => {
                        let new_pos = map
                            .step(&pos, dir, 1)
                            .expect("Boxes can't be on the edge of the map");
                        let right_pos = map
                            .step(&pos, &Direction::East, 1)
                            .expect("BoxR must exist, if BoxL exists");
                        let new_right_pos = map
                            .step(&right_pos, dir, 1)
                            .expect("Boxes can't be on the edge of the map");

                        movable_boxes.push((pos, Tile::BoxL));
                        movable_boxes.push((right_pos, Tile::BoxR));
                        pos_to_check.push(new_pos);
                        pos_to_check.push(new_right_pos);
                    }
                    Direction::East => {
                        let new_pos = map
                            .step(&pos, dir, 2)
                            .expect("Boxes can't be on the edge of the map");

                        pos_to_check.push(new_pos);
                        movable_boxes
                            .push((map.step(&pos, &Direction::East, 1).unwrap(), Tile::BoxR));
                        movable_boxes.push((pos, Tile::BoxL));
                    }
                    Direction::West => {
                        panic!("Cannot encounter BoxL when moving West")
                    }
                },
                Tile::BoxR => match dir {
                    Direction::North | Direction::South => {
                        pos_to_check.push(map.step(&pos, &Direction::West, 1).unwrap());
                    }
                    Direction::West => {
                        let new_pos = map
                            .step(&pos, dir, 2)
                            .expect("Boxes can't be on the edge of the map");

                        pos_to_check.push(new_pos);
                        movable_boxes
                            .push((map.step(&pos, &Direction::West, 1).unwrap(), Tile::BoxL));
                        movable_boxes.push((pos, Tile::BoxR));
                    }
                    Direction::East => {
                        panic!("Cannot encounter BoxR when moving East")
                    }
                },
            }
        }

        if !blocked {
            movable_boxes.sort();
            movable_boxes.dedup();

            for (pos, _) in &movable_boxes {
                map.tiles[pos.y][pos.x] = Tile::Empty;
            }

            for (pos, tile) in movable_boxes.drain(..) {
                if let Some(new_pos) = map.step(&pos, dir, 1) {
                    map.tiles[new_pos.y][new_pos.x] = tile;
                }
            }

            if let Some(new_pos) = map.step(&map.robot, dir, 1) {
                map.robot = new_pos;
            }
        } else {
            movable_boxes.clear();
        }
    }

    sum_gps(&map)
}

fn sum_gps(map: &Map) -> usize {
    map.tiles
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, tile)| {
                if matches!(tile, Tile::Box | Tile::BoxL) {
                    Some(x + 100 * y)
                } else {
                    None
                }
            })
        })
        .sum::<usize>()
}
