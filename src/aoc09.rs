use parse_int::parse;
use std::fs::File;
use std::io::{BufRead, BufReader};
use substring::Substring;

fn sign(n: isize) -> isize {
    if n > 0 {
        return 1;
    }
    if n < 0 {
        return -1;
    }
    return 0;
}

#[derive(Debug, Clone)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new(x: isize, y: isize) -> Position {
        Position { x, y }
    }

    fn update(&mut self, char: &char) {
        match char {
            'U' => self.y += 1,
            'D' => self.y -= 1,
            'R' => self.x += 1,
            'L' => self.x -= 1,
            _ => panic!(),
        }
    }

    fn need_move(&self, other: &Position) -> bool {
        // println!("need move? {:?} {:?}", self, other);
        (self.x).abs_diff(other.x) > 1 || self.y.abs_diff(other.y) > 1
    }

    fn equals(&self, other: &Position) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug, Clone)]
struct Tail {
    pos: Position,
    history: Vec<Position>,
}

impl Tail {
    fn new() -> Tail {
        Tail {
            pos: Position::new(0, 0),
            history: vec![],
        }
    }

    fn follow(&mut self, head: &Position) {
        if !self.pos.need_move(head) {
            return;
        }

        self.history.push(self.pos.clone());

        // vertical movement
        if self.pos.x == head.x {
            self.pos.y += sign(head.y - self.pos.y);
            return;
        }
        // horizontal movement
        if self.pos.y == head.y {
            self.pos.x += sign(head.x - self.pos.x);
            return;
        }

        // diagonal movement
        if self.pos.x.abs_diff(head.x) == 1 {
            self.pos.x = head.x;
            self.pos.y += sign(head.y - self.pos.y);
            return;
        }
        if self.pos.y.abs_diff(head.y) == 1 {
            self.pos.y = head.y;
            self.pos.x += sign(head.x - self.pos.x);
            return;
        }

        self.pos.y += sign(head.y - self.pos.y);
        self.pos.x += sign(head.x - self.pos.x);
    }

    fn n_steps(&self) -> usize {
        let mut h = self.history.clone();
        h.push(self.pos.clone());

        h.sort_by(|a, b| {
            if a.x == b.x {
                return a.y.cmp(&b.y);
            }
            return a.x.cmp(&b.x);
        });
        let mut n_doubles = 0;
        for i in 1..h.len() {
            if h.get(i - 1).unwrap().equals(h.get(i).unwrap()) {
                n_doubles += 1;
            }
        }
        h.len() - n_doubles
    }
}

fn aoc09_1() {
    println!("solving AOC day 9 part 1");
    let reader = BufReader::new(File::open("input-09").unwrap());

    let mut head = Position::new(0, 0);
    let mut tail = Tail::new();

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let direction = line.chars().nth(0).unwrap();
        let mut steps = parse::<usize>(line.substring(2, line.len())).unwrap();
        while steps > 0 {
            head.update(&direction);
            tail.follow(&head);
            steps -= 1;
        }
    }

    println!("n positions of tail: {}", tail.n_steps());
}

fn aoc09_2() {
    println!("solving AOC day 9 part 2");
    let reader = BufReader::new(File::open("input-09").unwrap());

    let mut score: usize = 0;
    let mut head = Position::new(0, 0);
    let mut tails: Vec<Tail> = vec![];
    while tails.len() < 9 {
        tails.push(Tail::new());
    }

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let direction = line.chars().nth(0).unwrap();
        let mut steps = parse::<usize>(line.substring(2, line.len())).unwrap();

        while steps > 0 {
            head.update(&direction);
            for n in 0..tails.len() {
                if n == 0 {
                    tails.get_mut(0).unwrap().follow(&head);
                } else {
                    let h = tails.get(n - 1).unwrap().pos.clone();
                    tails.get_mut(n).unwrap().follow(&h);
                }
            }
            steps -= 1;
        }
    }
    for n in 0..tails.len() {
        println!("tail {} {}", n, tails.get(n).unwrap().n_steps());
    }
}

pub fn aoc09() {
    aoc09_1();
    aoc09_2();
}
