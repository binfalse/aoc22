use min_max::*;
use std::fmt;
// use parse_int::parse;
// use substring::Substring;
use itertools::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct Pointer {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Elve {
    x: i32,
    y: i32,
}

impl Elve {
    fn new(x: i32, y: i32) -> Elve {
        Elve { x, y }
    }

    fn get_relative_position(&self, bounding_box: &BoundingBox) -> Pointer {
        Pointer {
            x: (self.x - bounding_box.minx),
            y: (self.y - bounding_box.miny),
        }
    }

    fn can(options: Vec<Pointer>, map: &Vec<Vec<bool>>) -> bool {
        options.iter().all(|o| {
            if o.x >= 0
                && o.y >= 0
                && o.y < map.len() as i32
                && o.x < map[o.y as usize].len() as i32
            {
                return !map[o.y as usize][o.x as usize];
            }
            return true;
        })
    }

    fn can_north(p: &Pointer, map: &Vec<Vec<bool>>) -> bool {
        if p.y == 0 {
            return true;
        }

        Elve::can(
            (-1..=1)
                .map(|delta| Pointer {
                    x: p.x + delta,
                    y: p.y - 1,
                })
                .collect(),
            map,
        )
    }

    fn can_south(p: &Pointer, map: &Vec<Vec<bool>>) -> bool {
        if p.y == map.len() as i32 - 1 {
            return true;
        }

        Elve::can(
            (-1..=1)
                .map(|delta| Pointer {
                    x: p.x + delta,
                    y: p.y + 1,
                })
                .collect(),
            map,
        )
    }

    fn can_west(p: &Pointer, map: &Vec<Vec<bool>>) -> bool {
        if p.x == 0 {
            return true;
        }

        Elve::can(
            (-1..=1)
                .map(|delta| Pointer {
                    x: p.x - 1,
                    y: p.y + delta,
                })
                .collect(),
            map,
        )
    }

    fn can_east(p: &Pointer, map: &Vec<Vec<bool>>) -> bool {
        if p.x == map[p.y as usize].len() as i32 - 1 {
            return true;
        }

        Elve::can(
            (-1..=1)
                .map(|delta| Pointer {
                    x: p.x + 1,
                    y: p.y + delta,
                })
                .collect(),
            map,
        )
    }

    fn has_neighbors(p: &Pointer, map: &Vec<Vec<bool>>) -> bool {
        !Elve::can(
            iproduct!((-1..=1), (-1..=1))
                .filter(|(x, y)| !(*x == 0 && *y == 0))
                .map(|(x, y)| Pointer {
                    x: p.x + x,
                    y: p.y + y,
                })
                .collect(),
            map,
        )
    }

    fn turn(&self, round: usize, map: &Vec<Vec<bool>>, bounding_box: &BoundingBox) -> Elve {
        let p = self.get_relative_position(bounding_box);

        if Elve::has_neighbors(&p, map) {
            for trial in 0..4 {
                match (round + trial) % 4 {
                    0 if Elve::can_north(&p, map) => return Elve::new(self.x, self.y - 1),
                    1 if Elve::can_south(&p, map) => return Elve::new(self.x, self.y + 1),
                    2 if Elve::can_west(&p, map) => return Elve::new(self.x - 1, self.y),
                    3 if Elve::can_east(&p, map) => return Elve::new(self.x + 1, self.y),
                    _ => (),
                }
            }
        }
        return Elve::new(self.x, self.y);
    }

    fn same_position(&self, other: &Elve) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn collides(&self, options: &Vec<Elve>) -> bool {
        options.iter().filter(|o| self.same_position(o)).count() > 1
    }
}

impl fmt::Display for Elve {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Debug, Clone)]
struct BoundingBox {
    minx: i32,
    maxx: i32,
    miny: i32,
    maxy: i32,
}

impl BoundingBox {
    fn new(minx: i32, maxx: i32, miny: i32, maxy: i32) -> BoundingBox {
        BoundingBox {
            minx,
            maxx,
            miny,
            maxy,
        }
    }

    fn update(&mut self, x: i32, y: i32) {
        self.minx = min!(self.minx, x);
        self.maxx = max!(self.maxx, x);
        self.miny = min!(self.miny, y);
        self.maxy = max!(self.maxy, y);
    }

    fn get_smaller_copy(&self) -> BoundingBox {
        let n = 2;
        BoundingBox {
            minx: self.minx + n,
            maxx: self.maxx - n,
            miny: self.miny + n,
            maxy: self.maxy - n,
        }
    }

    fn size(&self) -> i32 {
        (1 + self.maxx - self.minx) * (1 + self.maxy - self.miny)
    }
}

#[derive(Debug, Clone)]
struct Map {
    elves: Vec<Elve>,
    bounding_box: BoundingBox,
}

impl Map {
    fn new() -> Map {
        Map {
            elves: vec![],
            bounding_box: BoundingBox::new(1000, 0, 1000, 0),
        }
    }
    fn add_elve(&mut self, row: i32, col: i32) {
        self.elves.push(Elve {
            x: col as i32,
            y: row as i32,
        });
        self.bounding_box.update(col, row)
    }

    fn occupation(&self) -> Vec<Vec<bool>> {
        (self.bounding_box.miny..=self.bounding_box.maxy)
            .map(|y| {
                (self.bounding_box.minx..=self.bounding_box.maxx)
                    .map(|x| self.elves.iter().any(|e| e.x == x && e.y == y))
                    .collect()
            })
            .collect()
    }

    fn turn(&mut self, round: usize) -> bool {
        let occupied = self.occupation();

        let mut moved = false;
        let options: Vec<Elve> = self
            .elves
            .iter()
            .map(|e| {
                let elve_update = e.turn(round, &occupied, &self.bounding_box);
                if !moved && !elve_update.same_position(e) {
                    moved = true
                }
                elve_update
            })
            .collect();

        let mut bounding_box = self.bounding_box.get_smaller_copy();
        let new_elves: Vec<Elve> = options
            .iter()
            .enumerate()
            .map(|(index, elve)| {
                let e = if !elve.collides(&options) {
                    elve.clone()
                } else {
                    self.elves[index].clone()
                };
                bounding_box.update(e.x, e.y);
                return e;
            })
            .collect();

        self.bounding_box = bounding_box;
        self.elves = new_elves;
        return moved;
    }

    fn empty_fields(&self) -> i32 {
        self.bounding_box.size() - self.elves.len() as i32
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in self.bounding_box.miny..=self.bounding_box.maxy {
            for x in self.bounding_box.minx..=self.bounding_box.maxx {
                if self.elves.iter().any(|e| e.x == x && e.y == y) {
                    write!(f, "#")?;
                } else {
                    write!(f, " ")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn aoc23_1(map: &mut Map) {
    println!("solving AOC day 23 part 1");

    for round in 0..10 {
        map.turn(round);
    }

    println!("empty ground: {}", map.empty_fields());
}

fn aoc23_2(map: &mut Map) {
    println!("solving AOC day 23 part 2");

    for round in 0..1000 {
        if !map.turn(round) {
            println!("break as no elve moves after round {}", round + 1);
            break;
        }
    }

    println!("empty ground: {}", map.empty_fields());
}

pub fn aoc23() {
    let reader = BufReader::new(File::open("input-23").unwrap());
    let mut map = Map::new();

    for (row, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                map.add_elve(row as i32, col as i32);
            }
        }
    }

    aoc23_1(&mut map.clone());
    aoc23_2(&mut map.clone());
}
