use min_max::*;
use parse_int::parse;
use std::fs::File;
use std::io::{BufRead, BufReader};
use substring::Substring;

fn aoc19_1() {
    println!("solving AOC day 19 part 1");
    let reader = BufReader::new(File::open("input-19").unwrap());

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
    }

    println!("solution: {}", 0);
}

fn aoc19_2() {
    println!("solving AOC day 19 part 2");
    let reader = BufReader::new(File::open("input-19").unwrap());

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
    }

    println!("solution: {}", 0);
}

pub fn aoc19() {
    aoc19_1();
    aoc19_2();
}
