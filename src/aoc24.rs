use min_max::*;
use parse_int::parse;
use std::fs::File;
use std::io::{BufRead, BufReader};
use substring::Substring;

fn aoc24_1() {
    println!("solving AOC day 24 part 1");
    let reader = BufReader::new(File::open("input-24").unwrap());

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
    }

    println!("solution: {}", 0);
}

fn aoc24_2() {
    println!("solving AOC day 24 part 2");
    let reader = BufReader::new(File::open("input-24").unwrap());

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
    }

    println!("solution: {}", 0);
}

pub fn aoc24() {
    aoc24_1();
    aoc24_2();
}
