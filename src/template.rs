use min_max::*;
use parse_int::parse;
use std::fs::File;
use std::io::{BufRead, BufReader};
use substring::Substring;

fn aocXXX_1() {
    println!("solving AOC day XXX part 1");
    let reader = BufReader::new(File::open("input-XXX").unwrap());

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
    }

    println!("solution: {}", 0);
}

fn aocXXX_2() {
    println!("solving AOC day XXX part 2");
    let reader = BufReader::new(File::open("input-XXX").unwrap());

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
    }

    println!("solution: {}", 0);
}

pub fn aocXXX() {
    aocXXX_1();
    aocXXX_2();
}
