use min_max::*;
use parse_int::parse;
use std::fs::File;
use std::io::{BufRead, BufReader};
use substring::Substring;

fn aoc13_1() {
    println!("solving AOC day 13 part 1");
    let reader = BufReader::new(File::open("input-13").unwrap());

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
    }

    println!("solution: {}", 0);
}

fn aoc13_2() {
    println!("solving AOC day 13 part 2");
    let reader = BufReader::new(File::open("input-13").unwrap());

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
    }

    println!("solution: {}", 0);
}

pub fn aoc13() {
    aoc13_1();
    aoc13_2();
}
