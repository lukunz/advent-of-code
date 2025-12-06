use std::ops::Range;

#[derive(PartialEq)]
enum Op {
    Add,
    Mul,
}

impl Op {
    fn from_str(s: &str) -> Self {
        match s.trim() {
            "+" => Self::Add,
            "*" => Self::Mul,
            _ => panic!("Unknown op '{}'", s),
        }
    }

    fn from_char(c: char) -> Self {
        match c {
            '+' => Self::Add,
            '*' => Self::Mul,
            _ => panic!("Unknown op '{}'", c),
        }
    }
}

struct Group {
    op: Op,
    nums: Vec<u64>,
}

impl Group {
    fn execute(&self) -> u64 {
        match self.op {
            Op::Add => self.nums.iter().sum(),
            Op::Mul => self.nums.iter().product(),
        }
    }
}

fn parse_part1(input: &str) -> Vec<Group> {
    let first_line = input.lines().next().unwrap();

    let col_count = first_line.split_whitespace().count();

    let mut data: Vec<Group> = Vec::with_capacity(col_count);

    for _ in 0..col_count {
        data.push(Group {
            op: Op::Add,
            nums: Vec::new(),
        });
    }

    for line in input.lines() {
        let cols = line.split_whitespace();
        for (index, col) in cols.enumerate() {
            if col == "+" || col == "*" {
                data[index].op = Op::from_str(col);
            } else {
                data[index].nums.push(col.parse::<u64>().unwrap());
            }
        }
    }

    data
}

fn parse_part2(input: &str) -> Vec<Group> {
    let data: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    assert!(data.len() > 1);

    let ops_line = &data[data.len() - 1];
    let mut groups: Vec<Group> = Vec::new();
    let mut group_ranges: Vec<Range<usize>> = Vec::new();
    let mut last_op_index = 0;

    for (index, &c) in ops_line.iter().enumerate() {
        if c == '+' || c == '*' {
            if index > 0 {
                group_ranges.push(last_op_index..index - 1);
                last_op_index = index;
            }
            groups.push(Group {
                op: Op::from_char(c),
                nums: Vec::new(),
            });
        }
    }

    group_ranges.push(last_op_index..ops_line.len());

    for (group_index, range) in group_ranges.iter().enumerate() {
        let mut nums: Vec<Vec<char>> = Vec::with_capacity(range.len());

        for _ in range.clone() {
            nums.push(Vec::new());
        }

        for line in data[0..data.len() - 1].iter() {
            for (num_index, data_index) in range.clone().enumerate() {
                nums[num_index].push(line[data_index]);
            }
        }

        groups[group_index].nums.extend(
            nums.iter()
                .map(|s| s.iter().collect::<String>())
                .map(|s| s.trim().parse::<u64>().unwrap()),
        );
    }

    groups
}

fn main() {
    let input = include_str!("../day06.txt");

    let result_part1: u64 = parse_part1(input).iter().map(|group| group.execute()).sum();
    println!("Day 06 Part 1: {}", result_part1);

    let result_part2: u64 = parse_part2(input).iter().map(|group| group.execute()).sum();
    println!("Day 06 Part 2: {}", result_part2);
}
