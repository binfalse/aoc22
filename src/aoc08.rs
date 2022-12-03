use std::fs::File;
use std::io::{BufRead, BufReader};

fn aoc08_1 () {
  println!("solving AOC day 8 part 1");
  let reader = BufReader::new(File::open("input-08").unwrap());

  let mut score: usize = 0;

  for (_index, line) in reader.lines().enumerate() {
      let line = line.unwrap();

  }

  println!("score: {}", score);

}

fn aoc08_2 () {
  println!("solving AOC day 8 part 2");
    let reader = BufReader::new(File::open("input-08").unwrap());

    let mut score: usize = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

    }

    println!("score: {}", score);

}


pub fn aoc08() {
  aoc08_1();
  aoc08_2();
}