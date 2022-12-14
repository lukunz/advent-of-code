use std::fs;
use std::ops::RangeInclusive;

#[derive(Clone)]
struct Point {
    x: u64,
    y: u64,
}

struct Limits {
    min_x: u64,
    max_x: u64,
    min_y: u64,
    max_y: u64,
}

#[derive(Clone, PartialEq)]
enum Tile {
    Air,
    Rock,
    Sand,
}

struct Map {
    offset_x: u64,
    offset_y: u64,
    tiles: Vec<Vec<Tile>>,
}

enum TikResult {
    Falling(Point),
    Settled,
    Finished,
}

impl Point {
    fn range_x(&self, other: &Point) -> RangeInclusive<u64> {
        if self.x < other.x {
            self.x..=other.x
        } else {
            other.x..=self.x
        }
    }

    fn range_y(&self, other: &Point) -> RangeInclusive<u64> {
        if self.y < other.y {
            self.y..=other.y
        } else {
            other.y..=self.y
        }
    }

    fn down(&self) -> Point {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn down_right(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y + 1,
        }
    }

    fn down_left(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y + 1,
        }
    }
}

impl Map {
    fn from_file(file: &str) -> Self {
        let data = fs::read_to_string(file).expect("Can't read input file");

        let paths: Vec<Vec<Point>> = data.lines().map(|line| parse_line(line)).collect();
        let limits = find_limits(&paths);

        Self::build(&limits, &paths)
    }

    fn build(limits: &Limits, paths: &Vec<Vec<Point>>) -> Self {
        let mut map = Map {
            offset_x: limits.min_x - 1,
            offset_y: 0,
            tiles: Vec::new(),
        };

        for x in map.offset_x..limits.max_x + 2 {
            map.tiles.push(vec![Tile::Air; 1 + limits.max_y as usize])
        }

        for path in paths {
            for i in 0..path.len() - 1 {
                let (start, end) = (&path[i], &path[i + 1]);

                for x in start.range_x(&end) {
                    for y in start.range_y(&end) {
                        map.set_tile(&Point { x, y }, Tile::Rock);
                    }
                }
            }
        }

        map
    }

    fn set_tile(&mut self, point: &Point, tile: Tile) {
        self.tiles[(point.x - self.offset_x) as usize][point.y as usize] = tile;
    }

    fn get_tile(&self, point: &Point) -> &Tile {
        &self.tiles[(point.x - self.offset_x) as usize][point.y as usize]
    }

    fn print(&self) {
        for y in 0..self.tiles[0].len() {
            for x in 0..self.tiles.len() {
                print!(
                    "{}",
                    match self.tiles[x][y] {
                        Tile::Air => '.',
                        Tile::Sand => 'o',
                        Tile::Rock => '#',
                    }
                );
            }
            println!();
        }
    }

    fn is_blocked(&self, point: &Point) -> bool {
        self.get_tile(point) != &Tile::Air
    }

    fn contains(&self, point: &Point) -> bool {
        (0..self.tiles.len()).contains(&((point.x - self.offset_x) as usize))
            && (0..self.tiles[0].len()).contains(&(point.y as usize))
    }

    fn tik(&mut self, point: &Point) -> TikResult {
        let down_point = point.down();

        if !self.contains(&down_point) {
            return TikResult::Finished;
        }

        if !self.is_blocked(&down_point) {
            return TikResult::Falling(down_point);
        }

        let down_point = point.down_left();

        if !self.is_blocked(&down_point) {
            return TikResult::Falling(down_point);
        }

        let down_point = point.down_right();

        if !self.is_blocked(&down_point) {
            return TikResult::Falling(down_point);
        }

        self.set_tile(&point, Tile::Sand);

        TikResult::Settled
    }
}

fn parse_line(line: &str) -> Vec<Point> {
    line.split(" -> ")
        .map(|point_data| {
            let (x, y) = point_data.split_once(',').expect("Invalid input format");

            Point {
                x: x.parse().expect("Invalid input format"),
                y: y.parse().expect("Invalid input Format"),
            }
        })
        .collect()
}

fn find_limits(paths: &Vec<Vec<Point>>) -> Limits {
    let mut limits = Limits {
        min_x: u64::MAX,
        max_x: u64::MIN,
        min_y: u64::MAX,
        max_y: u64::MIN,
    };

    for path in paths {
        for point in path {
            limits.min_x = limits.min_x.min(point.x);
            limits.max_x = limits.max_x.max(point.x);
            limits.min_y = limits.min_y.min(point.y);
            limits.max_y = limits.max_y.max(point.y);
        }
    }

    limits
}

fn part_one(map: &mut Map) -> u64 {
    let start_point = Point { x: 500, y: 0 };
    let mut point = start_point.clone();
    let mut sand_counter = 0;

    loop {
        match map.tik(&point) {
            TikResult::Finished => break,
            TikResult::Falling(new_point) => point = new_point,
            TikResult::Settled => {
                point = start_point.clone();
                sand_counter += 1;
            }
        }
    }

    sand_counter
}

fn main() {
    let mut map = Map::from_file("day14/input.txt");

    println!("Part one: {}", part_one(&mut map));

    map.print();
}
