use std::cmp::max;
use std::collections::{HashMap, VecDeque};
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Signal {
    Low,
    High,
}

#[derive(Debug, PartialEq)]
enum FlipFlopState {
    On,
    Off,
}

#[derive(Debug)]
enum Module {
    Broadcaster,
    FlipFlop(FlipFlopState),
    Conjunction(HashMap<String, Signal>),
}

#[derive(Debug)]
struct Connection {
    module: Module,
    outputs: Vec<String>,
}

type Connections = HashMap<String, Connection>;

fn parse_input(data: &str) -> Connections {
    let mut connections: Connections = HashMap::new();
    let mut conjunction_labels: Vec<&str> = Vec::new();

    for line in data.lines() {
        let (module_str, output_str) = line.split_once(" -> ").unwrap();

        let (label, module) = match module_str.get(..1) {
            Some("%") => (
                module_str.get(1..).unwrap(),
                Module::FlipFlop(FlipFlopState::Off),
            ),
            Some("&") => {
                let label = module_str.get(1..).unwrap();
                conjunction_labels.push(label);
                (label, Module::Conjunction(HashMap::new()))
            }
            _ => (module_str, Module::Broadcaster),
        };

        let outputs: Vec<String> = output_str.split(", ").map(|s| s.into()).collect();

        connections.insert(label.into(), Connection { module, outputs });
    }

    for label in conjunction_labels {
        let input_label = find_inputs(&connections, label);

        let conjunction = connections.get_mut(label).unwrap();
        if let Module::Conjunction(inputs) = &mut conjunction.module {
            for input in input_label {
                inputs.insert(input.clone(), Signal::Low);
            }
        }
    }

    connections
}

fn find_inputs(connections: &Connections, label: &str) -> Vec<String> {
    let input_label: Vec<String> = connections
        .iter()
        .filter_map(|(l, c)| {
            if c.outputs.contains(&label.into()) {
                Some(l.clone())
            } else {
                None
            }
        })
        .collect();
    input_label
}

fn send_signal(
    sender: &str,
    receivers: &Vec<String>,
    signal: Signal,
    queue: &mut VecDeque<(String, Signal, String)>,
) {
    for receiver in receivers {
        queue.push_back((sender.into(), signal, receiver.clone()));
    }
}

fn process_signal(
    sender: &str,
    receiver: &str,
    signal: Signal,
    connection: &mut Connection,
    queue: &mut VecDeque<(String, Signal, String)>,
) {
    match &mut connection.module {
        Module::Broadcaster => {
            send_signal(receiver, &connection.outputs, signal, queue);
        }
        Module::FlipFlop(state) => {
            if signal == Signal::Low {
                if state == &FlipFlopState::Off {
                    send_signal(receiver, &connection.outputs, Signal::High, queue);
                    connection.module = Module::FlipFlop(FlipFlopState::On);
                } else {
                    send_signal(receiver, &connection.outputs, Signal::Low, queue);
                    connection.module = Module::FlipFlop(FlipFlopState::Off);
                }
            }
        }
        Module::Conjunction(state) => {
            state.insert(sender.into(), signal);

            if state.iter().all(|(_, signal)| signal == &Signal::High) {
                send_signal(receiver, &connection.outputs, Signal::Low, queue);
            } else {
                send_signal(receiver, &connection.outputs, Signal::High, queue);
            }
        }
    }
}

fn prime_factors(mut n: u64) -> HashMap<u64, u64> {
    let mut factors: Vec<u64> = Vec::new();

    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }

    for i in 3..(n as f64).sqrt() as u64 {
        while n % i == 0 {
            factors.push(i);
            n /= i;
        }
    }

    if n > 2 {
        factors.push(n);
    }

    let mut result: HashMap<u64, u64> = HashMap::new();

    for f in factors {
        if let Some(f_count) = result.get(&f) {
            result.insert(f, f_count + 1);
        } else {
            result.insert(f, 1);
        }
    }

    result
}

fn find_lcm(numbers: &[u64]) -> u64 {
    let mut all_factors: HashMap<u64, u64> = HashMap::new();

    for factors in numbers.iter().copied().map(prime_factors) {
        for (factor, f_count) in factors {
            if let Some(f_count2) = all_factors.get(&factor) {
                all_factors.insert(factor, max(f_count, *f_count2));
            } else {
                all_factors.insert(factor, f_count);
            }
        }
    }

    all_factors
        .iter()
        .map(|(factor, count)| factor.pow(*count as u32))
        .product()
}

fn solve_part1(data: &str) -> i32 {
    let mut connections = parse_input(data);
    let mut queue: VecDeque<(String, Signal, String)> = VecDeque::new();
    let mut low_counter = 0;
    let mut high_counter = 0;

    for _ in 0..1000 {
        queue.push_back(("button".into(), Signal::Low, "broadcaster".into()));

        while let Some((sender, signal, receiver)) = queue.pop_front() {
            match signal {
                Signal::Low => low_counter += 1,
                Signal::High => high_counter += 1,
            }

            if let Some(connection) = connections.get_mut(&receiver) {
                process_signal(&sender, &receiver, signal, connection, &mut queue);
            }
        }
    }

    low_counter * high_counter
}

fn solve_part2(data: &str) -> u64 {
    let mut connections = parse_input(data);
    let mut queue: VecDeque<(String, Signal, String)> = VecDeque::new();
    let rx_inputs: Vec<String> = find_inputs(&connections, "rx")
        .iter()
        .flat_map(|l| find_inputs(&connections, l))
        .collect();
    let mut rx_input_cycles: HashMap<String, u64> = HashMap::new();

    for button_press in 1.. {
        queue.push_back(("button".into(), Signal::Low, "broadcaster".into()));

        while let Some((sender, signal, receiver)) = queue.pop_front() {
            if signal == Signal::High
                && rx_inputs.contains(&sender)
                && !rx_input_cycles.contains_key(&sender)
            {
                rx_input_cycles.insert(sender.clone(), button_press);
            }

            if let Some(connection) = connections.get_mut(&receiver) {
                process_signal(&sender, &receiver, signal, connection, &mut queue);
            }
        }

        if rx_input_cycles.len() == rx_inputs.len() {
            break;
        }
    }

    let numbers: Vec<u64> = rx_input_cycles.values().copied().collect();
    find_lcm(&numbers)
}

fn main() {
    let data = fs::read_to_string("day20.txt").expect("Can't read input file");

    println!("Day 20 Part 1: {}", solve_part1(&data));
    println!("Day 20 Part 2: {}", solve_part2(&data));
}
