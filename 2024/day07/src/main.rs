#[derive(Debug)]
struct Equation {
    result: u64,
    operants: Vec<u64>,
}

fn main() {
    let data = include_str!("../day07.txt");

    let equations = data
        .lines()
        .map(|line| {
            let (result_str, operants_str) = line.split_once(": ").expect("Line has wrong format");
            let operants = operants_str
                .split(' ')
                .map(|op| op.parse::<u64>().expect("Operants should be integers."))
                .collect();

            Equation {
                result: result_str
                    .parse::<u64>()
                    .expect("Result should be an integer"),
                operants,
            }
        })
        .collect::<Vec<Equation>>();

    let part1_result = equations
        .iter()
        .filter(|eq| is_solvable(eq))
        .map(|eq| eq.result)
        .sum::<u64>();

    println!("Day 7 Part 1: {}", part1_result);
}

fn is_solvable(equation: &Equation) -> bool {
    let solutions = find_solutions(&equation.operants);

    solutions.contains(&equation.result)
}

fn find_solutions(ops: &[u64]) -> Vec<u64> {
    match ops {
        [tail @ .., head] => {
            if tail.is_empty() {
                return vec![*head];
            }

            let solutions = find_solutions(tail);
            let mut mul_soltions = solutions
                .iter()
                .map(|value| *head * value)
                .collect::<Vec<u64>>();
            let mut add_solutions = solutions
                .iter()
                .map(|value| *head + value)
                .collect::<Vec<u64>>();

            mul_soltions.append(&mut add_solutions);

            mul_soltions
        }
        [] => panic!("No operants found"),
    }
}
