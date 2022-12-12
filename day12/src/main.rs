use std::collections::{BTreeMap, BTreeSet};
use std::fs;

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn as_tuple(&self) -> (i64, i64) {
        (self.x, self.y)
    }

    fn from_tuple(tuple: &(i64, i64)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

struct Map {
    points: Vec<Vec<i64>>,
    start: Point,
    end: Point,
    width: i64,
    height: i64,
}

impl Map {
    fn from_file(file: &str) -> Self {
        let data = fs::read_to_string(file).expect("Can't read input file");

        let mut points = Vec::new();
        let mut start: Option<Point> = None;
        let mut end: Option<Point> = None;
        let mut x: i64 = 0;
        let mut y: i64 = 0;

        for line in data.lines() {
            let mut row = Vec::new();
            x = 0;
            for height_char in line.chars() {
                match height_char {
                    'E' => {
                        end = Some(Point { x, y });
                        row.push(25);
                    }
                    'S' => {
                        start = Some(Point { x, y });
                        row.push(0);
                    }
                    'a'..='z' => row.push(height_char.to_digit(36).unwrap() as i64 - 10),
                    _ => panic!("Invalid map format"),
                }
                x += 1;
            }
            points.push(row);
            y += 1;
        }

        Map {
            points,
            start: start.expect("Invalid map format"),
            end: end.expect("Invalid map format"),
            width: x,
            height: y,
        }
    }

    fn contains(&self, point: &Point) -> bool {
        if point.x < 0 || point.y < 0 {
            return false;
        }

        (0..self.width).contains(&point.x) && (0..self.height).contains(&point.y)
    }

    fn height(&self, point: &Point) -> i64 {
        self.points[point.y as usize][point.x as usize]
    }

    fn neighbours(&self, point: &Point) -> Vec<Point> {
        let edges = vec![
            Point {
                x: point.x + 1,
                y: point.y,
            },
            Point {
                x: point.x,
                y: point.y + 1,
            },
            Point {
                x: point.x - 1,
                y: point.y,
            },
            Point {
                x: point.x,
                y: point.y - 1,
            },
        ];

        edges
            .into_iter()
            .filter(|p| self.contains(p) && self.height(&point) + 1 >= self.height(p))
            .collect()
    }

    fn shortest_path(&self) -> i64 {
        let mut distances = BTreeMap::new();
        let mut queue = BTreeSet::new();

        for x in 0..self.width {
            for y in 0..self.height {
                let p = (x, y);
                queue.insert(p);
                distances.insert(p, i64::MAX);
            }
        }

        distances.insert(self.start.as_tuple(), 0);

        while !queue.is_empty() {
            let (point, distance) = distances
                .iter()
                .filter(|(p, _)| queue.contains(p))
                .reduce(|(nearest_point, nearest_distance), (point, distance)| {
                    if nearest_distance < distance {
                        (nearest_point, nearest_distance)
                    } else {
                        (point, distance)
                    }
                })
                .expect("not possible");
            queue.remove(point);

            if *distance == i64::MAX {
                continue;
            }

            let point = point.clone();
            let distance = distance.clone();

            for neighbour in self.neighbours(&Point::from_tuple(&point)) {
                if queue.contains(&neighbour.as_tuple()) {
                    let new_distance = distance + 1;

                    if new_distance < *distances.get(&neighbour.as_tuple()).expect("impossible") {
                        distances.insert(neighbour.as_tuple(), new_distance);
                    }
                }
            }
        }

        *distances.get(&self.end.as_tuple()).expect("impossible")
    }
}

fn main() {
    let map = Map::from_file("day12/input.txt");
    let shortest_path_length = map.shortest_path();
    println!("{}", shortest_path_length);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains() {
        let map = Map::from_file("input-small.txt");

        assert!(map.contains(&Point { x: 0, y: 0 }));
        assert!(map.contains(&Point { x: 1, y: 0 }));
        assert!(map.contains(&Point { x: 0, y: 1 }));
        assert!(map.contains(&Point { x: 1, y: 1 }));

        assert!(!map.contains(&Point { x: -1, y: 1 }));
        assert!(!map.contains(&Point { x: 0, y: 100 }));
    }

    #[test]
    fn test_height() {
        let map = Map::from_file("input-small.txt");

        assert_eq!(0, map.height(&Point { x: 0, y: 0 }));
        assert_eq!(0, map.height(&Point { x: 1, y: 0 }));
        assert_eq!(0, map.height(&Point { x: 0, y: 1 }));
        assert_eq!(1, map.height(&Point { x: 1, y: 1 }));
    }

    #[test]
    fn test_neighbours() {
        let map = Map::from_file("input-small.txt");

        let neighbours = map.neighbours(&Point { x: 0, y: 0 });

        assert_eq!(2, neighbours.len());
    }
}
