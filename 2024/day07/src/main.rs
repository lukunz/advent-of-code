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
        .filter(|eq| find_solutions(&eq.operants, false).contains(&eq.result))
        .map(|eq| eq.result)
        .sum::<u64>();

    let part2_result = equations
        .iter()
        .filter(|eq| find_solutions(&eq.operants, true).contains(&eq.result))
        .map(|eq| eq.result)
        .sum::<u64>();

    println!("Day 7 Part 1: {}", part1_result);
    println!("Day 7 Part 2: {}", part2_result);
}

fn find_solutions(ops: &[u64], with_concat: bool) -> Vec<u64> {
    match ops {
        [tail @ .., head] => {
            if tail.is_empty() {
                return vec![*head];
            }

            let solutions = find_solutions(tail, with_concat);
            let mut mul_soltions = solutions
                .iter()
                .map(|value| *head * value)
                .collect::<Vec<u64>>();
            let mut add_solutions = solutions
                .iter()
                .map(|value| *head + value)
                .collect::<Vec<u64>>();

            mul_soltions.append(&mut add_solutions);

            if with_concat {
                let mut concat_solutions = solutions
                    .iter()
                    .map(|value| {
                        let decimal_places = (0..).take_while(|i| 10u64.pow(*i) <= *head).count();
                        value * 10u64.pow(decimal_places as u32) + head
                    })
                    .collect::<Vec<u64>>();

                mul_soltions.append(&mut concat_solutions);
            }

            mul_soltions
        }
        [] => panic!("No operants found"),
    }
}
