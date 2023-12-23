use std::collections::HashMap;
use std::fs;

enum Category {
    X,
    M,
    A,
    S,
}

impl Category {
    fn from_str(data: &str) -> Self {
        match data {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => panic!("Unknown category '{}'", data),
        }
    }
}

#[derive(Clone)]
enum Result {
    Accepted,
    Rejected,
    Send(String),
}

impl Result {
    fn from_str(data: &str) -> Self {
        match data {
            "A" => Self::Accepted,
            "R" => Self::Rejected,
            label => Self::Send(label.into()),
        }
    }
}

enum FinalResult {
    Accepted,
    Rejected,
}

impl FinalResult {
    fn from_result(result: &Result) -> Self {
        match result {
            Result::Accepted => Self::Accepted,
            Result::Rejected => Self::Rejected,
            Result::Send(_) => panic!("Result is not final"),
        }
    }
}

enum WorkflowStep {
    Bigger(Category, u64, Result),
    Smaller(Category, u64, Result),
    Just(Result),
}

impl WorkflowStep {
    fn from_str(data: &str) -> Self {
        if let Some((condition_str, result_str)) = data.split_once(':') {
            let result = Result::from_str(result_str);

            if let Some((category_str, value_str)) = condition_str.split_once('<') {
                Self::Smaller(
                    Category::from_str(category_str),
                    value_str.parse().unwrap(),
                    result,
                )
            } else if let Some((category_str, value_str)) = condition_str.split_once('>') {
                Self::Bigger(
                    Category::from_str(category_str),
                    value_str.parse().unwrap(),
                    result,
                )
            } else {
                panic!("Unknown condition '{}'", condition_str)
            }
        } else {
            Self::Just(Result::from_str(data))
        }
    }
}

type Workflow = Vec<WorkflowStep>;
type WorkflowMap = HashMap<String, Workflow>;

#[derive(Debug)]
struct Item {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Item {
    fn set(&mut self, cat: &Category, value: u64) {
        match cat {
            Category::X => self.x = value,
            Category::M => self.m = value,
            Category::A => self.a = value,
            Category::S => self.s = value,
        }
    }

    fn get(&self, cat: &Category) -> u64 {
        match cat {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s,
        }
    }

    fn from_str(data: &str) -> Self {
        let x: &[_] = &['{', '}'];
        let data = data.trim_matches(x);
        let mut new_item = Item {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
        };

        for data_item_string in data.split(',') {
            let (category_str, value_str) = data_item_string.split_once('=').unwrap();
            new_item.set(
                &Category::from_str(category_str),
                value_str.parse().unwrap(),
            );
        }

        new_item
    }

    fn sum(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

fn parse_workflows(data: &str) -> WorkflowMap {
    let mut result: HashMap<String, Workflow> = HashMap::new();

    for line in data.lines() {
        let (label, rest) = line.split_once('{').unwrap();
        let steps_str = rest.trim_end_matches('}');

        let workflow: Workflow = steps_str.split(',').map(WorkflowStep::from_str).collect();

        result.insert(label.into(), workflow);
    }

    result
}

fn parse_items(data: &str) -> Vec<Item> {
    data.lines().map(Item::from_str).collect()
}

fn parse_input(data: &str) -> (WorkflowMap, Vec<Item>) {
    let data = data.trim();
    let (workflow_str, item_str) = data.split_once("\n\n").unwrap();

    (parse_workflows(workflow_str), parse_items(item_str))
}

fn execute_workflow(workflow: &Workflow, item: &Item) -> Result {
    for step in workflow {
        match step {
            WorkflowStep::Bigger(cat, value, result) => {
                if item.get(cat) > *value {
                    return result.clone();
                }
            }
            WorkflowStep::Smaller(cat, value, result) => {
                if item.get(cat) < *value {
                    return result.clone();
                }
            }
            WorkflowStep::Just(result) => {
                return result.clone();
            }
        }
    }

    panic!("Workflow did not finish");
}

fn process_item(workflows: &WorkflowMap, item: &Item) -> FinalResult {
    let mut workflow_label = String::from("in");
    loop {
        let workflow = workflows.get(&workflow_label).expect("Unknown workflow");
        let result = execute_workflow(workflow, item);

        match result {
            Result::Send(new_label) => workflow_label = new_label,
            result => {
                return FinalResult::from_result(&result);
            }
        }
    }
}

fn main() {
    let data = fs::read_to_string("day19.txt").expect("Can't read input file");

    let (workflows, items) = parse_input(&data);

    let mut accepted: Vec<&Item> = Vec::new();

    for item in &items {
        match process_item(&workflows, item) {
            FinalResult::Accepted => accepted.push(item),
            FinalResult::Rejected => {}
        }
    }

    let part1_result: u64 = accepted.iter().map(|item| item.sum()).sum();

    println!("Day 19 Part 1: {}", part1_result);
}
