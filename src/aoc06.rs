use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_unique_slice(lines: &Vec<String>, slice_len: usize) {
    for line in lines {
        for i in (slice_len - 1)..line.len() {
            let mut slice: Vec<char> = line[(i - (slice_len - 1))..=i].chars().map(|c| c).collect();
            slice.sort();
            slice.dedup();
            if slice.len() == slice_len {
                println!("found {:?}, continue at {}", slice, i + 1);
                break;
            }
        }
    }
}

fn aoc06_1(lines: &Vec<String>) {
    println!("solving AOC day 6 part 1");
    find_unique_slice(&lines, 4);
}

fn aoc06_2(lines: &Vec<String>) {
    println!("solving AOC day 6 part 2");
    find_unique_slice(&lines, 14);
}

pub fn aoc06() {
    let reader = BufReader::new(File::open("input-06").unwrap());
    let mut lines: Vec<String> = vec![];

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        lines.push(line.clone());
    }

    aoc06_1(&lines);
    aoc06_2(&lines);
}
