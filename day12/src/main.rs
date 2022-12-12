use std::collections::{BTreeMap, VecDeque};
use std::fs;

type Point = (i64, i64);

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
                        end = Some((x, y));
                        row.push(25);
                    }
                    'S' => {
                        start = Some((x, y));
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
        if point.0 < 0 || point.1 < 0 {
            return false;
        }

        (0..self.width).contains(&point.0) && (0..self.height).contains(&point.1)
    }

    fn height(&self, point: &Point) -> i64 {
        self.points[point.1 as usize][point.0 as usize]
    }

    fn neighbours(&self, point: &Point) -> Vec<Point> {
        let edges = vec![
            (point.0 + 1, point.1),
            (point.0, point.1 + 1),
            (point.0 - 1, point.1),
            (point.0, point.1 - 1),
        ];

        edges
            .into_iter()
            .filter(|p| self.contains(p) && self.height(&point) + 1 >= self.height(p))
            .collect()
    }

    fn shortest_path(&self, start: &Point) -> i64 {
        let mut distances = BTreeMap::new();
        let mut queue = VecDeque::new();

        for x in 0..self.width {
            for y in 0..self.height {
                distances.insert((x, y), i64::MAX);
            }
        }

        distances.insert(*start, 0);
        queue.push_back(*start);

        while !queue.is_empty() {
            let point = queue.pop_front().unwrap();
            let distance = distances.get(&point).unwrap().clone();

            for neighbour in self.neighbours(&point) {
                if distances.get(&neighbour).unwrap() > &(distance + 1) {
                    distances.insert(neighbour, distance + 1);
                    queue.push_back(neighbour);
                }
            }
        }

        *distances.get(&self.end).unwrap()
    }
}

fn main() {
    let map = Map::from_file("day12/input.txt");
    let shortest_path_length = map.shortest_path(&map.start);
    println!("Part one: {}", shortest_path_length);

    let mut points = Vec::new();
    for x in 0..map.width {
        for y in 0..map.height {
            points.push((x, y));
        }
    }

    let mut a = 0;

    let shortest = points
        .iter()
        .filter(|p| map.height(p) == 0)
        .map(|p| {
            a += 1;
            map.shortest_path(p)
        })
        .min();

    println!("Part two: {}", shortest.unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains() {
        let map = Map::from_file("input-small.txt");

        assert!(map.contains(&(0, 0)));
        assert!(map.contains(&(0, 1)));
        assert!(map.contains(&(1, 0)));
        assert!(map.contains(&(1, 1)));

        assert!(!map.contains(&(-1, 1)));
        assert!(!map.contains(&(0, 100)));
    }

    #[test]
    fn test_height() {
        let map = Map::from_file("input-small.txt");

        assert_eq!(0, map.height(&(0, 0)));
        assert_eq!(0, map.height(&(1, 0)));
        assert_eq!(0, map.height(&(0, 1)));
        assert_eq!(1, map.height(&(1, 1)));
    }

    #[test]
    fn test_neighbours() {
        let map = Map::from_file("input-small.txt");

        let neighbours = map.neighbours(&(0, 0));

        assert_eq!(2, neighbours.len());
    }
}
