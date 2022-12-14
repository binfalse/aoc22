use parse_int::parse;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{fmt, thread, time};
#[derive(Debug, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(line: &str) -> Position {
        let mut tokens = line.split(",");
        Position {
            x: parse::<usize>(tokens.next().unwrap()).unwrap(),
            y: parse::<usize>(tokens.next().unwrap()).unwrap(),
        }
    }

    fn equal(&self, other: &Position) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn move_towards(&mut self, target: &Position) {
        if self.x != target.x {
            if target.x > self.x {
                self.x += 1;
            } else {
                self.x -= 1;
            }
        }
        if self.y != target.y {
            if target.y > self.y {
                self.y += 1;
            } else {
                self.y -= 1;
            }
        }
    }
}

struct Line {
    points: Vec<Position>,
}

impl Line {
    fn new() -> Line {
        Line { points: vec![] }
    }

    fn get(&self, n: usize) -> &Position {
        self.points.get(n).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum FieldValue {
    AIR,
    ROCK,
    SAND,
    START,
}

impl fmt::Display for FieldValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            FieldValue::AIR => " ",
            FieldValue::START => "+",
            FieldValue::ROCK => "#",
            FieldValue::SAND => "o",
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, Clone)]
struct Field {
    value: FieldValue,
}

#[derive(Debug, Clone)]
struct Map {
    matrix: Vec<Vec<Field>>,
    minx: usize,
    maxx: usize,
    miny: usize,
    maxy: usize,
}

impl Map {
    fn new() -> Map {
        let mut matrix = vec![];
        for y in 0..1000 {
            let mut row = vec![];
            for x in 0..1000 {
                if y == 0 && x == 500 {
                    row.push(Field {
                        value: FieldValue::START,
                    });
                } else {
                    row.push(Field {
                        value: FieldValue::AIR,
                    });
                }
            }
            matrix.push(row);
        }
        Map {
            matrix,
            minx: 500,
            maxx: 500,
            miny: 0,
            maxy: 0,
        }
    }

    fn get(&self, x: usize, y: usize) -> &Field {
        self.matrix.get(y).unwrap().get(x).unwrap()
    }

    fn put(&mut self, x: usize, y: usize, value: FieldValue) {
        self.matrix.get_mut(y).unwrap().get_mut(x).unwrap().value = value
    }

    fn update_bounding_box(&mut self, point: &Position) {
        if self.minx > point.x {
            self.minx = point.x;
        }
        if self.maxx < point.x {
            self.maxx = point.x;
        }
        if self.maxy < point.y {
            self.maxy = point.y;
        }
    }

    fn draw_rocks(&mut self, line: &Line) {
        for point in 1..line.points.len() {
            let mut p = line.get(point - 1).clone();
            let target = line.get(point);

            self.update_bounding_box(&p);
            self.update_bounding_box(&target);

            while !p.equal(&target) {
                self.put(p.x, p.y, FieldValue::ROCK);
                p.move_towards(&target);
            }
            self.put(p.x, p.y, FieldValue::ROCK);
        }
    }

    fn print(&self) {
        for y in self.miny..=self.maxy {
            for x in self.minx..=self.maxx {
                print!("{}", self.get(x, y).value);
            }
            println!();
        }
    }

    fn print_slice(&self, minx: usize, maxx: usize, miny: usize) {
        for y in miny..=self.maxy {
            for x in minx..=maxx {
                print!("{}", self.get(x, y).value);
            }
            println!();
        }
    }

    fn blocked(&self, x: usize, y: usize) -> bool {
        match self.get(x, y).value {
            FieldValue::SAND => true,
            FieldValue::ROCK => true,
            _ => false,
        }
    }

    fn drop(&mut self, x: usize, y: usize) -> usize {
        if x < self.minx || x > self.maxx || y > self.maxy {
            return usize::MAX;
        }
        if self.blocked(x, y) {
            return 0;
        }

        let mut steps = self.drop(x, y + 1);
        if steps == 0 {
            steps = self.drop(x - 1, y + 1);
        }
        if steps == 0 {
            steps = self.drop(x + 1, y + 1);
        }
        if steps == 0 {
            if x == 500 && y == 0 {
                return usize::MAX;
            }
            self.put(x, y, FieldValue::SAND);
            return 1;
        }
        if steps == usize::MAX {
            return steps;
        }
        return steps + 1;
    }
}

fn aoc14_1(debug: bool) {
    println!("solving AOC day 14 part 1");
    let reader = BufReader::new(File::open("input-14").unwrap());

    let mut map = Map::new();

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut rock_line = Line::new();
        for token in line
            .split(" ")
            .filter(|t| t.len() > 2)
            .map(|c| Position::new(c))
        {
            rock_line.points.push(token);
        }
        map.draw_rocks(&rock_line);
    }

    if debug {
        map.print();
    }

    let mut sand_units = 0;
    let mut steps = 0;
    let s = time::Duration::from_millis(50);

    while steps < 1000 {
        steps = map.drop(500, 0);
        sand_units += 1;
        if debug && sand_units % 10 == 0 {
            map.print();
            println!("\n\nSAND: {}", sand_units);
            thread::sleep(s);
        }
    }

    if debug {
        map.print();
    }
    println!("sand: {}", sand_units - 1);
}

fn aoc14_2(debug: bool) {
    println!("solving AOC day 14 part 2");
    let reader = BufReader::new(File::open("input-14").unwrap());

    let mut map = Map::new();

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut rock_line = Line::new();
        for token in line
            .split(" ")
            .filter(|t| t.len() > 2)
            .map(|c| Position::new(c))
        {
            rock_line.points.push(token);
        }
        map.draw_rocks(&rock_line);
    }

    // draw bottom line
    {
        let maxy = map.maxy + 2;
        let mut bottom_line = Line::new();
        bottom_line.points.push(Position { x: 00, y: maxy });
        bottom_line.points.push(Position { x: 999, y: maxy });
        map.draw_rocks(&bottom_line);
    }

    if debug {
        map.print_slice(400, 600, 0);
    }

    let mut sand_units = 0;
    let mut steps = 0;
    let s = time::Duration::from_millis(50);

    while steps < 1000 {
        steps = map.drop(500, 0);
        sand_units += 1;
        if debug && sand_units % 50 == 0 {
            map.print_slice(400, 550, 0);
            println!("\n\nSAND: {}", sand_units);
            thread::sleep(s);
        }
    }

    if debug {
        map.print_slice(400, 550, 0);
    }
    println!("sand: {}", sand_units);
}

pub fn aoc14() {
    aoc14_1(false);
    aoc14_2(false);
}
