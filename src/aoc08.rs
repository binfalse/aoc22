use min_max::*;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

const RADIX: u32 = 10;

#[derive(Debug, Clone)]
struct Map {
    matrix: Vec<Vec<usize>>,
    rows: usize,
    columns: usize,
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.matrix)
    }
}

impl Map {
    fn value(&self, row: usize, column: usize) -> usize {
        self.matrix.get(row).unwrap().get(column).unwrap().clone()
    }

    fn on_edge(&self, row: usize, column: usize) -> bool {
        row == 0 || column == 0 || row == self.rows - 1 || column == self.columns - 1
    }

    fn scenic_score_row<I>(&self, range: I, column: usize, value: usize) -> usize
    where
        I: Iterator<Item = usize>,
    {
        range.take_while(|r| self.value(*r, column) < value).count()
    }

    fn scenic_score_col(
        &self,
        range: impl Iterator<Item = usize>,
        row: usize,
        value: usize,
    ) -> usize {
        range.take_while(|c| self.value(row, *c) < value).count()
    }

    fn scenic_score(&self, row: usize, column: usize) -> usize {
        let value = self.value(row, column);
        let mut score: usize = 1;

        // rows before
        score *= min!(
            row,
            self.scenic_score_row((0..row).rev(), column, value) + 1
        );

        // rows after
        score *= min!(
            self.rows - row - 1,
            self.scenic_score_row(row + 1..self.rows, column, value) + 1
        );

        // cols before
        score *= min!(
            column,
            self.scenic_score_col((0..column).rev(), row, value) + 1
        );

        // cols after
        score *= min!(
            self.columns - column - 1,
            self.scenic_score_col(column + 1..self.columns, row, value) + 1
        );

        score
    }

    fn is_visible(&self, row: usize, column: usize) -> bool {
        let value = self.value(row, column);

        if (0..row).all(|r| self.value(r, column) < value) {
            return true;
        }
        if (row + 1..self.rows).all(|r| self.value(r, column) < value) {
            return true;
        }
        if (0..column).all(|c| self.value(row, c) < value) {
            return true;
        }
        if (column + 1..self.columns).all(|c| self.value(row, c) < value) {
            return true;
        }

        false
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
    let mut score: usize = 0;

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

    let mut matrix: Vec<Vec<usize>> = vec![];
    let mut rows: usize = 0;
    let mut columns: usize = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        matrix.push(
            line.chars()
                .map(|c| c.to_digit(RADIX).unwrap().try_into().unwrap())
                .collect(),
        );
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
