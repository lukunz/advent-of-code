enum Op {
    Adv(i32),
    Bxl(i32),
    Bst(i32),
    Jnz(i32),
    Bxc,
    Out(i32),
    Bdv(i32),
    Cdv(i32),
}

impl Op {
    fn new(op: u8, operand: i32) -> Self {
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
    a: i32,
    b: i32,
    c: i32,
    pc: usize,
    out: Vec<i32>,
}

fn main() {
    let data = include_str!("../day17.txt");

    let (register_input, program_input) = data.split_once("\n\n").unwrap();

    let mut register = parse_register(register_input.trim());
    let program = parse_program(program_input.trim());

    execute_program(&program, &mut register);

    let out = register
        .out
        .iter()
        .map(|&a| a.to_string())
        .collect::<Vec<String>>()
        .join(",");

    println!("Day 17 Part 1: {}", out);
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

fn parse_program(input: &str) -> Vec<Op> {
    let (_, program) = input.split_once("Program: ").unwrap();
    let program = program.split(",").collect::<Vec<&str>>();

    assert_eq!(program.len() % 2, 0);

    program
        .as_slice()
        .chunks(2)
        .map(|op| Op::new(op[0].parse().unwrap(), op[1].parse().unwrap()))
        .collect()
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

#[inline]
fn calculate_dv(value: i32, operand: i32, register: &State) -> i32 {
    (value as f64 / 2.0f64.powi(combo_operand(operand as u8, register))).trunc() as i32
}

#[inline]
fn combo_operand(operand: u8, register: &State) -> i32 {
    match operand {
        0..=3 => operand as i32,
        4 => register.a,
        5 => register.b,
        6 => register.c,
        _ => panic!("Unknown combo operand {}", operand),
    }
}
