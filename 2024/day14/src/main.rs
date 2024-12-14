#[derive(Clone)]
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

#[derive(Clone)]
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

    fn simulate(&mut self, map_width: i32, map_height: i32, ticks: i32) {
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

    fn period(&self, map_width: i32, map_height: i32) -> i32 {
        let x_period = map_width / gcd(self.vel.x.abs(), map_width);
        let y_period = map_height / gcd(self.pos.y, map_height);

        lcm(x_period, y_period)
    }
}

fn main() {
    let map_width = 101;
    let map_height = 103;
    let data = include_str!("../day14.txt");

    let robots = data.lines().map(Robot::from_str).collect::<Vec<_>>();

    let max_ticks = robots
        .iter()
        .map(|robot| robot.period(map_width, map_height))
        .reduce(lcm)
        .expect("No robots found");

    let part1_result = part1(robots.clone(), map_width, map_height, 100);
    let part2_result = part2(robots, map_width, map_height, max_ticks);

    println!("Day 14 Part 1: {:?}", part1_result);
    println!("Day 14 Part 1: {:?}", part2_result);
}

fn part1(mut robots: Vec<Robot>, map_width: i32, map_height: i32, ticks: i32) -> i32 {
    for robot in robots.iter_mut() {
        robot.simulate(map_width, map_height, ticks);
    }

    let map_center_x = (map_width - 1) / 2;
    let map_center_y = (map_height - 1) / 2;

    let mut tl = 0;
    let mut tr = 0;
    let mut bl = 0;
    let mut br = 0;

    for robot in robots.iter() {
        match robot.pos {
            Point { x, y } if x < map_center_x && y < map_center_y => tl += 1,
            Point { x, y } if x > map_center_x && y < map_center_y => tr += 1,
            Point { x, y } if x < map_center_x && y > map_center_y => bl += 1,
            Point { x, y } if x > map_center_x && y > map_center_y => br += 1,
            _ => {}
        }
    }

    tl * tr * bl * br
}

fn part2(mut robots: Vec<Robot>, map_width: i32, map_height: i32, ticks: i32) -> i32 {
    let mut max_streak = 0;
    let mut tick_of_max_streak = 0;

    let mut map = vec![vec![false; map_width as usize]; map_height as usize];

    for tick in 1..ticks {
        for robot in robots.iter_mut() {
            robot.simulate(map_width, map_height, 1);
            map[robot.pos.y as usize][robot.pos.x as usize] = true;
        }

        let longest_streak = map
            .iter()
            .map(|row| {
                row.iter().fold((0, 0), |(current, longest), x| {
                    if *x {
                        (current + 1, longest.max(current + 1))
                    } else {
                        (0, longest)
                    }
                })
            })
            .map(|(_, longest)| longest)
            .max()
            .unwrap();

        if longest_streak > max_streak {
            max_streak = longest_streak;
            tick_of_max_streak = tick;
        }

        for row in map.iter_mut() {
            row.fill(false);
        }
    }

    tick_of_max_streak
}

fn gcd(a: i32, b: i32) -> i32 {
    let (mut a, mut b) = if a < b { (b, a) } else { (a, b) };

    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }

    a
}

fn lcm(a: i32, b: i32) -> i32 {
    (a * b).abs() / gcd(a, b)
}
