use parse_int::parse;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use substring::Substring;

struct Monkeys {
    map: HashMap<String, String>,
    calc: String,
}

impl Monkeys {
    fn new() -> Monkeys {
        Monkeys {
            map: HashMap::new(),
            calc: "".to_string(),
        }
    }

    fn add(&mut self, line: &str) {
        let re = Regex::new(r"^([a-zA-Z]{4}): (.*)$").unwrap();
        for cap in re.captures_iter(&line) {
            self.map.insert(cap[1].to_string(), cap[2].to_string());
        }
    }

    fn calc(&mut self) -> String {
        let has_calc = Regex::new(r"\((\d+) ([+-/*]) (\d+)\)").unwrap();

        let mut replaced = true;
        while replaced {
            replaced = false;
            let mut new_calc = self.calc.clone();

            for cap in has_calc.captures_iter(&self.calc) {
                let first = parse::<isize>(&cap[1]).unwrap();
                let operator = &cap[2];
                let second = parse::<isize>(&cap[3]).unwrap();

                let replacement = match operator {
                    "+" => first + second,
                    "-" => first - second,
                    "*" => first * second,
                    "/" => first / second,
                    _ => panic!("do not understand operator {}", operator),
                };
                new_calc = new_calc.replace(&cap[0], &replacement.to_string());
                replaced = true;
            }
            self.calc = new_calc;
        }
        self.calc.clone()
    }

    fn resolve(&mut self, include_human: bool) -> String {
        self.calc = self.map.get("root").unwrap().clone();
        let has_vars = Regex::new(r"([a-zA-Z]{4})").unwrap();

        let mut replaced = true;
        while replaced {
            replaced = false;
            let mut new_calc = self.calc.clone();

            for cap in has_vars.captures_iter(&self.calc) {
                let var = &cap[1];
                if include_human || var != "humn" {
                    let replacement = self.map.get(var).unwrap();
                    // println!("   {} => {}", var, replacement);
                    if has_vars.is_match(&replacement) {
                        new_calc = new_calc.replace(var, &format!("({})", replacement));
                    } else {
                        new_calc = new_calc.replace(var, &replacement);
                    }
                    replaced = true;
                }
            }
            // println!("{} => {}", self.calc, new_calc);
            self.calc = new_calc;
        }
        self.calc.clone()
    }

    fn calc_human_value(number: isize, complex_human: &str) -> (isize, String) {
        let mut complex_human = complex_human.to_string();

        if complex_human.starts_with("(") && complex_human.ends_with(")") {
            complex_human = complex_human
                .substring(1, complex_human.len() - 1)
                .to_string();
        }

        if complex_human == "humn" {
            println!("found humn to be {}", number);
            return (0, "".to_string());
        }

        let mut calc_parts =
            Regex::new(r"^(?P<complex>.*humn.*) (?P<operator>[+-/*]) (?P<number>\d+)$").unwrap();
        let mut match_left = true;

        if !calc_parts.is_match(&complex_human) {
            match_left = false;
            calc_parts =
                Regex::new(r"^(?P<number>\d+) (?P<operator>[+-/*]) \((?P<complex>.*humn.*)\)$")
                    .unwrap();
        }

        let cap = calc_parts.captures(&complex_human).unwrap(); //.a

        let cur_number = parse::<isize>(&cap.name("number").unwrap().as_str()).unwrap();
        let complex = cap.name("complex").unwrap().as_str();
        let operator = cap.name("operator").unwrap().as_str();

        let new_number = match operator {
            "+" => number - cur_number,
            "*" => number / cur_number,
            "-" => {
                if match_left {
                    number + cur_number
                } else {
                    cur_number - number
                }
            }
            "/" => {
                if match_left {
                    number * cur_number
                } else {
                    cur_number / number
                }
            }
            _ => panic!("do not understand operator {}", operator),
        };

        return (new_number, complex.to_string());
    }

    fn calc_human(&mut self) {
        let mut calc_parts =
            Regex::new(r"^\((?P<complex>.*)\) (?P<operator>[+-/*]) (?P<number>\d+)$").unwrap();

        if !calc_parts.is_match(&self.calc) {
            calc_parts =
                Regex::new(r"^(?P<number>\d+) (?P<operator>[+-/*]) \((?P<complex>.*)\)$").unwrap();
        }

        let cap = calc_parts.captures(&self.calc).unwrap(); //.and_then(|cap| {

        let mut complex: String = cap.name("complex").unwrap().as_str().to_string();
        let mut number = parse::<isize>(&cap.name("number").unwrap().as_str()).unwrap();

        while complex.len() > 0 {
            let (result, new_complex) = Monkeys::calc_human_value(number, &complex);
            complex = new_complex.to_string();
            number = result;
        }
    }
}

fn aoc21_1(monkeys: &mut Monkeys) {
    println!("solving AOC day 21 part 1");
    monkeys.resolve(true);
    let calc_result = monkeys.calc();

    if calc_result.contains(" + ") {
        let parts = calc_result.split(" + ");
        let mut sum = 0;
        for part in parts {
            sum += parse::<isize>(part).unwrap();
        }
        println!("{} = {}", calc_result, sum);
    } else {
        panic!("unexpeced calc result {}", calc_result);
    }
}

fn aoc21_2(monkeys: &mut Monkeys) {
    println!("\nsolving AOC day 21 part 2");
    monkeys.resolve(false);
    monkeys.calc();
    monkeys.calc_human();
}

pub fn aoc21() {
    let reader = BufReader::new(File::open("input-21").unwrap());

    let mut monkeys = Monkeys::new();

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        monkeys.add(&line);
    }

    aoc21_1(&mut monkeys);
    aoc21_2(&mut monkeys);
}
