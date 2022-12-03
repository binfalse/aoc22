use std::fs::File;
use std::io::{BufRead, BufReader};

fn aoc06_1 () {
  println!("solving AOC day 6 part 1");
  let reader = BufReader::new(File::open("input-06").unwrap());

  let mut score: usize = 0;

  for (_index, line) in reader.lines().enumerate() {
      let line = line.unwrap();

  }

  println!("score: {}", score);

}

fn aoc06_2 () {
  println!("solving AOC day 6 part 2");
    let reader = BufReader::new(File::open("input-06").unwrap());

    let mut score: usize = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

    }

    println!("score: {}", score);

}


pub fn aoc06() {
  aoc06_1();
  aoc06_2();
}