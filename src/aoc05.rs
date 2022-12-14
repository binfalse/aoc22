use parse_int::parse;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Mutex;

#[derive(Debug, Clone)]
struct Stack {
    chars: Vec<char>,
}

impl Stack {
    fn add_char(&mut self, c: char) {
        self.chars.push(c);
    }
    fn resort(&mut self) {
        self.chars.reverse();
    }
    fn pop_many(&mut self, num: usize) -> Vec<char> {
        let l = self.chars.len();
        self.chars.splice((l - num)..(l), []).collect()
    }
}

struct Action {
    num: usize,
    from: usize,
    to: usize,
}

impl Action {
    fn new(line: &str) -> Action {
        let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
        for cap in re.captures_iter(&line) {
            let mut num = parse::<usize>(&cap[1]).unwrap();
            let from = parse::<usize>(&cap[2]).unwrap();
            let to = parse::<usize>(&cap[3]).unwrap();
            return Action {
                num,
                from: from - 1,
                to: to - 1,
            };
        }
        panic!("failed to parse");
    }
}

fn aoc05_1(stacks: &mut Vec<Mutex<Stack>>, actions: &Vec<Action>) {
    println!("solving AOC day 5 part 1");
    for action in actions.iter() {
        let mut num = action.num;

        let mut stack_from = stacks.get(action.from).unwrap().lock().unwrap();
        let mut stack_to = stacks.get(action.to).unwrap().lock().unwrap();
        while num > 0 {
            stack_to.add_char(stack_from.chars.pop().unwrap());
            num -= 1;
        }
    }
    for s in stacks.iter() {
        print!("{}", s.lock().unwrap().chars.last().unwrap());
    }
    println!();
}

fn aoc05_2(stacks: &mut Vec<Mutex<Stack>>, actions: &Vec<Action>) {
    println!("solving AOC day 5 part 2");
    for action in actions.iter() {
        let mut stack_from = stacks.get(action.from).unwrap().lock().unwrap();
        let mut stack_to = stacks.get(action.to).unwrap().lock().unwrap();
        let mut u: Vec<_> = stack_from.pop_many(action.num);
        stack_to.chars.append(&mut u);
    }
    for s in stacks.iter() {
        print!("{}", s.lock().unwrap().chars.last().unwrap());
    }
    println!();
}

pub fn aoc05() {
    let reader = BufReader::new(File::open("input-05").unwrap());

    let mut stacks: Vec<Mutex<Stack>> = vec![];
    let mut actions: Vec<Action> = vec![];
    let mut init_done = false;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line == "" {
            continue;
        }
        if !init_done {
            let mut stack_number = 0;
            while (stack_number * 4) + 1 < line.len() && !init_done {
                match line.chars().nth((stack_number * 4) + 1).unwrap() {
                    '1' => {
                        init_done = true;
                        for s in stacks.iter() {
                            s.lock().unwrap().resort();
                        }
                        break;
                    }
                    ' ' => (),
                    x => {
                        while stacks.len() < stack_number + 1 {
                            stacks.push(Mutex::new(Stack { chars: vec![] }));
                        }
                        let mut stack = stacks.get(stack_number).unwrap().lock().unwrap();
                        stack.add_char(x);
                    }
                }
                stack_number += 1;
            }
        } else {
            actions.push(Action::new(&line))
        }
    }

    let mut stacks2 = stacks
        .iter()
        .map(|s| Mutex::new(s.lock().unwrap().clone()))
        .collect::<Vec<_>>();

    aoc05_1(&mut stacks, &actions);
    aoc05_2(&mut stacks2, &actions);
}
