use min_max::*;
use parse_int::parse;
use petgraph::algo::dijkstra;
use petgraph::prelude::*;
use petgraph::Graph;
use priority_queue::PriorityQueue;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::time::Instant;

struct Valve {
    id: String,
    pet: NodeIndex,
    rate: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Trial {
    position: NodeIndex,
    time: i32,
    score: i32,
    flow_rate: i32,
    open: Vec<NodeIndex>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct NextOption {
    position: NodeIndex,
    rate: i32,
    wait: i32,
}

impl NextOption {
    fn invalidate(&self) -> NextOption {
        NextOption {
            position: self.position,
            rate: -100,
            wait: -100,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ElephantTrial {
    my_direction: NextOption,
    elephant_direction: NextOption,
    time: i32,
    score: i32,
    flow_rate: i32,
    open: Vec<NodeIndex>,
}

impl ElephantTrial {
    fn iterate(&mut self) {
        self.score += self.flow_rate;
        self.time += 1;
        self.my_direction.wait -= 1;
        self.elephant_direction.wait -= 1;
        let min_wait = min!(self.my_direction.wait, self.elephant_direction.wait);
        if min_wait > 0 {
            self.time += min_wait;
            self.my_direction.wait -= min_wait;
            self.elephant_direction.wait -= min_wait;
            self.score += min_wait * self.flow_rate;
        }
    }

    fn can_become_best(&self, best_score: &i32, max_flow: &i32) -> bool {
        // is there a chance this can become the best solution...?
        self.score + (26 - self.time) * max_flow > *best_score
    }

    fn get_options(
        &self,
        nodes: &HashMap<NodeIndex, &Valve>,
        distances: &HashMap<(NodeIndex, NodeIndex), i32>,
        start: &NodeIndex,
    ) -> Vec<ElephantTrial> {
        let mut options: Vec<ElephantTrial> = vec![];
        if self.my_direction.wait != 0 && self.elephant_direction.wait != 0 {
            options.push(self.clone());
            return options;
        }

        let mut new_open = self.open.clone();
        let mut new_flow_rate = self.flow_rate;

        let my_options: Vec<NextOption> = if self.my_direction.wait == 0 {
            if self.my_direction.position != *start {
                new_open.push(self.my_direction.position);
                new_flow_rate += self.my_direction.rate;
            }
            distances
                .iter()
                .filter(|d| {
                    d.0 .0 == self.my_direction.position
                        && !self.open.contains(&d.0 .1)
                        && self.my_direction.position != d.0 .1
                        && self.elephant_direction.position != d.0 .1
                })
                .map(|dist| NextOption {
                    position: dist.0 .1,
                    rate: nodes.get(&dist.0 .1).unwrap().rate,
                    wait: *dist.1 + 1,
                })
                .collect()
        } else {
            vec![self.my_direction.clone()]
        };

        let elephant_options: Vec<NextOption> = if self.elephant_direction.wait == 0 {
            if self.elephant_direction.position != *start {
                new_open.push(self.elephant_direction.position);
                new_flow_rate += self.elephant_direction.rate;
            }
            distances
                .iter()
                .filter(|d| {
                    d.0 .0 == self.elephant_direction.position
                        && !self.open.contains(&d.0 .1)
                        && self.my_direction.position != d.0 .1
                        && self.elephant_direction.position != d.0 .1
                })
                .map(|dist| NextOption {
                    position: dist.0 .1,
                    rate: nodes.get(&dist.0 .1).unwrap().rate,
                    wait: *dist.1 + 1,
                })
                .collect()
        } else {
            vec![self.elephant_direction.clone()]
        };

        for my_direction in my_options.iter() {
            for elephant_direction in elephant_options.iter() {
                if my_direction.position != elephant_direction.position {
                    options.push(ElephantTrial {
                        my_direction: my_direction.clone(),
                        elephant_direction: elephant_direction.clone(),
                        time: self.time,
                        score: self.score,
                        flow_rate: new_flow_rate,
                        open: new_open.clone(),
                    });
                }
            }
        }

        if options.len() < 1 {
            options.push(self.clone());
        }

        return options;
    }

    fn print(&self, nodes: &HashMap<NodeIndex, &Valve>) {
        println!(
            "found better solution {} = {} @ {}: {:?}",
            self.score,
            self.flow_rate,
            self.time,
            self.open
                .iter()
                .map(|o| nodes.get(o).unwrap().id.clone())
                .collect::<Vec<String>>(),
        );
    }
}

fn print_solution(score: i32, solution: &Trial, nodes: &HashMap<NodeIndex, &Valve>) {
    println!(
        "found better solution {} ({} = {} @ {}) {:?}",
        score,
        solution.score,
        solution.flow_rate,
        solution.time,
        solution
            .open
            .iter()
            .map(|o| nodes.get(o).unwrap().id.clone())
            .collect::<Vec<String>>()
    );
}

fn find_paths(
    nodes: &HashMap<NodeIndex, &Valve>,
    distances: &HashMap<(NodeIndex, NodeIndex), i32>,
    start: &NodeIndex,
    debug: bool,
) {
    let mut queue: PriorityQueue<Trial, i32> = PriorityQueue::new();
    queue.push(
        Trial {
            position: *start,
            score: 0,
            time: 0,
            flow_rate: 0,
            open: vec![],
        },
        0,
    );

    let mut best_solution: Option<Trial> = None;

    while queue.len() > 0 {
        let current_trial = queue.pop().unwrap();

        if current_trial.0.time >= 30 {
            let mut solution = current_trial.0;

            let score = solution.score - solution.flow_rate * (solution.time - 30);

            solution.score = score;
            solution.time = 30;

            if let Some(best) = &best_solution {
                if score > best.score {
                    if debug {
                        print_solution(score, &solution, nodes);
                    }

                    best_solution = Some(solution);
                }
            } else {
                if debug {
                    print_solution(score, &solution, nodes);
                }
                best_solution = Some(solution);
            }
            continue;
        }

        for dist in distances.iter().filter(|d| {
            d.0 .0 == current_trial.0.position && !current_trial.0.open.contains(&d.0 .1)
        }) {
            let rounds = dist.1;
            let mut new_open = current_trial.0.open.clone();
            let target_node = nodes.get(&dist.0 .1).unwrap();
            new_open.push(dist.0 .1);
            let score = current_trial.0.score + (rounds + 1) * current_trial.0.flow_rate;
            let trial = Trial {
                position: dist.0 .1,
                score,
                time: current_trial.0.time + rounds + 1,
                flow_rate: current_trial.0.flow_rate + target_node.rate,
                open: new_open,
            };

            queue.push(trial, score);
        }
        // stay here...

        let rounds = 30 - current_trial.0.time;
        let score = current_trial.0.score + (rounds) * current_trial.0.flow_rate;
        if let Some(best) = &best_solution {
            if score < best.score {
                continue;
            }
        }

        let trial = Trial {
            position: current_trial.0.position,
            score,
            time: current_trial.0.time + rounds,
            flow_rate: current_trial.0.flow_rate,
            open: current_trial.0.open,
        };
        queue.push(trial, score);
    }

    if let Some(best) = &best_solution {
        println!("========= best solution =========");
        print_solution(best.score, &best, nodes);
    }
}

fn find_paths_with_elephant(
    nodes: &HashMap<NodeIndex, &Valve>,
    distances: &HashMap<(NodeIndex, NodeIndex), i32>,
    start: &NodeIndex,
    debug: bool,
) {
    let mut queue: PriorityQueue<ElephantTrial, i32> = PriorityQueue::new();
    queue.push(
        ElephantTrial {
            my_direction: NextOption {
                position: *start,
                rate: 0,
                wait: 1,
            },
            elephant_direction: NextOption {
                position: *start,
                rate: 0,
                wait: 1,
            },
            score: 0,
            time: -1,
            flow_rate: 0,
            open: vec![],
        },
        0,
    );

    let mut best_solution: Option<ElephantTrial> = None;
    let mut best_score: i32 = 0;
    let mut round = 0;
    let max_flow: i32 = nodes.values().map(|n| n.rate).sum();

    while queue.len() > 0 {
        round += 1;
        if debug && round % 100000 == 0 {
            println!(
                "round {} queue len {} score {} ",
                round,
                queue.len(),
                best_score
            );
        }
        let mut current_trial = queue.pop().unwrap();

        current_trial.0.iterate();

        if current_trial.0.time >= 26 {
            let mut solution = current_trial.0;

            let score = solution.score - solution.flow_rate * (solution.time - 26);

            solution.score = score;
            solution.time = 26;

            if let Some(best) = &best_solution {
                if score > best.score {
                    if debug {
                        solution.print(&nodes);
                    }

                    best_solution = Some(solution);
                    best_score = score;
                }
            } else {
                if debug {
                    solution.print(&nodes);
                }
                best_solution = Some(solution);
            }
            continue;
        }

        if !current_trial.0.can_become_best(&best_score, &max_flow) {
            // there is no way this can become the best solution... :/
            continue;
        }

        if current_trial.0.open.len() < nodes.len() {
            let options = current_trial.0.get_options(&nodes, &distances, &start);
            for option in options.iter() {
                queue.push(option.clone(), option.score);
            }
        } else {
            let time_left = 25 - current_trial.0.time;
            let score = current_trial.0.score + time_left * current_trial.0.flow_rate;
            let n = ElephantTrial {
                my_direction: current_trial.0.my_direction.invalidate(),
                elephant_direction: current_trial.0.elephant_direction.invalidate(),
                time: current_trial.0.time + time_left,
                score: score,
                flow_rate: current_trial.0.flow_rate,
                open: current_trial.0.open.clone(),
            };
            queue.push(n, score);
        }
    }

    if let Some(best) = &best_solution {
        println!("\n\n========= best solution =========");
        best.print(&nodes);
    }
}

fn aoc16_1(
    nodes: &HashMap<NodeIndex, &Valve>,
    distances: &HashMap<(NodeIndex, NodeIndex), i32>,
    start: &NodeIndex,
) {
    println!("\n\nsolving AOC day 16 part 1");
    let current = Instant::now();
    find_paths(&nodes, &distances, &start, false);
    println!("Time elapsed in find_paths is: {:?}", current.elapsed());
}

fn aoc16_2(
    nodes: &HashMap<NodeIndex, &Valve>,
    distances: &HashMap<(NodeIndex, NodeIndex), i32>,
    start: &NodeIndex,
) {
    println!("\n\nsolving AOC day 16 part 2");
    let current = Instant::now();
    find_paths_with_elephant(&nodes, &distances, &start, false);
    println!(
        "Time elapsed in find_paths_with_elephant is: {:?}",
        current.elapsed()
    );
}

pub fn aoc16() {
    let reader = BufReader::new(File::open("input-16").unwrap());
    let mut graph: Graph<(), (), Directed> = Graph::new();
    let re = Regex::new(r"^Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? (.*)$")
        .unwrap();
    let mut tmp_edges = vec![];

    let mut nodes_byid = HashMap::new();
    let mut nodes = HashMap::new();
    let mut start: Option<NodeIndex> = None;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        for cap in re.captures_iter(&line) {
            let valve_id = &cap[1];
            let valve_rate = parse::<i32>(&cap[2]).unwrap();
            let nodeindex = graph.add_node(());

            if valve_id == "AA" {
                start = Some(nodeindex);
            }

            let node = Valve {
                id: valve_id.to_string(),
                rate: valve_rate,
                pet: nodeindex,
            };
            nodes.insert(nodeindex, node);
            nodes_byid.insert(valve_id.to_string(), nodeindex);

            for tunnel in cap[3].split(", ") {
                tmp_edges.push((valve_id.to_string(), tunnel.to_string()));
            }
        }
    }

    let start = start.unwrap();

    for edge in tmp_edges.iter() {
        let from = nodes.get(nodes_byid.get(&edge.0).unwrap()).unwrap();
        let to = nodes.get(nodes_byid.get(&edge.1).unwrap()).unwrap();
        graph.add_edge(from.pet, to.pet, ());
    }

    let mut relevant_nodes = HashMap::new();
    let mut distances = HashMap::new();

    for node in nodes.iter() {
        if node.1.rate > 0 || node.0 == &start {
            relevant_nodes.insert(node.0.clone(), node.1);

            let cur_distances = dijkstra(&graph, node.1.pet, None, |_| 1);
            for dist in cur_distances.iter().filter(|d| node.0 != d.0) {
                let target = nodes.get(dist.0).unwrap();
                if target.rate > 0 {
                    distances.insert((node.1.pet, target.pet), dist.1.clone());
                }
            }
        }
    }

    aoc16_1(&relevant_nodes, &distances, &start);
    aoc16_2(&relevant_nodes, &distances, &start);
}
