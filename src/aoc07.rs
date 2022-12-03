use std::fs::File;
use std::io::{BufRead, BufReader};

fn aoc07_1 () {
  println!("solving AOC day 7 part 1");
  let reader = BufReader::new(File::open("input-07").unwrap());

  let mut score: usize = 0;

  for (_index, line) in reader.lines().enumerate() {
      let line = line.unwrap();

  }

  println!("score: {}", score);

}

fn aoc07_2 () {
  println!("solving AOC day 7 part 2");
    let reader = BufReader::new(File::open("input-07").unwrap());

    let mut score: usize = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

    }

    println!("score: {}", score);

}


pub fn aoc07() {
  aoc07_1();
  aoc07_2();
}