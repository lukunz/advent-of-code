struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn from_str(s: &str) -> Self {
        let (x, y) = s.split_once(",").expect("Invalid point format");

        Self {
            x: x.parse().expect("Invalid x coordinate"),
            y: y.parse().expect("Invalid y coordinate"),
        }
    }
}

struct Robot {
    pos: Point,
    vel: Point,
}

impl Robot {
    fn from_str(s: &str) -> Self {
        let (pos_data, vel_data) = s.split_once(" ").expect("Invalid robot format");
        assert!(pos_data.starts_with("p"));
        assert!(vel_data.starts_with("v"));

        let (_, pos_data) = pos_data
            .split_once("=")
            .expect("Invalid robot position format");
        let (_, vel_data) = vel_data
            .split_once("=")
            .expect("Invalid robot velocity format");

        Self {
            pos: Point::from_str(pos_data),
            vel: Point::from_str(vel_data),
        }
    }

    fn sumulate(&mut self, map_width: i32, map_height: i32, ticks: i32) {
        self.pos.x += self.vel.x * ticks;
        self.pos.y += self.vel.y * ticks;

        self.pos.x %= map_width;
        self.pos.y %= map_height;

        if self.pos.x < 0 {
            self.pos.x += map_width;
        }

        if self.pos.y < 0 {
            self.pos.y += map_height;
        }
    }
}

fn main() {
    let ticks = 100;
    let map_width = 101;
    let map_height = 103;
    let data = include_str!("../day14.txt");

    let mut robots = data.lines().map(Robot::from_str).collect::<Vec<_>>();

    for robot in robots.iter_mut() {
        robot.sumulate(map_width, map_height, ticks);
    }

    let map_center_x = (map_width - 1) / 2;
    let map_center_y = (map_height - 1) / 2;

    let quadrants = robots
        .iter()
        .fold((0, 0, 0, 0), |(tl, tr, bl, br), robot| match robot.pos {
            Point { x, y } if x < map_center_x && y < map_center_y => (tl + 1, tr, bl, br),
            Point { x, y } if x > map_center_x && y < map_center_y => (tl, tr + 1, bl, br),
            Point { x, y } if x < map_center_x && y > map_center_y => (tl, tr, bl + 1, br),
            Point { x, y } if x > map_center_x && y > map_center_y => (tl, tr, bl, br + 1),
            _ => (tl, tr, bl, br),
        });

    let part1_result = quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3;

    println!("Day 14 Part 1: {:?}", part1_result);
}
