use std::fs;

enum Operation {
    Noop,
    AddX(i32),
}

struct Cpu {
    register: i32,
    cycle: i32,
    signal_strength: i32,
}

impl Cpu {
    fn execute(&mut self, operation: &Operation) {
        if (self.cycle + 20) % 40 == 0 {
            self.signal_strength += self.cycle * self.register;
        }

        if (self.register..self.register + 3).contains(&((self.cycle - 1) % 40 + 1)) {
            print!("#")
        } else {
            print!(".");
        }

        if self.cycle % 40 == 0 {
            println!();
        }

        match operation {
            Operation::Noop => {}
            Operation::AddX(amount) => {
                self.register += amount;
            }
        }

        self.cycle += 1;
    }
}

fn main() {
    let data = fs::read_to_string("day10/input.txt").expect("Can't read input file");
    let mut operations: Vec<Operation> = Vec::new();
    for line in data.lines() {
        let parts = line.split_once(' ');

        match parts {
            None => {
                operations.push(Operation::Noop);
            }
            Some((_, amount)) => {
                operations.push(Operation::Noop);
                operations.push(Operation::AddX(amount.parse().expect("Error parsing input")));
            },
        }
    }

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
