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

        let conjunction = connections.get_mut(label).unwrap();
        if let Module::Conjunction(inputs) = &mut conjunction.module {
            for input in input_label {
                inputs.insert(input.clone(), Signal::Low);
            }
        }
    }

    connections
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

fn main() {
    let data = fs::read_to_string("day20.txt").expect("Can't read input file");

    let mut connections = parse_input(&data);
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

    let part1_result = low_counter * high_counter;

    println!("Day 20 Part 1: {}", part1_result);
}
