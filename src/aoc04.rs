use std::fs::File;
use std::io::{BufRead, BufReader};

fn aoc04_1 () {
  println!("solving AOC day 4 part 1");
  let reader = BufReader::new(File::open("input-04").unwrap());
  
  let mut score: usize = 0;

  for (_index, line) in reader.lines().enumerate() {
      let line = line.unwrap();

  }

  println!("score: {}", score);

}

fn aoc04_2 () {
  println!("solving AOC day 4 part 2");
    let reader = BufReader::new(File::open("input-04").unwrap());

    let mut score: usize = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

    }

    println!("score: {}", score);

}


pub fn aoc04() {
  aoc04_1();
  aoc04_2();
}