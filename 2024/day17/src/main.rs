enum Op {
    Adv(u64),
    Bxl(u64),
    Bst(u64),
    Jnz(u64),
    Bxc,
    Out(u64),
    Bdv(u64),
    Cdv(u64),
}

impl Op {
    fn new(op: u8, operand: u64) -> Self {
        match op {
            0 => Self::Adv(operand),
            1 => Self::Bxl(operand),
            2 => Self::Bst(operand),
            3 => Self::Jnz(operand),
            4 => Self::Bxc,
            5 => Self::Out(operand),
            6 => Self::Bdv(operand),
            7 => Self::Cdv(operand),
            _ => panic!("Unknown op {}", op),
        }
    }
}

struct State {
    a: u64,
    b: u64,
    c: u64,
    pc: usize,
    out: Vec<u64>,
}

impl State {
    fn reset(&mut self) {
        self.a = 0;
        self.b = 0;
        self.c = 0;
        self.pc = 0;
        self.out.clear();
    }
}

fn main() {
    let data = include_str!("../day17.txt");

    let (register_input, program_input) = data.split_once("\n\n").unwrap();

    let mut register = parse_register(register_input.trim());
    let (program, raw_program) = parse_program(program_input.trim());

    execute_program(&program, &mut register);

    let part1_result = register
        .out
        .iter()
        .map(|&a| a.to_string())
        .collect::<Vec<String>>()
        .join(",");

    println!("Day 17 Part 1: {}", part1_result);

    let part2_result = find_a(
        &mut register,
        &program,
        &raw_program,
        0,
        raw_program.len() - 1,
    )
    .unwrap();

    println!("Day 17 Part 2: {}", part2_result);
}

fn parse_register(input: &str) -> State {
    let mut register = State {
        a: 0,
        b: 0,
        c: 0,
        pc: 0,
        out: Vec::new(),
    };

    for line in input.lines() {
        let (label, value) = line.split_once(": ").unwrap();
        match label {
            "Register A" => register.a = value.parse().unwrap(),
            "Register B" => register.b = value.parse().unwrap(),
            "Register C" => register.c = value.parse().unwrap(),
            _ => panic!("Unknown register {}", label),
        }
    }

    register
}

fn parse_program(input: &str) -> (Vec<Op>, Vec<u64>) {
    let (_, program) = input.split_once("Program: ").unwrap();
    let program = program.split(",").collect::<Vec<&str>>();

    assert_eq!(program.len() % 2, 0);
    let raw_program = program
        .iter()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u64>>();

    let program = raw_program
        .as_slice()
        .chunks(2)
        .map(|op| Op::new(op[0] as u8, op[1]))
        .collect();

    (program, raw_program)
}

fn execute_program(program: &[Op], register: &mut State) {
    while register.pc < program.len() {
        let mut inc_pc = true;
        match program[register.pc] {
            Op::Adv(operand) => register.a = calculate_dv(register.a, operand, register),
            Op::Bdv(operand) => register.b = calculate_dv(register.a, operand, register),
            Op::Cdv(operand) => register.c = calculate_dv(register.a, operand, register),
            Op::Bxl(operand) => register.b ^= operand,
            Op::Bxc => register.b ^= register.c,
            Op::Bst(operand) => register.b = combo_operand(operand as u8, register) % 8,
            Op::Jnz(operand) => {
                if register.a != 0 {
                    assert_eq!(operand % 2, 0);
                    register.pc = operand as usize / 2;

                    inc_pc = false
                }
            }
            Op::Out(operand) => {
                register
                    .out
                    .push(combo_operand(operand as u8, register) % 8);
            }
        }

        if inc_pc {
            register.pc += 1;
        }
    }
}

fn find_a(
    register: &mut State,
    program: &[Op],
    raw_program: &[u64],
    base: u64,
    pos: usize,
) -> Option<u64> {
    let mut mul = 1;
    for _ in 0..pos {
        mul *= 8;
    }

    for i in 0..8 {
        let a = mul * i + base;

        register.reset();
        register.a = a;
        execute_program(program, register);

        if register.out.len() > pos && raw_program[pos] == register.out[pos] {
            if pos == 0 {
                return Some(a);
            }
            if let Some(a) = find_a(register, program, raw_program, a, pos - 1) {
                return Some(a);
            }
        }
    }

    None
}

#[inline]
fn calculate_dv(value: u64, operand: u64, register: &State) -> u64 {
    (value as f64 / 2.0f64.powi(combo_operand(operand as u8, register) as i32)).trunc() as u64
}

#[inline]
fn combo_operand(operand: u8, register: &State) -> u64 {
    match operand {
        0..=3 => operand as u64,
        4 => register.a,
        5 => register.b,
        6 => register.c,
        _ => panic!("Unknown combo operand {}", operand),
    }
}
