use std::fs;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    fn from_xyz(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    fn from_str(data: &str) -> Self {
        let mut data = data.split(", ");
        Point::from_xyz(
            data.next().unwrap().trim().parse().unwrap(),
            data.next().unwrap().trim().parse().unwrap(),
            data.next().unwrap().trim().parse().unwrap(),
        )
    }
}

#[derive(Debug)]
struct Hail {
    location: Point,
    velocity: Point,
}

impl Hail {
    fn intersect(&self, other: &Self) -> Option<Point> {
        let cross = self.velocity.y * other.velocity.x - self.velocity.x * other.velocity.y;

        if cross == 0.0 {
            return None;
        }

        let s = (self.velocity.x * (other.location.y - self.location.y)
            + self.location.x * self.velocity.y
            - other.location.x * self.velocity.y)
            / cross;
        let t = (other.location.x - self.location.x + s * other.velocity.x) / self.velocity.x;

        if t <= 0.0 || s <= 0.0 {
            return None;
        }

        Some(Point {
            x: self.location.x + t * self.velocity.x,
            y: self.location.y + t * self.velocity.y,
            z: self.location.z,
        })
    }
}

fn is_in_area(min: &Point, max: &Point, point: &Point) -> bool {
    min.x <= point.x && point.x <= max.x && min.y <= point.y && point.y <= max.y
}

fn parse_input(data: &str) -> Vec<Hail> {
    data.lines()
        .map(|line| {
            let (location_str, velocity_str) = line.split_once(" @ ").unwrap();
            let location = Point::from_str(location_str);
            let velocity = Point::from_str(velocity_str);

            Hail { location, velocity }
        })
        .collect()
}

fn main() {
    let data = fs::read_to_string("day24.txt").expect("Can't read input file");
    let hailstones = parse_input(&data);

    let test_area_min = Point::from_xyz(200000000000000.0, 200000000000000.0, 0.0);
    let test_area_max = Point::from_xyz(400000000000000.0, 400000000000000.0, 0.0);
    let mut counter = 0;

    for (index, hail) in hailstones.iter().enumerate() {
        for other_hail in hailstones.iter().skip(index + 1) {
            if let Some(p) = hail.intersect(other_hail) {
                if is_in_area(&test_area_min, &test_area_max, &p) {
                    counter += 1;
                }
            }
        }
    }

    println!("Day 24 Part 1: {}", counter);
}
