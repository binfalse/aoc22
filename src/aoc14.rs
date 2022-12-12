use min_max::*;
use parse_int::parse;
use std::fs::File;
use std::io::{BufRead, BufReader};
use substring::Substring;

fn aoc14_1() {
    println!("solving AOC day 14 part 1");
    let reader = BufReader::new(File::open("input-14").unwrap());

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
    }

    println!("solution: {}", 0);
}

fn aoc14_2() {
    println!("solving AOC day 14 part 2");
    let reader = BufReader::new(File::open("input-14").unwrap());

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
    }

    println!("solution: {}", 0);
}

pub fn aoc14() {
    aoc14_1();
    aoc14_2();
}
