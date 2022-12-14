// use min_max::*;
// use parse_int::parse;
// use substring::Substring;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn aoc16_1() {
    println!("solving AOC day 16 part 1");
    let reader = BufReader::new(File::open("input-16").unwrap());
    let mut score = 0;
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        score += line.len();
    }

    println!("solution: {}", score);
}

fn aoc16_2() {
    println!("solving AOC day 16 part 2");
    let reader = BufReader::new(File::open("input-16").unwrap());
    let mut score = 0;
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        score += line.len();
    }

    println!("solution: {}", score);
}

pub fn aoc16() {
    aoc16_1();
    aoc16_2();
}
