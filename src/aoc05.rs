use std::fs::File;
use std::io::{BufRead, BufReader};

fn aoc05_1 () {
  println!("solving AOC day 5 part 1");
  let reader = BufReader::new(File::open("input-05").unwrap());

  let mut score: usize = 0;

  for (_index, line) in reader.lines().enumerate() {
      let line = line.unwrap();

  }

  println!("score: {}", score);

}

fn aoc05_2 () {
  println!("solving AOC day 5 part 2");
    let reader = BufReader::new(File::open("input-05").unwrap());

    let mut score: usize = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

    }

    println!("score: {}", score);

}


pub fn aoc05() {
  aoc05_1();
  aoc05_2();
}