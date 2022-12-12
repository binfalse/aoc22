use min_max::*;
use parse_int::parse;
use std::fs::File;
use std::io::{BufRead, BufReader};
use substring::Substring;

fn aoc16_1() {
    println!("solving AOC day 16 part 1");
    let reader = BufReader::new(File::open("input-16").unwrap());

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
    }

    println!("solution: {}", 0);
}

fn aoc16_2() {
    println!("solving AOC day 16 part 2");
    let reader = BufReader::new(File::open("input-16").unwrap());

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
    }

    println!("solution: {}", 0);
}

pub fn aoc16() {
    aoc16_1();
    aoc16_2();
}
