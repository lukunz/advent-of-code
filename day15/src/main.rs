use std::fs;

#[derive(Debug, PartialEq)]
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

fn print(sensors: &Vec<Sensor>) {
    let (min_x, max_x, min_y, max_y) = sensors
        .iter()
        .fold((i64::MAX, i64::MIN, i64::MAX, i64::MIN), |(min_x, max_x, min_y, max_y), sensor| {
            (
                min_x.min(sensor.location.x - sensor.distance),
                max_x.max(sensor.location.x + sensor.distance),
                min_y.min(sensor.location.y - sensor.distance),
                max_y.max(sensor.location.y + sensor.distance),
            )
        });

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let p = Point { x, y };
            if sensors.iter().any(|sensor| sensor.covers(&p)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn part_one(sensors: &Vec<Sensor>, y: i64) -> u64 {
    let (min_x, max_x) = sensors
        .iter()
        .fold((i64::MAX, i64::MIN), |(min_x, max_x), sensor| {
            (
                min_x.min(sensor.location.x - sensor.distance),
                max_x.max(sensor.location.x + sensor.distance),
            )
        });

    (min_x..=max_x)
        .map(|x| {
            let p = Point { x, y };
            if sensors.iter().any(|sensor| sensor.closest_beacon == p) {
                return 0
            }

            if sensors.iter().any(|sensor| sensor.covers(&p)) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn main() {
    let sensors = parse_file("day15/input.txt");
    // print(&sensors);

    println!("Part one: {}", part_one(&sensors, 2000000));
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
}
