use std::fmt::{self, Display};
use std::fs::File;
use std::io::{BufRead, BufReader};

const RADIX: u32 = 10;

#[derive(Debug, Clone)]
struct Map {
    matrix: Vec<Vec<u32>>,
    rows: usize,
    columns: usize,
}
impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.matrix)
    }
}
impl Map {
    fn value(&self, row: usize, column: usize) -> u32 {
        self.matrix.get(row).unwrap().get(column).unwrap().clone()
    }

    fn on_edge(&self, row: usize, column: usize) -> bool {
        row == 0 || column == 0 || row == self.rows - 1 || column == self.columns - 1
    }

    fn scenic_score(&self, row: usize, column: usize) -> u32 {
        let value = self.value(row, column);

        let mut score: u32 = 1;
        let mut n = 0;

        for r in (0..row).rev() {
            n += 1;
            if self.value(r, column) >= value && r != row {
                break;
            }
        }
        score *= n;
        n = 0;
        for r in row + 1..self.rows {
            n += 1;
            if self.value(r, column) >= value && r != row {
                break;
            }
        }
        score *= n;
        n = 0;
        for c in (0..column).rev() {
            n += 1;
            if self.value(row, c) >= value && c != column {
                break;
            }
        }
        score *= n;
        n = 0;
        for c in column + 1..self.columns {
            n += 1;
            if self.value(row, c) >= value && c != column {
                break;
            }
        }
        score *= n;
        return score;
    }

    fn is_visible(&self, row: usize, column: usize) -> bool {
        let value = self.value(row, column);

        let mut visible = true;

        for r in 0..row {
            if self.value(r, column) >= value && row != r {
                visible = false;
                break;
            }
        }
        if visible {
            return true;
        }
        visible = true;

        for r in row + 1..self.rows {
            if self.value(r, column) >= value && row != r {
                visible = false;
                break;
            }
        }
        if visible {
            return true;
        }
        visible = true;
        for c in 0..column {
            if self.value(row, c) >= value && column != c {
                visible = false;
                break;
            }
        }
        if visible {
            return true;
        }
        visible = true;
        for c in column + 1..self.columns {
            if self.value(row, c) >= value && column != c {
                visible = false;
                break;
            }
        }
        if visible {
            return true;
        }

        return false;
    }
}

fn aoc08_1(map: &Map) {
    println!("solving AOC day 8 part 1");
    let mut score: usize = 0;

    for row in 0..map.rows {
        for column in 0..map.columns {
            if map.on_edge(row, column) {
                score += 1;
            } else if map.is_visible(row, column) {
                score += 1;
            }
        }
    }
    println!("score: {:?}", score);
}

fn aoc08_2(map: &Map) {
    println!("solving AOC day 8 part 2");
    let mut score: u32 = 0;

    for row in 0..map.rows {
        for column in 0..map.columns {
            if map.on_edge(row, column) {
                continue;
            }
            let s = map.scenic_score(row, column);
            if s > score {
                score = s;
            }
        }
    }
    println!("score: {:?}", score);
}

pub fn aoc08() {
    let reader = BufReader::new(File::open("input-08").unwrap());

    let mut matrix: Vec<Vec<u32>> = vec![];
    let mut rows: usize = 0;
    let mut columns: usize = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        matrix.push(line.chars().map(|c| c.to_digit(RADIX).unwrap()).collect());
        rows += 1;
        if columns == 0 {
            columns = line.len();
        }
    }
    let map = Map {
        matrix,
        rows,
        columns,
    };

    aoc08_1(&map);
    aoc08_2(&map);
}
