use std::fs::File;
use std::io::{BufRead, BufReader};

const LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn aoc03_1 () {
  println!("solving AOC day 3 part 1");
    let reader = BufReader::new(File::open("input-03").unwrap());
    let mut score: usize = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        for c in line.chars() {
          if line.rfind(c).unwrap() >= line.len() / 2 {
            score += LETTERS.find(c).unwrap() + 1;
            break;
          }
        }

    }

    println!("score: {}", score);

}

fn aoc03_2 () {
  println!("solving AOC day 3 part 2");
    let reader = BufReader::new(File::open("input-03").unwrap());
    let mut score: usize = 0;
    let mut collection = vec![];

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        collection.push(line);

        if collection.len() == 3 {

          for c in LETTERS.chars() {
            let mut found = true;
            for line in collection.iter() {
              if line.find(c) == None {
                found = false;
                break;
              }
            }

            if found {
              score += LETTERS.find(c).unwrap() + 1;
break;
            }
          }


          collection.clear();
        }

    }

    println!("score: {}", score);

}


pub fn aoc03() {
  aoc03_1();
  aoc03_2();
}