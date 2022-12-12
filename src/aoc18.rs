use min_max::*;
use parse_int::parse;
use std::fs::File;
use std::io::{BufRead, BufReader};
use substring::Substring;

fn aoc18_1() {
    println!("solving AOC day 18 part 1");
    let reader = BufReader::new(File::open("input-18").unwrap());

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
    }

    println!("solution: {}", 0);
}

fn aoc18_2() {
    println!("solving AOC day 18 part 2");
    let reader = BufReader::new(File::open("input-18").unwrap());

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
    }

    println!("solution: {}", 0);
}

pub fn aoc18() {
    aoc18_1();
    aoc18_2();
}
