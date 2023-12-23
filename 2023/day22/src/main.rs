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

    fn supports_map(&self) -> Vec<HashSet<usize>> {
        let supports = |block: &Point, brick_index: usize| {
            self.map[block.z + 1][block.y][block.x]
                .iter()
                .filter(move |&index| *index != brick_index)
        };

        self.bricks
            .iter()
            .enumerate()
            .map(|(index, brick)| {
                brick
                    .blocks
                    .iter()
                    .flat_map(|brick| supports(brick, index))
                    .copied()
                    .collect()
            })
            .collect()
    }

    fn supported_by_map(&self) -> Vec<HashSet<usize>> {
        let supported_by = |block: &Point, brick_index: usize| {
            self.map[block.z - 1][block.y][block.x]
                .iter()
                .filter(move |&index| *index != brick_index)
        };

        self.bricks
            .iter()
            .enumerate()
            .map(|(index, brick)| {
                brick
                    .blocks
                    .iter()
                    .flat_map(|brick| supported_by(brick, index))
                    .copied()
                    .collect()
            })
            .collect()
    }

    fn can_be_disintegrated(
        &self,
        brick_index: usize,
        supported_map: &[HashSet<usize>],
        supported_by_map: &[HashSet<usize>],
    ) -> bool {
        supported_map[brick_index].iter().all(|&index| {
            supported_by_map[index]
                .iter()
                .filter(|i| **i != brick_index)
                .count()
                > 0
        })
    }

    fn count_falling_bricks(
        &self,
        brick_index: usize,
        supported_by_map: &[HashSet<usize>],
    ) -> usize {
        let mut falling_bricks = 0;
        let mut ignored_indices = vec![brick_index];

        let min_z = self.bricks[brick_index]
            .start
            .z
            .min(self.bricks[brick_index].end.z);

        for (index, _) in self
            .bricks
            .iter()
            .enumerate()
            .filter(|(_, brick)| brick.start.z.min(brick.end.z) > min_z)
        {
            if index == brick_index {
                continue;
            }

            let count: HashSet<&usize> = supported_by_map[index]
                .iter()
                .filter(|i| !ignored_indices.contains(i))
                .collect();

            if count.is_empty() {
                falling_bricks += 1;
                ignored_indices.push(index);
            }
        }

        falling_bricks
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
    let supports_map = map.supports_map();
    let supported_by_map = map.supported_by_map();

    let part1_result = map
        .bricks
        .iter()
        .enumerate()
        .filter(|(index, _)| map.can_be_disintegrated(*index, &supports_map, &supported_by_map))
        .count();

    println!("Day 22 Part 1: {}", part1_result);

    let part2_result: usize = map
        .bricks
        .iter()
        .enumerate()
        .map(|(index, _)| map.count_falling_bricks(index, &supported_by_map))
        .sum();

    println!("Day 22 Part 2: {}", part2_result);
}
