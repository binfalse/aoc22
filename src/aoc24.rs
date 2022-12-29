use min_max::*;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn equals(&self, x: usize, y: usize) -> bool {
        self.x == x && self.y == y
    }

    fn translated_copy(&self, x: i64, y: i64, maxx: usize, maxy: usize) -> Point {
        let mut x = ((self.x as i64 - 1) + x) % (maxx as i64 - 1);
        let mut y = ((self.y as i64 - 1) + y) % (maxy as i64 - 1);
        while x < 0 {
            x += maxx as i64 - 1;
        }
        while y < 0 {
            y += maxy as i64 - 1;
        }
        Point {
            x: x as usize + 1,
            y: y as usize + 1,
        }
    }

    fn get_options(&self, occ: &Vec<Vec<bool>>, maxx: usize, maxy: usize) -> Vec<Point> {
        let mut options = vec![];
        if self.x > 1 && !occ[self.y][self.x - 1] {
            options.push(Point {
                x: self.x - 1,
                y: self.y,
            });
        }
        if self.x < maxx && !occ[self.y][self.x + 1] {
            options.push(Point {
                x: self.x + 1,
                y: self.y,
            });
        }
        if self.y > 0 && !occ[self.y - 1][self.x] {
            options.push(Point {
                x: self.x,
                y: self.y - 1,
            });
        }
        if self.y < maxy && !occ[self.y + 1][self.x] {
            options.push(Point {
                x: self.x,
                y: self.y + 1,
            });
        }
        if !occ[self.y][self.x] {
            options.push(Point {
                x: self.x,
                y: self.y,
            });
        }
        return options;
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Blizzard {
    position: Point,
    direction: Direction,
}

impl Blizzard {
    fn new(position: &Point, direction: &char) -> Blizzard {
        use Direction::*;
        let d = match direction {
            '^' => UP,
            '>' => RIGHT,
            'v' => DOWN,
            '<' => LEFT,
            _ => panic!("unrecognised direction {}", direction),
        };
        Blizzard {
            position: *position,
            direction: d,
        }
    }

    fn position_at(&self, round: usize, maxx: usize, maxy: usize) -> Point {
        match self.direction {
            Direction::LEFT => self
                .position
                .translated_copy(round as i64 * -1, 0, maxx, maxy),
            Direction::RIGHT => self.position.translated_copy(round as i64, 0, maxx, maxy),
            Direction::UP => self
                .position
                .translated_copy(0, round as i64 * -1, maxx, maxy),
            Direction::DOWN => self.position.translated_copy(0, round as i64, maxx, maxy),
        }
    }
}

struct Map {
    blizzards: Vec<Blizzard>,
    start_position: Option<Point>,
    end_position: Option<Point>,
    occupations: Option<HashMap<usize, Vec<Vec<bool>>>>,
    maxx: usize,
    maxy: usize,
}

impl Map {
    fn new() -> Map {
        Map {
            blizzards: vec![],
            start_position: None,
            end_position: None,
            occupations: None,
            maxx: 0,
            maxy: 0,
        }
    }

    fn add_blizzard(&mut self, position: &Point, direction: &char) {
        self.maxx = max!(self.maxx, position.x);
        self.maxy = max!(self.maxy, position.y);

        if *direction == '.' {
            if self.start_position == None {
                self.start_position = Some(position.clone());
            } else {
                self.end_position = Some(position.clone());
            }
        } else if *direction != '#' {
            self.blizzards.push(Blizzard::new(position, direction));
        }
    }

    fn get_repetition_cycle(&self) -> usize {
        (self.maxx - 1) * (self.maxy - 1)
    }

    fn calc_occupation(&mut self) {
        let start = self.start_position.unwrap();
        let end = self.end_position.unwrap();
        let repetition_cycle = self.get_repetition_cycle();
        let occupations: HashMap<usize, Vec<Vec<bool>>> = (0..repetition_cycle)
            .into_par_iter()
            .map(|iteration| {
                let blizzard_positions: HashSet<_> = self
                    .blizzards
                    .iter()
                    .map(|b| {
                        let tmp = b.position_at(iteration, self.maxx, self.maxy);
                        (tmp.x, tmp.y)
                    })
                    .collect();

                let mut occupation_iteration = vec![];
                for y in 0..=self.maxy {
                    let mut occupation_iteration_row = vec![];
                    for x in 0..=self.maxx {
                        if start.equals(x, y) {
                            occupation_iteration_row.push(false);
                            continue;
                        }
                        if end.equals(x, y) {
                            occupation_iteration_row.push(false);
                            continue;
                        }
                        if x == 0 || y == 0 || x == self.maxx || y == self.maxy {
                            occupation_iteration_row.push(true);
                            continue;
                        }
                        let blizzard = blizzard_positions.contains(&(x, y));
                        occupation_iteration_row.push(blizzard);
                    }
                    occupation_iteration.push(occupation_iteration_row);
                }
                (iteration, occupation_iteration)
            })
            .collect();

        self.occupations = Some(occupations);
    }

    fn run_start_end(&self, round: usize) -> usize {
        let start = self.start_position.unwrap();
        let end = self.end_position.unwrap();
        self.run(&start, &end, round)
    }

    fn run_end_start(&self, round: usize) -> usize {
        let start = self.start_position.unwrap();
        let end = self.end_position.unwrap();
        self.run(&end, &start, round)
    }

    fn run(&self, start: &Point, end: &Point, start_round: usize) -> usize {
        let max_rounds = 1000;
        let repetition_cycle = self.get_repetition_cycle();
        let occupations = self.occupations.as_ref().unwrap();

        let mut options: Vec<Point> = vec![start.clone()];
        let mut seen = HashSet::new();

        for round in start_round..start_round + max_rounds {
            if round % 100 == 0 {
                println!(".. round {} -- n options {}", round, options.len(),);
            }
            let mut new_options: Vec<Point> = vec![];
            let occ = &occupations.get(&(round % occupations.len())).unwrap();

            for current in options.iter() {
                for option in current.get_options(&occ, self.maxx, self.maxy) {
                    if option.equals(end.x, end.y) {
                        return round;
                    }
                    let hashkey = (option.x, option.y, round % repetition_cycle);
                    if !seen.contains(&hashkey) {
                        new_options.push(option);
                        seen.insert(hashkey);
                    }
                }
            }
            options = new_options;
        }
        panic!("didn't manage to find a solution in {} rounds", max_rounds);
    }
}

fn aoc24_1(map: &Map) {
    println!("solving AOC day 24 part 1");
    println!("steps needed: {}", map.run_start_end(0));
}

fn aoc24_2(map: &Map) {
    println!("solving AOC day 24 part 2");

    let mut steps = map.run_start_end(0);
    steps = map.run_end_start(steps);
    steps = map.run_start_end(steps);

    println!("steps needed: {}", steps);
}

pub fn aoc24() {
    let reader = BufReader::new(File::open("input-24").unwrap());
    let mut map = Map::new();
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        for (i, c) in line.chars().enumerate() {
            map.add_blizzard(&Point { x: i, y: index }, &c);
        }
    }
    map.calc_occupation();

    aoc24_1(&map);
    aoc24_2(&map);
}
