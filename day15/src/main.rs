use std::collections::HashSet;
use std::fs;

#[derive(Debug, PartialEq, Hash, Eq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new() -> Point {
        Point { x: 0, y: 0 }
    }

    fn distance(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

struct Sensor {
    location: Point,
    closest_beacon: Point,
    distance: i64,
}

impl Sensor {
    fn new(location: Point, closest_beacon: Point) -> Self {
        let distance = location.distance(&closest_beacon);
        Self {
            location,
            closest_beacon,
            distance,
        }
    }
    fn covers(&self, point: &Point) -> bool {
        self.distance >= self.location.distance(point)
    }
}

fn parse_point(input: &str) -> Point {
    let mut p = Point::new();

    for item in input.split_whitespace() {
        match item.split_once('=') {
            Some(("x", number)) => {
                p.x = number.trim_matches(',').parse().expect("Input error");
            }
            Some(("y", number)) => p.y = number.parse().expect("Input error"),
            _ => {}
        }
    }

    p
}

fn parse_line(line: &str) -> Sensor {
    let (left, right) = line.split_once(':').expect("Invalid input");

    Sensor::new(parse_point(left), parse_point(right))
}

fn parse_file(file: &str) -> Vec<Sensor> {
    let data = fs::read_to_string(file).expect("Can't read input file");

    data.lines().map(|line| parse_line(line)).collect()
}

fn part_one(sensors: &Vec<Sensor>, y: i64) -> i64 {
    let (min_x, max_x) = sensors
        .iter()
        .fold((i64::MAX, i64::MIN), |(min_x, max_x), sensor| {
            (
                min_x.min(sensor.location.x - sensor.distance),
                max_x.max(sensor.location.x + sensor.distance),
            )
        });

    let mut beacons = HashSet::new();

    for sensor in sensors {
        if sensor.closest_beacon.y == y {
            beacons.insert(&sensor.closest_beacon);
        }
    }

    let beacon_count = beacons.len() as i64;

    let mut x = min_x;
    let mut tile_count = 0;

    while x <= max_x {
        let p = Point { x, y };
        let mut new_x = None;

        for sensor in sensors {
            if sensor.covers(&p) {
                new_x = Some(
                    sensor.location.x + 1 + (sensor.distance - (sensor.location.y - p.y).abs()),
                );
                break;
            }
        }

        if let Some(new_x) = new_x {
            x = new_x.min(max_x);
            tile_count += x - p.x;
        } else {
            x += 1;
        }
    }

    tile_count - beacon_count
}

fn part_two(sensors: &Vec<Sensor>, limit: i64) -> i64 {
    for y in 0..=limit {
        let mut x = 0;

        while x <= limit {
            let p = Point { x, y };
            let mut new_x = None;

            for sensor in sensors {
                if sensor.covers(&p) {
                    new_x = Some(
                        sensor.location.x + 1 + (sensor.distance - (sensor.location.y - p.y).abs()),
                    );
                    break;
                }
            }

            if let Some(new_x) = new_x {
                x = new_x.min(limit + 1);
            } else {
                return p.x * 4000000 + p.y;
            }
        }
    }

    0
}

fn main() {
    let sensors = parse_file("day15/input.txt");

    println!("Part one: {}", part_one(&sensors, 2000000));
    println!("Part two: {}", part_two(&sensors, 4000000));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_point() {
        let point1 = parse_point("Sensor at x=2, y=18");
        let point2 = parse_point("closest beacon is at x=-2, y=15");

        assert_eq!(Point { x: 2, y: 18 }, point1);
        assert_eq!(Point { x: -2, y: 15 }, point2);
    }

    #[test]
    fn test_part_one_small() {
        let sensors = parse_file("input-small.txt");

        assert_eq!(26, part_one(&sensors, 10));
    }

    #[test]
    fn test_part_one() {
        let sensors = parse_file("input.txt");

        assert_eq!(4582667, part_one(&sensors, 2_000_000));
    }

    #[test]
    fn test_part_two_small() {
        let sensors = parse_file("input-small.txt");

        assert_eq!(56000011, part_two(&sensors, 20));
    }
}
