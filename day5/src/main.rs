use std::fs;

struct Action {
    number_of_crates: usize,
    source_stack: usize,
    target_stack: usize,
}

fn parse_header(data: &str) -> Vec<Vec<char>> {
    let mut header = data.lines().rev();

    let stack_ids = header.next().expect("Invalid data format");
    let mut stacks: Vec<Vec<char>> = stack_ids.split_whitespace().map(|_| Vec::new()).collect();

    for line in header {
        for (index, crate_tag) in line.match_indices(char::is_alphabetic) {
            let stack_index = (index - 1) / 4;
            stacks[stack_index].push(crate_tag.chars().next().expect("Invalid data format"))
        }
    }

    stacks
}

fn parse_actions(data: &str) -> Vec<Action> {
    data.lines()
        .map(|line| {
            let mut numbers = line
                .split_whitespace()
                .filter_map(|item| item.parse::<usize>().ok());

            Action {
                number_of_crates: numbers.next().expect("Invalid data format"),
                source_stack: numbers.next().expect("Invalid data format") - 1,
                target_stack: numbers.next().expect("Invalid data format") - 1,
            }
        })
        .collect()
}

fn part_one(data: &str) -> String {
    let (header, actions) = data.split_once("\n\n").expect("Invalid data format");

    let mut stacks = parse_header(header);
    let actions = parse_actions(actions);

    for action in actions {
        for _ in 0..action.number_of_crates {
            let crate_tag = stacks[action.source_stack]
                .pop()
                .expect("Can't take from empty stack");
            stacks[action.target_stack].push(crate_tag);
        }
    }

    let mut result = String::new();

    for stack in stacks {
        result.push(match stack.last() {
            Some(c) => *c,
            None => ' ',
        });
    }

    result
}

fn main() {
    let data = fs::read_to_string("day5/input.txt").expect("Can't read input file");

    let result_part_one = part_one(&data);

    println!("{result_part_one}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_small() {
        let data = fs::read_to_string("input-small.txt").expect("Can't read input file");
        let result = part_one(&data);

        assert_eq!(result, "CMZ");
    }
}
