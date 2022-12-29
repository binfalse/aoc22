use core::panic;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Jet {
    LEFT,
    RIGHT,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn translate(&mut self, x: isize, y: isize) {
        self.x += x;
        self.y += y;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rock {
    positions: [Point; 5],
}

impl Rock {
    fn spawn_hline(top: isize) -> Rock {
        Rock {
            positions: [
                Point { x: 2, y: top + 4 },
                Point { x: 3, y: top + 4 },
                Point { x: 4, y: top + 4 },
                Point { x: 5, y: top + 4 },
                Point { x: 5, y: top + 4 },
            ],
        }
    }

    fn spawn_plus(top: isize) -> Rock {
        Rock {
            positions: [
                Point { x: 2, y: top + 5 },
                Point { x: 3, y: top + 4 },
                Point { x: 3, y: top + 5 },
                Point { x: 3, y: top + 6 },
                Point { x: 4, y: top + 5 },
            ],
        }
    }

    fn spawn_l(top: isize) -> Rock {
        Rock {
            positions: [
                Point { x: 2, y: top + 4 },
                Point { x: 3, y: top + 4 },
                Point { x: 4, y: top + 4 },
                Point { x: 4, y: top + 5 },
                Point { x: 4, y: top + 6 },
            ],
        }
    }

    fn spawn_block(top: isize) -> Rock {
        Rock {
            positions: [
                Point { x: 2, y: top + 4 },
                Point { x: 3, y: top + 4 },
                Point { x: 2, y: top + 5 },
                Point { x: 3, y: top + 5 },
                Point { x: 3, y: top + 5 },
            ],
        }
    }

    fn spawn_vline(top: isize) -> Rock {
        Rock {
            positions: [
                Point { x: 2, y: top + 4 },
                Point { x: 2, y: top + 5 },
                Point { x: 2, y: top + 6 },
                Point { x: 2, y: top + 7 },
                Point { x: 2, y: top + 7 },
            ],
        }
    }

    fn can_left(&self) -> bool {
        self.positions[0].x > 0
    }

    fn can_right(&self) -> bool {
        self.positions[4].x < 6
    }

    #[allow(dead_code)]
    fn contains(&self, x: isize, y: isize) -> bool {
        for i in 0..5 {
            if self.positions[i].x == x && self.positions[i].y == y {
                return true;
            }
        }
        return false;
    }

    fn translate(&mut self, x: isize, y: isize) {
        for i in 0..5 {
            self.positions[i].translate(x, y);
        }
    }

    fn jet(&mut self, jet: &Jet) {
        match jet {
            Jet::LEFT => {
                if self.can_left() {
                    self.translate(-1, 0);
                }
            }
            Jet::RIGHT => {
                if self.can_right() {
                    self.translate(1, 0);
                }
            }
        }
    }

    fn undo_jet(&mut self, jet: &Jet) {
        match jet {
            Jet::LEFT => {
                self.translate(1, 0);
            }
            Jet::RIGHT => {
                self.translate(-1, 0);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map {
    positions: Vec<Point>,
    top: isize,
    jet_pattern: Vec<Jet>,
    jet_pattern_position: usize,
    n_rocks: usize,
    toppest: [isize; 7],
}

impl Map {
    fn new(jet_pattern: &Vec<Jet>) -> Map {
        let mut positions = vec![];
        for x in 0..7 {
            positions.push(Point { x, y: 0 })
        }

        Map {
            positions,
            top: 0,
            jet_pattern: jet_pattern.clone(),
            jet_pattern_position: 0,
            n_rocks: 0,
            toppest: [0, 0, 0, 0, 0, 0, 0],
        }
    }

    #[allow(dead_code)]
    fn print_rock(&self, rock: &Rock) {
        println!("\n\nvvvvvvv");
        for y in 0..self.top + 7 {
            for x in 0..7 {
                if rock.contains(x, y) {
                    print!("@");
                } else if self.collides_coord(x, y) {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!("\n\nvvvvvvv");
        for y in 0..self.top + 7 {
            for x in 0..7 {
                if self.collides_coord(x, y) {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }

    #[allow(dead_code)]
    fn collides_coord(&self, x: isize, y: isize) -> bool {
        self.positions.iter().any(|p| p.x == x && p.y == y)
    }

    fn collides(&self, position: &Point) -> bool {
        self.positions.iter().any(|p| p.eq(position))
    }

    fn collides_rock(&self, rock: &Rock) -> bool {
        for i in 0..5 {
            if self.collides(&rock.positions[i]) {
                return true;
            }
        }
        return false;
    }

    fn spawn(&mut self) {
        let mut rock = match self.n_rocks % 5 {
            0 => Rock::spawn_hline(self.top),
            1 => Rock::spawn_plus(self.top),
            2 => Rock::spawn_l(self.top),
            3 => Rock::spawn_vline(self.top),
            4 => Rock::spawn_block(self.top),
            x => panic!("which rock!? {} {}", x, self.n_rocks),
        };

        let mut round = 0;
        while round < 1000 {
            round += 1;
            let jet = self
                .jet_pattern
                .get(self.jet_pattern_position % self.jet_pattern.len())
                .unwrap();
            rock.jet(jet);
            if self.collides_rock(&rock) {
                rock.undo_jet(jet);
            }
            self.jet_pattern_position += 1;

            rock.translate(0, -1);
            if self.collides_rock(&rock) {
                self.freeze_rock(&rock);
                // self.print();
                return;
            }
        }
        panic!("didn't come down after {} rounds", round);
    }

    fn freeze_rock(&mut self, rock: &Rock) {
        for position in rock.positions.iter() {
            let y = position.y + 1;
            self.positions.push(Point { x: position.x, y });
            if self.top < y {
                self.top = y;
            }
            if self.toppest[position.x as usize] < y {
                self.toppest[position.x as usize] = y;
            }
        }
        self.n_rocks += 1;
        if self.positions.len() > 200 {
            let mut min = self.top;
            for t in self.toppest {
                if t < min {
                    min = t;
                }
            }
            let mut new_positions = vec![];
            for p in self.positions.iter() {
                if p.y > min - 20 {
                    new_positions.push(p.clone());
                }
            }
            self.positions = new_positions;
        }
    }

    fn get_top_fingerprint(&self) -> isize {
        let mut fp = 0;
        for x in 0..7 {
            fp = 10_isize.pow(x) * (self.top as isize - self.toppest[x as usize]);
        }
        return fp;
    }
}

fn aoc17_1(jet_pattern: &Vec<Jet>) {
    println!("\n\nsolving AOC day 17 part 1");

    let mut map = Map::new(&jet_pattern);

    let current_time = Instant::now();

    while map.n_rocks < 2022 {
        if map.n_rocks % 1000 == 0 {
            println!("rocks {}", map.n_rocks);
        }
        map.spawn();
    }

    println!(
        "height: {}  -- time for {} rocks: {:?}",
        map.top,
        map.n_rocks,
        current_time.elapsed()
    );
}

fn aoc17_2(jet_pattern: &Vec<Jet>) {
    println!("\n\nsolving AOC day 17 part 2");

    let mut map = Map::new(&jet_pattern);

    let mut cache_map = HashMap::new();

    let current_time = Instant::now();
    let nrounds = 1000000000000;
    let mut fast_forward_addition = 0;

    while map.n_rocks < nrounds {
        if map.n_rocks % 1000 == 0 {
            println!("rocks {}", map.n_rocks);
        }

        if fast_forward_addition == 0 {
            let round = map.n_rocks;
            let current_height = map.top as usize;
            let current_rock = map.n_rocks % 5;
            let current_jet = map.jet_pattern_position % map.jet_pattern.len();

            let cache_key = (current_rock, current_jet, map.get_top_fingerprint());
            if cache_map.contains_key(&cache_key) {
                let (prev_round, prev_height) = cache_map.get(&cache_key).unwrap();

                let height_diff = current_height - prev_height;
                let round_diff = round - prev_round;
                let repeat = (nrounds - round) / round_diff;

                fast_forward_addition += repeat * height_diff;
                map.n_rocks += repeat * round_diff;

                println!(
                    "can fast-forward round {} to {} (seen the config in round {} already)",
                    round, map.n_rocks, prev_round
                );
            } else {
                cache_map.insert(cache_key, (round, current_height));
            }
        }
        map.spawn();
    }
    println!(
        "height: {}  -- time for {} rocks: {:?}",
        map.top + fast_forward_addition as isize,
        map.n_rocks,
        current_time.elapsed()
    );
}

pub fn aoc17() {
    let reader = BufReader::new(File::open("input-17").unwrap());
    let mut jet_pattern = vec![];
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        for c in line.chars() {
            if c == '<' {
                jet_pattern.push(Jet::LEFT);
            } else if c == '>' {
                jet_pattern.push(Jet::RIGHT);
            } else {
                panic!("unrecogised jet pattern {} {}", c, line);
            }
        }
    }

    aoc17_1(&jet_pattern);
    aoc17_2(&jet_pattern);
}
