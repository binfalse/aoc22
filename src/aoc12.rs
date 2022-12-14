use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct Visit {
    n: usize,
}

#[derive(Debug, Clone)]
struct Map {
    n_cols: usize,
    elevation: Vec<Vec<isize>>,
    visits: Vec<Vec<Visit>>,
}

impl Map {
    fn new() -> Map {
        Map {
            n_cols: 0,
            elevation: vec![],
            visits: vec![],
        }
    }
    fn elevation_value(&self, x: usize, y: usize) -> isize {
        self.elevation.get(y).unwrap().get(x).unwrap().clone()
    }

    fn visibility_value(&self, x: usize, y: usize) -> usize {
        self.visits.get(y).unwrap().get(x).unwrap().n.clone()
    }

    fn try_update(&mut self, x: usize, y: usize, value: usize) -> bool {
        if self.visibility_value(x, y) > value {
            self.visits.get_mut(y).unwrap().get_mut(x).unwrap().n = value;
            true
        } else {
            false
        }
    }

    fn check(&mut self, x: usize, y: usize) {
        if self.visibility_value(x, y) < usize::MAX {
            let current_elevation = self.elevation_value(x, y);
            let next_visit_value = self.visibility_value(x, y) + 1;

            if x > 0 {
                if self.elevation_value(x - 1, y) <= current_elevation + 1 {
                    self.try_update(x - 1, y, next_visit_value);
                }
            }
            if x < self.n_cols - 1 {
                if self.elevation_value(x + 1, y) <= current_elevation + 1 {
                    self.try_update(x + 1, y, next_visit_value);
                }
            }
            if y > 0 {
                if self.elevation_value(x, y - 1) <= current_elevation + 1 {
                    self.try_update(x, y - 1, next_visit_value);
                }
            }
            if y < self.visits.len() - 1 {
                if self.elevation_value(x, y + 1) <= current_elevation + 1 {
                    self.try_update(x, y + 1, next_visit_value);
                }
            }
        }
    }

    fn run(&mut self) -> usize {
        let mut runs = 0;
        while runs < 1000 {
            runs += 1;
            for y in 0..self.elevation.len() {
                for x in 0..self.elevation.get(y).unwrap().len() {
                    if self.visibility_value(x, y) < usize::MAX {
                        if self.elevation_value(x, y) == 26 {
                            return self.visibility_value(x, y);
                        }
                        self.check(x, y);
                    }
                }
            }
        }
        println!("didn't find a solution: {:?}", self.visits);
        panic!();
    }

    fn print(&self) {
        println!();
        println!("ELEVATIONS");
        for y in 0..self.elevation.len() {
            for x in 0..self.elevation.get(y).unwrap().len() {
                print!("{:>4}", self.elevation_value(x, y));
            }
            println!();
        }
        println!();
        println!("VISITS");
        for y in 0..self.elevation.len() {
            for x in 0..self.elevation.get(y).unwrap().len() {
                if self.visibility_value(x, y) < usize::MAX {
                    print!("{:>4}", self.visibility_value(x, y));
                } else {
                    print!("    ");
                }
            }
            println!();
        }
    }
}

fn aoc12_1() {
    println!("solving AOC day 12 part 1");
    let reader = BufReader::new(File::open("input-12").unwrap());

    let mut map = Map::new();

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if map.n_cols < 1 {
            map.n_cols = line.len();
        }
        map.elevation.push(
            line.chars()
                .map(|c| match c {
                    'S' => 0,
                    'E' => 26,
                    _ => c as isize - 97,
                })
                .collect(),
        );
        map.visits.push(
            line.chars()
                .map(|c| match c {
                    'S' => Visit { n: 0 },
                    _ => Visit { n: usize::MAX },
                })
                .collect(),
        );
    }

    println!("solution: {:?}", map.run());
}

fn aoc12_2() {
    println!("solving AOC day 12 part 2");
    let reader = BufReader::new(File::open("input-12").unwrap());

    let mut map = Map::new();

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if map.n_cols < 1 {
            map.n_cols = line.len();
        }
        map.elevation.push(
            line.chars()
                .map(|c| match c {
                    'S' => 0,
                    'E' => 26,
                    _ => c as isize - 97,
                })
                .collect(),
        );
        map.visits.push(
            line.chars()
                .map(|c| match c {
                    'S' => Visit { n: 0 },
                    'a' => Visit { n: 0 },
                    _ => Visit { n: usize::MAX },
                })
                .collect(),
        );
    }

    println!("solution: {:?}", map.run());
    map.print();
}

pub fn aoc12() {
    aoc12_1();
    aoc12_2();
}
