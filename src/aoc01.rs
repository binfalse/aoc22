use parse_int::parse;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn aoc01() {
    let reader = BufReader::new(File::open("input-01").unwrap());

    let mut elves: Vec<i32> = Vec::new();
    let mut current_calories = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.

        if line.len() < 1 {
            elves.push(current_calories);
            current_calories = 0;
            continue;
        } else {
            if let Ok(i) = parse::<i32>(&line) {
                current_calories += i;
            }
        }
    }

    elves.sort();

    println!("fattest: {}", elves[elves.len() - 1]);

    let mut sum = 0;
    for elve in elves.iter().rev().take(3) {
        sum += elve;
    }
    println!("sum of fattest 3: {}", sum);

    let x: i32 = elves.iter().rev().take(3).sum();
    println!("{}", x);
    println!("{}", elves.iter().rev().take(3).sum::<i32>());
}
