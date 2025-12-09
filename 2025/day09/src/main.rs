struct Point {
    x: u64,
    y: u64,
}

fn main() {
    let input = include_str!("../day09.txt");

    let points: Vec<Point> = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            Point {
                x: x.parse::<u64>().unwrap(),
                y: y.parse::<u64>().unwrap(),
            }
        })
        .collect();

    let mut largest_area = 0;

    for i in 0..points.len() - 1 {
        for j in i + 1..points.len() {
            let width = points[i].x.abs_diff(points[j].x) + 1;
            let height = points[i].y.abs_diff(points[j].y) + 1;
            largest_area = largest_area.max(width * height);
        }
    }

    println!("Day 09 Part 1: {}", largest_area);
}
