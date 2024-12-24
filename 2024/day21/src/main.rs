use std::collections::HashMap;

#[derive(PartialEq, Clone)]
enum MoveOrder {
    HorizontalVertical,
    VerticalHorizontal,
}

#[derive(Clone)]
struct StepGen {
    start: (isize, isize),
    end: (isize, isize),
    completed: bool,
    order: MoveOrder,
}

impl StepGen {
    fn new(start: (isize, isize), end: (isize, isize), order: MoveOrder) -> Self {
        Self {
            start,
            end,
            completed: false,
            order,
        }
    }

    fn make_horizontal_move(&mut self) -> char {
        let mv = match self.end.0 > self.start.0 {
            true => '>',
            false => '<',
        };

        self.start.0 += (self.end.0 - self.start.0).signum();

        mv
    }

    fn make_vertical_move(&mut self) -> char {
        let mv = match self.end.1 > self.start.1 {
            true => 'v',
            false => '^',
        };

        self.start.1 += (self.end.1 - self.start.1).signum();

        mv
    }
}

impl Iterator for StepGen {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.completed {
            return None;
        }

        let horizontal_complete = self.end.0 == self.start.0;
        let vertical_complete = self.end.1 == self.start.1;

        match (horizontal_complete, vertical_complete, &self.order) {
            (false, true, _) | (false, false, MoveOrder::HorizontalVertical) => {
                Some(self.make_horizontal_move())
            }
            (true, false, _) | (false, false, MoveOrder::VerticalHorizontal) => {
                Some(self.make_vertical_move())
            }
            (true, true, _) => {
                self.completed = true;
                Some('A')
            }
        }
    }
}

struct MoveGen {
    start: (isize, isize),
    end: (isize, isize),
    orders: Vec<MoveOrder>,
}

impl MoveGen {
    fn new(start: (isize, isize), end: (isize, isize), key_pad: &KeyPad) -> Self {
        let mut orders = Vec::with_capacity(2);

        if start.0 == end.0 || start.1 == end.1 {
            orders.push(MoveOrder::HorizontalVertical);
        } else {
            if key_pad.is_valid_position(&(start.0, end.1)) {
                orders.push(MoveOrder::VerticalHorizontal);
            }
            if key_pad.is_valid_position(&(end.0, start.1)) {
                orders.push(MoveOrder::HorizontalVertical);
            }
        }

        assert!(!orders.is_empty());

        Self { start, end, orders }
    }
}

impl Iterator for MoveGen {
    type Item = StepGen;

    fn next(&mut self) -> Option<Self::Item> {
        let order = self.orders.pop()?;

        Some(StepGen::new(self.start, self.end, order))
    }
}

struct MoveSeqGen<'a> {
    key_pad: &'a KeyPad,
    move_gens: Vec<(char, MoveGen)>,
    step_gens: Vec<StepGen>,
    completed: bool,
}

impl<'a> MoveSeqGen<'a> {
    fn new(key_pad: &'a KeyPad, code: &[char]) -> Self {
        let mut start = key_pad.pointer;
        let mut move_gens = Vec::new();

        for c in code {
            let end = key_pad.position(*c);
            move_gens.push((*c, MoveGen::new(start, end, key_pad)));
            start = end;
        }

        let step_gens = move_gens
            .iter_mut()
            .map(|(_, mg)| {
                mg.next()
                    .expect("MoveGens should always return at least one element")
            })
            .collect();

        Self {
            key_pad,
            move_gens,
            step_gens,
            completed: false,
        }
    }
}

impl Iterator for MoveSeqGen<'_> {
    type Item = Vec<char>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.completed {
            return None;
        }

        let step_gens = self.step_gens.clone();
        let mut result = Vec::new();
        for sg in step_gens {
            result.extend(sg);
        }

        let mut last_mgs = Vec::new();

        while let Some((c, mut mg)) = self.move_gens.pop() {
            if let Some(next_sg) = mg.next() {
                let index = self.move_gens.len();
                self.step_gens[index] = next_sg;
                self.move_gens.push((c, mg));
                break;
            }

            last_mgs.push((c, mg));
        }

        if self.move_gens.is_empty() {
            self.completed = true;
        } else {
            while let Some((c, mg)) = last_mgs.pop() {
                let index = self.move_gens.len();
                let mut new_mg = MoveGen::new(mg.start, mg.end, self.key_pad);
                self.step_gens[index] = new_mg
                    .next()
                    .expect("MoveGen should always return at least one element");
                self.move_gens.push((c, new_mg));
            }
        }

        Some(result)
    }
}

struct KeyPad {
    pointer: (isize, isize),
    keys: Vec<((isize, isize), char)>,
}

impl KeyPad {
    fn new_number_pad() -> Self {
        Self {
            pointer: (2, 3),
            keys: vec![
                ((0, 0), '7'),
                ((1, 0), '8'),
                ((2, 0), '9'),
                ((0, 1), '4'),
                ((1, 1), '5'),
                ((2, 1), '6'),
                ((0, 2), '1'),
                ((1, 2), '2'),
                ((2, 2), '3'),
                ((1, 3), '0'),
                ((2, 3), 'A'),
            ],
        }
    }

    fn new_arrow_pad() -> Self {
        Self {
            pointer: (2, 0),
            keys: vec![
                ((1, 0), '^'),
                ((2, 0), 'A'),
                ((0, 1), '<'),
                ((1, 1), 'v'),
                ((2, 1), '>'),
            ],
        }
    }

    fn is_valid_position(&self, pos: &(isize, isize)) -> bool {
        self.keys.iter().any(|(key_pos, _)| key_pos == pos)
    }

    fn position(&self, target: char) -> (isize, isize) {
        self.keys
            .iter()
            .find(|(_, c)| *c == target)
            .expect("Key pad does not have target")
            .0
    }
}

struct Level<'a> {
    key_pad: &'a KeyPad,
    cache: HashMap<Vec<char>, usize>,
    next: Option<Box<Level<'a>>>,
}

impl<'a> Level<'a> {
    fn new(key_pad: &'a KeyPad, depth: usize) -> Self {
        let next = if depth > 1 {
            Some(Box::new(Level::new(key_pad, depth - 1)))
        } else {
            None
        };

        Self {
            key_pad,
            cache: HashMap::new(),
            next,
        }
    }

    fn costs(&mut self, code: &[char]) -> usize {
        let codes = code.split_inclusive(|&c| c == 'A').collect::<Vec<_>>();

        let mut cost = 0;
        for code in codes {
            if let Some(&c) = self.cache.get(code) {
                cost += c;
            } else {
                let msg = MoveSeqGen::new(self.key_pad, code);

                let c = if let Some(next) = self.next.as_mut() {
                    msg.map(|code| next.costs(&code)).min().unwrap()
                } else {
                    msg.map(|code| code.len()).min().unwrap()
                };
                cost += c;

                self.cache.insert(code.to_vec(), c);
            }
        }

        cost
    }
}

fn main() {
    let data = include_str!("../day21.txt");

    let codes: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();

    let num_pad = KeyPad::new_number_pad();
    let dir_pad = KeyPad::new_arrow_pad();

    let part1_result = calculate_result(&codes, &num_pad, &dir_pad, 2);

    println!("Day 21 Part 1: {}", part1_result);

    let part2_result = calculate_result(&codes, &num_pad, &dir_pad, 25);

    println!("Day 21 Part 2: {}", part2_result);
}

fn calculate_result(
    codes: &[Vec<char>],
    num_pad: &KeyPad,
    dir_pad: &KeyPad,
    levels: usize,
) -> usize {
    let mut lvl = Level::new(dir_pad, levels);

    codes
        .iter()
        .map(|code| {
            let code_value = code_to_int(code);
            MoveSeqGen::new(num_pad, code)
                .map(|code_variant| lvl.costs(&code_variant) * code_value)
                .min()
                .unwrap()
        })
        .sum()
}

fn code_to_int(code: &[char]) -> usize {
    code.iter()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<usize>()
        .expect("code should be a number")
}
