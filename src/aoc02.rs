// use parse_int::parse;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn my_score(line: &str) -> i32 {
    if line.contains("X") {
        return 1;
    }
    if line.contains("Y") {
        return 2;
    }
    if line.contains("Z") {
        return 3;
    }
    panic!("ne message");
}

fn outcome(line: &str) -> i32 {
    match line {
        "A X" => return 3,
        "A Y" => return 6,
        "A Z" => return 0,
        "B X" => return 0,
        "B Y" => return 3,
        "B Z" => return 6,
        "C X" => return 6,
        "C Y" => return 0,
        "C Z" => return 3,
        // _ => return 0
        &_ => (),
    };
     panic!("ne message");
}
fn rewrite_line (line: &str) -> &'static str {
    match line {
        "A X" => return "A Z",
        "A Y" => return "A X",
        "A Z" => return "A Y",
        "B X" => return "B X",
        "B Y" => return "B Y",
        "B Z" => return "B Z",
        "C X" => return "C Y",
        "C Y" => return "C Z",
        "C Z" => return "C X",
        _ => panic!("ne message")
    }
}

pub fn aoc02() {
    let reader = BufReader::new(File::open("inputs/input-02").unwrap());
    let mut score: i32 = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
    
        let line2 = rewrite_line(&line);
        score += my_score(&line2) + outcome(&line2);
    }

    println!("score: {}", score);
}
