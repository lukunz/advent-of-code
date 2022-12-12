use std::collections::VecDeque;
use std::fs;

type Point = (i64, i64);

struct MapSquare {
    height: i64,
    distance: i64,
}

struct Map {
    grid: Vec<Vec<MapSquare>>,
    start: Point,
    end: Point,
    width: i64,
    height: i64,
}

impl Map {
    fn from_file(file: &str) -> Self {
        let data = fs::read_to_string(file).expect("Can't read input file");

        let mut grid = Vec::new();
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
                        row.push(MapSquare {
                            height: 25,
                            distance: i64::MAX,
                        });
                    }
                    'S' => {
                        start = Some((x, y));
                        row.push(MapSquare {
                            height: 0,
                            distance: i64::MAX,
                        });
                    }
                    'a'..='z' => row.push(MapSquare {
                        height: height_char as i64 - 97,
                        distance: i64::MAX,
                    }),
                    _ => panic!("Invalid map format"),
                }
                x += 1;
            }
            grid.push(row);
            y += 1;
        }

        Map {
            grid,
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
        self.grid[point.1 as usize][point.0 as usize].height
    }

    fn distance(&self, point: &Point) -> i64 {
        self.grid[point.1 as usize][point.0 as usize].distance
    }

    fn set_distance(&mut self, point: &Point, distance: i64) {
        self.grid[point.1 as usize][point.0 as usize].distance = distance;
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
            .filter(|p| self.contains(p) && self.height(&point) - 1 <= self.height(p))
            .collect()
    }

    fn calculate_distances(&mut self) {
        let mut queue = VecDeque::new();

        self.set_distance(&self.end.clone(), 0);
        queue.push_back(self.end);

        while !queue.is_empty() {
            let point = queue.pop_front().unwrap();
            let distance = self.distance(&point);

            for neighbour in self.neighbours(&point) {
                if self.distance(&neighbour) > distance + 1 {
                    self.set_distance(&neighbour, distance + 1);
                    queue.push_back(neighbour);
                }
            }
        }
    }
}

fn main() {
    let mut map = Map::from_file("day12/input.txt");
    map.calculate_distances();
    println!("Part one: {}", map.distance(&map.start));

    let mut shortest = i64::MAX;
    for x in 0..map.width {
        for y in 0..map.height {
            let point = &(x, y);
            if map.height(point) == 0 {
                shortest = i64::min(shortest, map.distance(point));
            }
        }
    }

    println!("Part two: {}", shortest);
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
