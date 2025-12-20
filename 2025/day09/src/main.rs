use std::{cmp::Ordering::*, collections::BTreeMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn flip(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

#[derive(PartialEq)]
enum Outside {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Edge {
    start: Point,
    end: Point,
    dir: Direction,
}

impl Edge {
    fn new(start: Point, end: Point) -> Self {
        let dir = match (start.x.cmp(&end.x), start.y.cmp(&end.y)) {
            (Equal, Greater) => Direction::Up,
            (Equal, Less) => Direction::Down,
            (Less, Equal) => Direction::Right,
            (Greater, Equal) => Direction::Left,
            _ => unreachable!(),
        };

        Self { start, end, dir }
    }

    fn flip(&self) -> Self {
        Self {
            start: self.end,
            end: self.start,
            dir: self.dir.flip(),
        }
    }

    fn contains(&self, point: &Point) -> bool {
        let x_range = self.start.x.min(self.end.x)..=self.start.x.max(self.end.x);
        let y_range = self.start.y.min(self.end.y)..=self.start.y.max(self.end.y);

        x_range.contains(&point.x) && y_range.contains(&point.y)
    }
}

struct Map {
    vertical_edges: BTreeMap<u64, Vec<Edge>>,
    horizontal_edges: BTreeMap<u64, Vec<Edge>>,
    out: Outside,
}

impl Map {
    fn new(edges: Vec<Edge>) -> Self {
        let mut vertical_edges: BTreeMap<u64, Vec<Edge>> = BTreeMap::new();
        let mut horizontal_edges: BTreeMap<u64, Vec<Edge>> = BTreeMap::new();

        for edge in edges.into_iter() {
            match edge.dir {
                Direction::Up | Direction::Down => vertical_edges
                    .entry(edge.start.x)
                    .and_modify(|edges: &mut Vec<Edge>| edges.push(edge))
                    .or_insert_with(|| vec![edge]),
                Direction::Left | Direction::Right => horizontal_edges
                    .entry(edge.start.y)
                    .and_modify(|edges: &mut Vec<Edge>| edges.push(edge))
                    .or_insert_with(|| vec![edge]),
            };
        }

        let first_edge = vertical_edges.iter().next().unwrap().1.first().unwrap();

        let out = if first_edge.dir == Direction::Up {
            Outside::Left
        } else {
            Outside::Right
        };

        Self {
            vertical_edges,
            horizontal_edges,
            out,
        }
    }

    fn expand_corners(&self) -> BTreeMap<Point, Vec<Direction>> {
        let mut points = BTreeMap::new();

        for (_, edges) in self
            .vertical_edges
            .iter()
            .chain(self.horizontal_edges.iter())
        {
            for edge in edges {
                let mut dir = match &edge.dir {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                };

                if self.out == Outside::Right {
                    dir = dir.flip();
                }

                points
                    .entry(edge.start)
                    .and_modify(|dirs: &mut Vec<Direction>| dirs.push(dir))
                    .or_insert_with(|| vec![dir]);
                points
                    .entry(edge.end)
                    .and_modify(|dirs: &mut Vec<Direction>| dirs.push(dir))
                    .or_insert_with(|| vec![dir]);
            }
        }

        points
    }

    fn contains(&self, point1: &Point, point2: &Point) -> bool {
        let min_x = point1.x.min(point2.x);
        let max_x = point1.x.max(point2.x);
        let min_y = point1.y.min(point2.y);
        let max_y = point1.y.max(point2.y);

        let top_left = Point { x: min_x, y: min_y };
        let top_right = Point { x: max_x, y: min_y };
        let bottom_left = Point { x: min_x, y: max_y };
        let bottom_right = Point { x: max_x, y: max_y };

        let edges = vec![
            (top_left, top_right),
            (bottom_left, bottom_right),
            (top_left, bottom_left),
            (top_right, bottom_right),
        ];

        edges
            .into_iter()
            .filter(|(p1, p2)| p1.x != p2.x || p1.y != p2.y)
            .map(|(p1, p2)| Edge::new(p1, p2))
            .all(|edge| self.contains_edge(&edge))
    }

    fn contains_edge(&self, edge: &Edge) -> bool {
        match edge.dir {
            Direction::Up => self.contains_down_edge(&edge.flip()),
            Direction::Down => self.contains_down_edge(edge),
            Direction::Left => self.contains_right_edge(&edge.flip()),
            Direction::Right => self.contains_right_edge(edge),
        }
    }

    fn contains_right_edge(&self, edge: &Edge) -> bool {
        assert!(edge.dir == Direction::Right);

        let edges = self.vertical_edges.range(edge.start.x..=edge.end.x);

        let mut point = edge.start;
        for (x, edges) in edges {
            point.x = *x;
            for edge in edges {
                if edge.contains(&point) {
                    return false;
                }
            }
        }

        true
    }

    fn contains_down_edge(&self, edge: &Edge) -> bool {
        assert!(edge.dir == Direction::Down);

        let edges = self.horizontal_edges.range(edge.start.y..=edge.end.y);

        let mut point = edge.start;
        for (y, edges) in edges {
            point.y = *y;
            for edge in edges {
                if edge.contains(&point) {
                    return false;
                }
            }
        }

        true
    }
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            Point {
                x: x.parse::<u64>().unwrap(),
                y: y.parse::<u64>().unwrap(),
            }
        })
        .collect()
}

fn build_edges(points: &[Point]) -> Vec<Edge> {
    let mut edges = points
        .windows(2)
        .map(|points| Edge::new(points[0], points[1]))
        .collect::<Vec<Edge>>();

    edges.push(Edge::new(*points.last().unwrap(), *points.first().unwrap()));

    edges
}

fn part1(points: &[Point]) -> u64 {
    let mut largest_area = 0;

    for i in 0..points.len() - 1 {
        for j in i + 1..points.len() {
            let width = points[i].x.abs_diff(points[j].x) + 1;
            let height = points[i].y.abs_diff(points[j].y) + 1;
            largest_area = largest_area.max(width * height);
        }
    }

    largest_area
}

fn part2(points: &[Point]) -> u64 {
    let edges = build_edges(points);
    let map = Map::new(edges.clone());

    let expanded_corners = map.expand_corners();

    let expanded_points: Vec<Point> = points
        .iter()
        .map(|p| {
            let mut p = *p;
            for dir in expanded_corners.get(&p).unwrap() {
                match dir {
                    Direction::Up => p.y -= 1,
                    Direction::Down => p.y += 1,
                    Direction::Left => p.x -= 1,
                    Direction::Right => p.x += 1,
                }
            }

            p
        })
        .collect();

    let edges = build_edges(&expanded_points);
    let map = Map::new(edges);

    let mut largest_area = 0;

    for i in 0..points.len() - 1 {
        for j in i + 1..points.len() {
            let width = points[i].x.abs_diff(points[j].x) + 1;
            let height = points[i].y.abs_diff(points[j].y) + 1;
            let area = width * height;
            if area > largest_area && map.contains(&points[i], &points[j]) {
                largest_area = area;
            }
        }
    }

    largest_area
}

fn main() {
    let input = include_str!("../day09.txt");

    let points = parse_input(input);

    println!("Day 09 Part 1: {}", part1(&points));
    println!("Day 09 Part 2: {}", part2(&points));
}
