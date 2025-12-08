use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn from_line(line: &str) -> Self {
        let mut parts = line.split(',').map(|part| part.parse::<i32>().unwrap());
        let x = parts.next().unwrap();
        let y = parts.next().unwrap();
        let z = parts.next().unwrap();

        Self { x, y, z }
    }

    fn distance(&self, other: &Self) -> f64 {
        let x = (self.x - other.x) as i64;
        let y = (self.y - other.y) as i64;
        let z = (self.z - other.z) as i64;

        ((x.pow(2) + y.pow(2) + z.pow(2)) as f64).sqrt()
    }
}

#[derive(Debug)]
struct Edge {
    point1: Point,
    point2: Point,
    distance: f64,
}

fn parse_input(input: &str) -> Vec<Point> {
    input.lines().map(Point::from_line).collect()
}

fn calculate_edges(points: &[Point]) -> Vec<Edge> {
    let mut edges: Vec<Edge> = Vec::new();

    for i in 0..points.len() - 1 {
        for j in i + 1..points.len() {
            let distance = points[i].distance(&points[j]);
            edges.push(Edge {
                point1: points[i],
                point2: points[j],
                distance,
            });
        }
    }

    edges.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

    edges
}

fn find_cluster_index(clusters: &Vec<HashSet<&Point>>, point: &Point) -> Option<usize> {
    for (index, cluster) in clusters.iter().enumerate() {
        if cluster.contains(point) {
            return Some(index);
        }
    }

    None
}

fn part1(edges: &[Edge], connection_count: usize) -> usize {
    let mut clusters: Vec<HashSet<&Point>> = Vec::new();

    for edge in edges.iter().take(connection_count) {
        let cluster1_index = find_cluster_index(&clusters, &edge.point1);
        let cluster2_index = find_cluster_index(&clusters, &edge.point2);

        match (cluster1_index, cluster2_index) {
            (Some(i1), Some(i2)) if i1 != i2 => {
                let keep_index = i1.min(i2);
                let remove_index = i1.max(i2);
                let remove_cluster = clusters.remove(remove_index);
                clusters[keep_index].extend(remove_cluster);
            }
            (Some(_), Some(_)) => {}
            (Some(i1), None) => {
                clusters[i1].insert(&edge.point2);
            }
            (None, Some(i2)) => {
                clusters[i2].insert(&edge.point1);
            }
            (None, None) => {
                clusters.push(HashSet::from([&edge.point1, &edge.point2]));
            }
        }
    }

    clusters.sort_by_key(|b| std::cmp::Reverse(b.len()));

    clusters.iter().take(3).map(|c| c.len()).product::<usize>()
}

fn main() {
    let input = include_str!("../day08.txt");

    let points: Vec<Point> = parse_input(input);
    let edges = calculate_edges(&points);

    let part1_result = part1(&edges, 1000);

    println!("Day 08 Part1: {}", part1_result);
}
