use core::panic;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn char_to_value(c: char) -> i64 {
    match c {
        '=' => -2,
        '-' => -1,
        '0' => 0,
        '1' => 1,
        '2' => 2,
        _ => panic!("do not understand char {}", c),
    }
}

fn value_to_char(n: i64) -> (char, i64) {
    match n {
        0 => ('0', 0),
        1 => ('1', 1),
        2 => ('2', 2),
        3 => ('=', -2),
        4 => ('-', -1),
        _ => panic!("do not understand value {}", n),
    }
}

fn number_to_snafu(n: i64) -> String {
    let mut s = vec![];
    let mut n = n;
    // let mut snafu = "";
    while n > 0 {
        let m = n % 5;
        let (c, m) = value_to_char(m);
        // println!("-- {} - {} -> {}", n, m, c);
        s.push(c);
        n = (n - m) / 5;
        // snafu += c;
    }
    // println!("===> {:?}", s);
    s.iter().rev().collect()
}

fn aoc25_1() {
    println!("solving AOC day 25 part 1");
    let reader = BufReader::new(File::open("input-25").unwrap());
    let mut score = 0;
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        // score += line.len();
        let mut sum = 0;
        for (i, c) in line.chars().enumerate() {
            sum += 5_i64.pow((line.len() - i - 1) as u32) * char_to_value(c);
        }
        // println!("{} => {}", line, sum);
        score += sum;
    }

    println!("solution: {}", score);
    println!("snafu: {}", number_to_snafu(score));
}

fn aoc25_2() {
    println!("solving AOC day 25 part 2");
    let reader = BufReader::new(File::open("input-25").unwrap());
    let mut score = 0;
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        score += line.len();
    }

    println!("solution: {}", score);
}

pub fn aoc25() {
    aoc25_1();
    aoc25_2();
}
