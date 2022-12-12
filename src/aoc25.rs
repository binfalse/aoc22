use min_max::*;
use parse_int::parse;
use std::fs::File;
use std::io::{BufRead, BufReader};
use substring::Substring;

fn aoc25_1() {
    println!("solving AOC day 25 part 1");
    let reader = BufReader::new(File::open("input-25").unwrap());

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
    }

    println!("solution: {}", 0);
}

fn aoc25_2() {
    println!("solving AOC day 25 part 2");
    let reader = BufReader::new(File::open("input-25").unwrap());

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
    }

    println!("solution: {}", 0);
}

pub fn aoc25() {
    aoc25_1();
    aoc25_2();
}
