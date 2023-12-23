use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    fn from_str(data: &str) -> Self {
        let mut data = data.split(',');
        let x: usize = data.next().unwrap().parse().unwrap();
        let y: usize = data.next().unwrap().parse().unwrap();
        let z: usize = data.next().unwrap().parse().unwrap();

        Self { x, y, z }
    }

    fn from_xyz(x: usize, y: usize, z: usize) -> Self {
        Point { x, y, z }
    }

    fn max(&self, other: &Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Brick {
    start: Point,
    end: Point,
    blocks: Vec<Point>,
}

impl PartialOrd<Self> for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start
            .z
            .min(self.end.z)
            .cmp(&other.start.z.min(other.end.z))
    }
}

impl Brick {
    fn from(start: Point, end: Point) -> Self {
        let blocks = Self::create_blocks(&start, &end);

        Self { start, end, blocks }
    }

    fn create_blocks(start: &Point, end: &Point) -> Vec<Point> {
        let mut blocks = Vec::new();

        for z in start.z.min(end.z)..=start.z.max(end.z) {
            for y in start.y.min(end.y)..=start.y.max(end.y) {
                for x in start.x.min(end.x)..=start.x.max(end.x) {
                    blocks.push(Point::from_xyz(x, y, z));
                }
            }
        }

        blocks
    }

    fn move_down(&mut self) {
        self.start.z -= 1;
        self.end.z -= 1;
        self.blocks = Self::create_blocks(&self.start, &self.end);
    }
}

struct Map {
    bricks: Vec<Brick>,
    map: Vec<Vec<Vec<HashSet<usize>>>>,
}

impl Map {
    fn from(mut bricks: Vec<Brick>) -> Map {
        let dimensions = bricks.iter().fold(Point::from_xyz(0, 0, 0), |acc, p| {
            acc.max(&p.start).max(&p.end)
        });

        let mut map: Vec<Vec<Vec<HashSet<usize>>>> = Vec::new();

        for _ in 0..=dimensions.z {
            let mut z_vec: Vec<Vec<HashSet<usize>>> = Vec::new();
            for _ in 0..=dimensions.y {
                let mut y_vec: Vec<HashSet<usize>> = Vec::new();
                for _ in 0..=dimensions.x {
                    y_vec.push(HashSet::new());
                }
                z_vec.push(y_vec);
            }
            map.push(z_vec);
        }

        bricks.sort();

        for (index, brick) in bricks.iter().enumerate() {
            for block in &brick.blocks {
                map[block.z][block.y][block.x].insert(index);
            }
        }

        Map { bricks, map }
    }

    fn condense(&mut self) {
        for index in 0..self.bricks.len() {
            let mut has_moved = true;
            while has_moved {
                has_moved = false;

                if self.can_move(index, &self.bricks[index]) {
                    has_moved = true;
                    for block in &self.bricks[index].blocks {
                        self.map[block.z][block.y][block.x].remove(&index);
                        self.map[block.z - 1][block.y][block.x].insert(index);
                    }

                    self.bricks[index].move_down();
                }
            }
        }
    }

    fn can_move(&self, index: usize, brick: &Brick) -> bool {
        brick.blocks.iter().all(|block| {
            block.z > 1
                && self.map[block.z - 1][block.y][block.x]
                    .iter()
                    .filter(|&&i| i != index)
                    .count()
                    == 0
        })
    }

    fn can_be_disintegrated(&self, brick_index: usize) -> bool {
        let supports = |block: &Point| {
            self.map[block.z + 1][block.y][block.x]
                .iter()
                .filter(|&index| *index != brick_index)
        };

        let supported_by = |block: &Point, except_index: usize| {
            self.map[block.z - 1][block.y][block.x]
                .iter()
                .filter(move |&index| *index != brick_index && *index != except_index)
        };

        let supported_bricks: HashSet<&usize> = self.bricks[brick_index]
            .blocks
            .iter()
            .flat_map(supports)
            .collect();

        let supported_by_bricks = |index: &usize| {
            self.bricks[*index]
                .blocks
                .iter()
                .flat_map(|block| supported_by(block, *index))
                .collect::<HashSet<&usize>>()
        };

        supported_bricks
            .iter()
            .all(|&index| !supported_by_bricks(index).is_empty())
    }
}

fn parse_input(data: &str) -> Vec<Brick> {
    data.lines()
        .map(|line| {
            let (start, end) = line.split_once('~').unwrap();

            Brick::from(Point::from_str(start), Point::from_str(end))
        })
        .collect()
}

fn main() {
    let data = fs::read_to_string("day22.txt").expect("Can't read input file");
    let bricks = parse_input(&data);

    let mut map = Map::from(bricks);
    map.condense();

    let part1_result = map
        .bricks
        .iter()
        .enumerate()
        .filter(|(index, _)| map.can_be_disintegrated(*index))
        .count();

    println!("Day 22 Part 1: {}", part1_result);
}
