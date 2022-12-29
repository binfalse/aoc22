use parse_int::parse;
use itertools::Itertools;
use overload::overload;
use rayon::prelude::*;
use regex::Regex;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops;
use std::thread::JoinHandle;
use std::thread;

#[derive(Clone, Hash, Eq, PartialEq)]
struct InventoryState {
    ore_robots: usize,
    clay_robots: usize,
    obsidian_robots: usize,
    geode_robots: usize,
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl InventoryState {

    fn cmp(&self, other: &Self) -> Ordering {
        usize::cmp(&self.geode, &other.geode)
            .then(usize::cmp(&self.geode_robots, &other.geode_robots))
            .then(usize::cmp(&self.obsidian_robots, &other.obsidian_robots))
            .then(usize::cmp(&self.clay_robots, &other.clay_robots))
            .then(usize::cmp(&self.ore_robots, &other.ore_robots))
    }

    fn can_buy(&self, cost: &Cost) -> bool {
        self.ore >= cost.ore && self.clay >= cost.clay && self.obsidian >= cost.obsidian
    }

    fn buy(
        &self,
        ore_robots: usize,
        clay_robots: usize,
        obsidian_robots: usize,
        geode_robots: usize,
        cost: &Cost,
    ) -> InventoryState {
        InventoryState {
            ore_robots: self.ore_robots + ore_robots,
            clay_robots: self.clay_robots + clay_robots,
            obsidian_robots: self.obsidian_robots + obsidian_robots,
            geode_robots: self.geode_robots + geode_robots,
            ore: self.ore + self.ore_robots - cost.ore,
            clay: self.clay + self.clay_robots - cost.clay,
            obsidian: self.obsidian + self.obsidian_robots - cost.obsidian,
            geode: self.geode + self.geode_robots,
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Cost {
    ore: usize,
    clay: usize,
    obsidian: usize,
}
overload!((a: Cost) * (b: Cost) -> Cost { Cost { 
                ore: a.ore * b.ore,
                clay: a.clay * b.clay,
                obsidian: a.obsidian * b.obsidian, } });
overload!((a: Cost) + (b: Cost) -> Cost { Cost { 
                ore: a.ore + b.ore,
                clay: a.clay + b.clay,
                obsidian: a.obsidian + b.obsidian, } });

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct BluePrint {
    id: usize,
    ore_cost: Cost,
    clay_cost: Cost,
    obsidian_cost: Cost,
    geode_cost: Cost,
    empty_cost: Cost,
}

impl BluePrint {
    fn run(&self, max_time: usize) -> usize {
        let mut states = vec![];
        states.push(InventoryState {
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        });
        for _ in 0..max_time {
            let new_states: Vec<InventoryState> = states
                .iter_mut()
                .flat_map(|state| {
                    let mut n = vec![];

                    if state.can_buy(&self.geode_cost) {
                        n.push(state.buy(0, 0, 0, 1, &self.geode_cost));
                    }
                    if state.can_buy(&self.obsidian_cost) {
                        n.push(state.buy(0, 0, 1, 0, &self.obsidian_cost));
                    }
                    if state.can_buy(&self.clay_cost) {
                        n.push(state.buy(0, 1, 0, 0, &self.clay_cost));
                    }
                    if state.can_buy(&self.ore_cost) {
                        n.push(state.buy(1, 0, 0, 0, &self.ore_cost));
                    }

                    n.push(state.buy(0,0,0,0, &self.empty_cost));
                    n
                })
                .collect();

            states = new_states
                .iter()
                .sorted_by(|a, b| InventoryState::cmp(&b, &a))
                .take(123456)
                .map(|a| a.clone())
                .collect();
        }
        let best_solution = states.iter().map(|s| s.geode).max();

        return best_solution.unwrap();
    }
}

fn aoc19_1(blue_prints: &Vec<BluePrint>) {
    println!("solving AOC day 19 part 1");

    let sum: usize = (0..blue_prints.len())
        .into_par_iter()
        .map(|bp| {
            let blue_print = blue_prints.get(bp).unwrap().clone();
            let result = blue_print.run(24);
            println!(
                "found solution for bp {} -> {} = {}",
                bp,
                result,
                result * (bp + 1)
            );
            return result * (bp + 1);
        })
        .sum();

    println!("sum: {}", sum);
}

fn aoc19_2(blue_prints: &Vec<BluePrint>) {
    println!("solving AOC day 19 part 2");

    let mut handles: Vec<JoinHandle<usize>> = vec![];

    for bp in 0..3 {
        let blue_print = blue_prints.get(bp).unwrap().clone();
        handles.push(thread::spawn(move || {
            let result = blue_print.run(32);
            println!(
                "found solution for bp {} -> {} = {}",
                bp,
                result,
                result * (bp + 1)
            );
            return result;
        }));
    }

    let mut product = 1;
    while handles.len() > 0 {
        let handle = handles.remove(0);
        let result = handle.join().unwrap();
        product *= result;
    }
    println!("product: {}", product);
}

pub fn aoc19() {
    let reader = BufReader::new(File::open("input-19").unwrap());
    let re = Regex::new(r"^Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.$")
        .unwrap();

    let mut blue_prints = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        for cap in re.captures_iter(&line) {
            blue_prints.push(BluePrint {
                id: parse::<usize>(&cap[1]).unwrap(),
                ore_cost: Cost {
                    ore: parse::<usize>(&cap[2]).unwrap(),
                    clay: 0,
                    obsidian: 0,
                },
                clay_cost: Cost {
                    ore: parse::<usize>(&cap[3]).unwrap(),
                    clay: 0,
                    obsidian: 0,
                },
                obsidian_cost: Cost {
                    ore: parse::<usize>(&cap[4]).unwrap(),
                    clay: parse::<usize>(&cap[5]).unwrap(),
                    obsidian: 0,
                },
                geode_cost: Cost {
                    ore: parse::<usize>(&cap[6]).unwrap(),
                    clay: 0,
                    obsidian: parse::<usize>(&cap[7]).unwrap(),
                },
                empty_cost: Cost{ ore: 0, clay: 0, obsidian: 0 }
            });
        }
    }

    aoc19_1(&blue_prints);
    aoc19_2(&blue_prints);
}
