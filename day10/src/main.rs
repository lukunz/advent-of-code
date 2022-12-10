use std::fs;

enum Operation {
    Noop,
    Addx(i32),
}

struct Cpu {
    register: i32,
    cycle: i32,
    signal_strength: i32,
}

impl Cpu {
    fn cycle(&mut self) {
        self.cycle += 1;

        if (self.cycle + 20) % 40 == 0 {
            self.signal_strength += self.cycle * self.register;
        }
    }

    fn execute(&mut self, operation: &Operation) {
        match operation {
            Operation::Noop => {
                self.cycle();
            },
            Operation::Addx(amount) => {
                self.cycle();
                self.register += amount;
                self.cycle();
            },
        }
    }
}

fn main() {
    let data = fs::read_to_string("day10/input.txt").expect("Can't read input file");

    let operations: Vec<Operation> = data.lines().map(|line| {
        let parts = line.split_once(' ');

        match parts {
            None => Operation::Noop,
            Some((_, amount)) => Operation::Addx(amount.parse().expect("Error parsing input")),
        }
    }).collect();

    let mut cpu = Cpu {
        register: 1,
        cycle: 1,
        signal_strength: 0,
    };

    for operation in operations {
        cpu.execute(&operation);
    }

    println!("{}", cpu.signal_strength);
}
