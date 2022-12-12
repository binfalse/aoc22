use parse_int::parse;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;
use substring::Substring;

use queues::*;

struct Action {
    item: isize,
    target: usize,
}

#[derive(Debug)]
struct Monkey {
    id: usize,
    items: Queue<isize>,
    operation: char,
    operation_target: String,
    test: isize,
    target_true: usize,
    target_false: usize,
    items_seen: usize,
}

impl Monkey {
    fn new(
        id: usize,
        items: String,
        operation: String,
        test: isize,
        true_target: usize,
        false_target: usize,
    ) -> Monkey {
        let operation_parts = operation.split(" ").collect::<Vec<&str>>();

        let mut monkey = Monkey {
            id,
            items: queue![],
            operation: operation_parts.get(1).unwrap().chars().nth(0).unwrap(),
            operation_target: operation_parts.get(2).unwrap().to_string(),
            test: test.clone(),
            target_true: true_target.clone(),
            target_false: false_target.clone(),
            items_seen: 0,
        };
        let item_parts = items.split(", ");
        for item in item_parts {
            monkey
                .items
                .add(parse::<isize>(item).unwrap())
                .map_err(|err| println!("{:?}", err))
                .ok();
        }

        return monkey;
    }

    fn operate(&self, item: isize) -> isize {
        let other = if self.operation_target == "old" {
            item
        } else {
            parse::<isize>(&self.operation_target).unwrap()
        };

        match self.operation {
            '*' => item * other,
            '+' => item + other,
            '-' => item - other,
            '/' => item / other,
            _ => panic!("do not understand operation {} ", self.operation),
        }
    }

    fn test(&self, item: isize) -> usize {
        let div = item / self.test;
        if div * self.test == item {
            self.target_true
        } else {
            self.target_false
        }
    }

    fn turn(&mut self, part: usize, lcm: isize) -> Vec<Action> {
        self.items_seen += self.items.size();
        let mut actions: Vec<Action> = vec![];
        let mut n_items = self.items.size();
        let mut todos: Vec<isize> = vec![];
        while n_items > 0 {
            todos.push(self.items.remove().unwrap());
            n_items -= 1;
        }
        for item in todos {
            let operated = if part == 1 {
                self.operate(item) / 3
            } else {
                self.operate(item) % lcm
            };
            actions.push(Action {
                item: operated,
                target: self.test(operated),
            });
        }

        return actions;
    }
}

fn aoc11_1() {
    println!("solving AOC day 11 part 1");
    let reader = BufReader::new(File::open("input-11").unwrap());

    let mut monkeys: Vec<Monkey> = vec![];
    let mut monkey_id: Option<usize> = None;
    let mut starting_items: String = String::from("");
    let mut operation: String = String::from("");
    let mut test: Option<isize> = None;
    let mut true_target: Option<usize> = None;
    let mut false_target: Option<usize> = None;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line.starts_with("Monkey") {
            monkey_id = Some(parse::<usize>(line.substring(7, line.len() - 1)).unwrap());
            continue;
        }
        if line.starts_with("  Starting items: ") {
            starting_items = line.substring(18, line.len()).to_string();
            continue;
        }
        if line.starts_with("  Operation: new = ") {
            operation = line.substring(19, line.len()).to_string();
            continue;
        }
        if line.starts_with("  Test: divisible by ") {
            test = Some(parse::<isize>(line.substring(21, line.len())).unwrap());
            continue;
        }
        if line.starts_with("    If true: throw to monkey ") {
            true_target = Some(parse::<usize>(line.substring(29, line.len())).unwrap());
            continue;
        }
        if line.starts_with("    If false: throw to monkey ") {
            false_target = Some(parse::<usize>(line.substring(30, line.len())).unwrap());
            continue;
        }
        if line == "" {
            monkeys.push(Monkey::new(
                monkey_id.unwrap(),
                starting_items.to_string(),
                operation.to_string(),
                test.unwrap(),
                true_target.unwrap(),
                false_target.unwrap(),
            ));
            continue;
        }
    }

    for _n in 0..20 {
        for m in 0..monkeys.len() {
            let current_monkey = monkeys.get_mut(m).unwrap();
            let actions = current_monkey.turn(1, 0);
            for action in actions {
                monkeys
                    .get_mut(action.target)
                    .unwrap()
                    .items
                    .add(action.item)
                    .map_err(|err| println!("{:?}", err))
                    .ok();
            }
        }
    }

    monkeys.sort_by(|a, b| b.items_seen.cmp(&a.items_seen));

    for monkey in monkeys.iter() {
        println!("{} {}", monkey.id, monkey.items_seen);
    }

    let max = monkeys.get(0).unwrap().items_seen;
    let max2 = monkeys.get(1).unwrap().items_seen;

    println!("{} * {} = {}", max, max2, max * max2);
}

fn aoc11_2() {
    println!("solving AOC day 11 part 2");
    let reader = BufReader::new(File::open("input-11").unwrap());

    let mut monkeys: Vec<Monkey> = vec![];
    let mut monkey_id: Option<usize> = None;
    let mut starting_items: String = String::from("");
    let mut operation: String = String::from("");
    let mut test: Option<isize> = None;
    let mut true_target: Option<usize> = None;
    let mut false_target: Option<usize> = None;
    let mut lcm = 1;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line.starts_with("Monkey") {
            monkey_id = Some(parse::<usize>(line.substring(7, line.len() - 1)).unwrap());
            continue;
        }
        if line.starts_with("  Starting items: ") {
            starting_items = line.substring(18, line.len()).to_string();
            continue;
        }
        if line.starts_with("  Operation: new = ") {
            operation = line.substring(19, line.len()).to_string();
            continue;
        }
        if line.starts_with("  Test: divisible by ") {
            test = Some(parse::<isize>(line.substring(21, line.len())).unwrap());
            continue;
        }
        if line.starts_with("    If true: throw to monkey ") {
            true_target = Some(parse::<usize>(line.substring(29, line.len())).unwrap());
            continue;
        }
        if line.starts_with("    If false: throw to monkey ") {
            false_target = Some(parse::<usize>(line.substring(30, line.len())).unwrap());
            continue;
        }
        if line == "" {
            monkeys.push(Monkey::new(
                monkey_id.unwrap(),
                starting_items.to_string(),
                operation.to_string(),
                test.unwrap(),
                true_target.unwrap(),
                false_target.unwrap(),
            ));
            lcm = lcm * test.unwrap();
            continue;
        }
    }

    for _n in 0..10000 {
        for m in 0..monkeys.len() {
            let current_monkey = monkeys.get_mut(m).unwrap();
            let actions = current_monkey.turn(2, lcm);
            for action in actions {
                monkeys
                    .get_mut(action.target)
                    .unwrap()
                    .items
                    .add(action.item)
                    .map_err(|err| println!("{:?}", err))
                    .ok();
            }
        }
    }

    monkeys.sort_by(|a, b| b.items_seen.cmp(&a.items_seen));

    for monkey in monkeys.iter() {
        println!("{} {}", monkey.id, monkey.items_seen);
    }

    let max = monkeys.get(0).unwrap().items_seen;
    let max2 = monkeys.get(1).unwrap().items_seen;

    println!("{} * {} = {}", max, max2, max * max2);
}

pub fn aoc11() {
    aoc11_1();
    aoc11_2();
}
