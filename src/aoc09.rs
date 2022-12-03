use std::fs::File;
use std::io::{BufRead, BufReader};

fn aoc09_1 () {
  println!("solving AOC day 9 part 1");
  let reader = BufReader::new(File::open("input-09").unwrap());

  let mut score: usize = 0;

  for (_index, line) in reader.lines().enumerate() {
      let line = line.unwrap();

  }

  println!("score: {}", score);

}

fn aoc09_2 () {
  println!("solving AOC day 9 part 2");
    let reader = BufReader::new(File::open("input-09").unwrap());

    let mut score: usize = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

    }

    println!("score: {}", score);

}


pub fn aoc09() {
  aoc09_1();
  aoc09_2();
}