use std::fs;

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    inspected_item_count: u64,
    operation: Operation,
    test: u64,
    success_target: usize,
    fail_target: usize,
}

#[derive(Debug)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Square
}

fn parse_monkey(input: &str) -> Monkey {
    let mut monkey = Monkey {
        items: Vec::new(),
        inspected_item_count: 0,
        operation: Operation::Square,
        test: 1,
        success_target: 0,
        fail_target: 0,
    };

    let lines = input.lines().map(|line| line.split_whitespace().collect::<Vec<&str>>());

    for line in lines {
        match line[0] {
            "Monkey" => {},
            "Starting" => monkey.items = parse_items(&line[2..]),
            "Operation:" => monkey.operation = parse_operation(&line[4..]),
            "Test:" => monkey.test = line[3].parse().expect("Can't parse input"),
            "If" => match line[1] {
                "true:" => monkey.success_target = line[5].parse().expect("Can't parse input"),
                "false:" => monkey.fail_target = line[5].parse().expect("Can't parse input"),
                _ => panic!("Can't parse input '{}'", line[1]),
            }
            _ => panic!("Can't parse input '{}'", line[0]),
        }
    }

    monkey
}

fn parse_items(input: &[&str]) -> Vec<u64> {
    input
        .iter()
        .map(|item| item.trim_end_matches(','))
        .map(|item| item.parse().expect("Can't parse input"))
        .collect()
}

fn parse_operation(input: &[&str]) -> Operation {
    match input[0] {
        "+" =>  Operation::Add(input[1].parse().expect("Can't parse input")),
        "*" => match input[1] {
            "old" => Operation::Square,
            number => Operation::Multiply(number.parse().expect("Can't parse input")),
        }
        _ => panic!("Can't parse input"),
    }
}

fn run_round(monkeys: &mut Vec<Monkey>, calm_down: bool) {
    let mut d = 1;
    for monkey in monkeys.into_iter() {
        d *= monkey.test;
    }

    for i in 0..monkeys.len() {
        for item_i in 0..monkeys[i].items.len() {
            let worry_amount = monkeys[i].items[item_i];
            let mut new_worry_amount = match monkeys[i].operation {
                Operation::Add(amount) => worry_amount + amount,
                Operation::Multiply(factor) => worry_amount * factor,
                Operation::Square => worry_amount * worry_amount,
            };

            if calm_down {
                new_worry_amount /= 3;
            }

            new_worry_amount %= d;

            if new_worry_amount % monkeys[i].test == 0 {
                let target_index = monkeys[i].success_target;
                monkeys[target_index].items.push(new_worry_amount);
            } else {
                let target_index = monkeys[i].fail_target;
                monkeys[target_index].items.push(new_worry_amount);
            }

            monkeys[i].inspected_item_count += 1;
        }

        monkeys[i].items.clear();
    }
}

fn part_one(data: &str) {
    let mut monkeys: Vec<Monkey> = data.split("\n\n").map(|input| parse_monkey(input)).collect();

    for _ in 0..20 {
        run_round(&mut monkeys, true);
    }

    println!("Part one:");
    println!("=========");
    println!();

    for monkey in &monkeys {
        println!("{:?}", monkey);
    }

    let mut item_counts: Vec<u64> = monkeys.iter().map(|monkey| monkey.inspected_item_count).collect();
    item_counts.sort();
    let monkey_business = item_counts.into_iter().rev().take(2).reduce(|a, b| a * b).unwrap();

    println!("{}", monkey_business);
}

fn part_two(data: &str) {
    let mut monkeys: Vec<Monkey> = data.split("\n\n").map(|input| parse_monkey(input)).collect();

    for _ in 0..10000 {
        run_round(&mut monkeys, false);
    }

    println!("Part two:");
    println!("=========");
    println!();

    for monkey in &monkeys {
        println!("{:?}", monkey);
    }

    let mut item_counts: Vec<u64> = monkeys.iter().map(|monkey| monkey.inspected_item_count).collect();
    item_counts.sort();
    let monkey_business = item_counts.into_iter().rev().take(2).reduce(|a, b| a * b).unwrap();

    println!("{}", monkey_business);
}

fn main() {
    let data = fs::read_to_string("day11/input.txt").expect("Can't read input file");
    part_one(&data);
    println!();
    println!();
    part_two(&data);
}
