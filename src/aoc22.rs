// use min_max::*;
// use parse_int::parse;
// use substring::Substring;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn aoc22_1() {
    println!("solving AOC day 22 part 1");
    let reader = BufReader::new(File::open("input-22").unwrap());

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
    }

    println!("solution: {}", 0);
}

fn aoc22_2() {
    println!("solving AOC day 22 part 2");
    let reader = BufReader::new(File::open("input-22").unwrap());

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
    }

    println!("solution: {}", 0);
}

pub fn aoc22() {
    aoc22_1();
    aoc22_2();
}
