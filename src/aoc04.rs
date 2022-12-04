use std::fs::File;
use std::io::{BufRead, BufReader};
use parse_int::parse;
use min_max::*;

fn splitter(s: &str, c: char) -> Vec<&str> {
  let parts = s.split(c).collect::<Vec<&str>>();
  if parts.len() != 2 {
    panic!("cannot split {} using {}", s, c);
  }
  parts
}

struct Elve {
  min: u32,
  max: u32,
}

impl Elve {
    fn new(s: &str) -> Elve {
      let parts = splitter(s, '-');
      let start = parse::<u32>(&parts.get(0).unwrap()).unwrap();
      let end = parse::<u32>(&parts.get(1).unwrap()).unwrap();
      Elve {
        min: min!(start,end),
        max: max!(start, end),
      }
    }

    fn fully_contains(&self, other: &Elve) -> bool {
       self.min >= other.min && self.max <= other.max
    }

    fn overlaps(&self, other: &Elve) -> bool {
      let other_first = self.min <= other.max && self.max >= other.min;
      let this_first = other.min <= self.max && other.max >= self.min;
      other_first || this_first
    }
}


struct ElvePair {
  first: Elve,
  last: Elve,
}

impl ElvePair {
    fn new(s: &str) -> ElvePair {
      let parts = splitter(s, ',');
      ElvePair { first: Elve::new(parts.get(0).unwrap()), last: Elve::new(parts.get(1).unwrap()) }
    }
}


fn aoc04_1 (elve_pairs: &Vec<ElvePair>) {
  println!("solving AOC day 4 part 1");
  let mut score: usize = 0;

  for elve_pair in elve_pairs.iter() {
    if elve_pair.first.fully_contains(&elve_pair.last) || elve_pair.last.fully_contains(&elve_pair.first) {
      score+=1;
    }
  }

  println!("score: {}", score);

}

fn aoc04_2 (elve_pairs: &Vec<ElvePair>)  {
  println!("solving AOC day 4 part 2");
  let mut score: usize = 0;

  for elve_pair in elve_pairs.iter() {
    if elve_pair.first.overlaps(&elve_pair.last) {
      score+=1;
    }
  }

  println!("score: {}", score);

}


pub fn aoc04() {
    let reader = BufReader::new(File::open("input-04").unwrap());

    let mut elve_pairs: Vec<ElvePair> = vec![];

    for (_index, line) in reader.lines().enumerate() {
        elve_pairs.push(ElvePair::new(&line.unwrap()));
    }

  aoc04_1(&elve_pairs);
  aoc04_2(&elve_pairs);
}