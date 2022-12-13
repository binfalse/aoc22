use min_max::*;
use parse_int::parse;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Deref;
use substring::Substring;

const RADIX: u32 = 10;

fn strip_stuff(s: &str) -> String {
    s.replace("[", "").replace("]", "").replace(",", "")
}

fn is_num(c: &char) -> bool {
    let digit = c.clone() as isize - 48;
    digit >= 0 && digit <= 9
}

#[derive(Debug, Clone)]
struct Pair {
    part1: String,
    part2: String,
}

impl Pair {
    fn compare(&self, debug: bool) -> bool {
        // let numbers1: Vec<char> = strip_stuff(&self.part1).chars().into_iter().collect();
        // let numbers2: Vec<char> = strip_stuff(&self.part2).chars().into_iter().collect();
        let mut n1 = 0;
        let mut n2 = 0;
        let mut deepnes1 = 0;
        let mut deepnes2 = 0;
        let mut compared_different_deepnes = false;
        let numbers1: Vec<char> = self.part1.chars().into_iter().collect();
        let numbers2: Vec<char> = self.part2.chars().into_iter().collect();

        while n1 < numbers1.len() && n2 < numbers2.len() {
            let char1 = numbers1.get(n1).unwrap();
            let char2 = numbers2.get(n2).unwrap();

            if deepnes1 == deepnes2 {
                compared_different_deepnes = false;
            }
            match (char1, char2) {
                (',', ',') => {}
                ('[', '[') => {
                    if debug {
                        println!("{}-{}: {} vs {} deeper in both", n1, n2, char1, char2);
                    }
                    deepnes1 += 1;
                    deepnes2 += 1;
                }
                (']', ']') => {
                    if debug {
                        println!("{}-{}: {} vs {} up in both", n1, n2, char1, char2);
                    }
                    if deepnes1 != deepnes2 {
                        if debug {
                            println!(
                                "{}-{}: up in booth at different deepnes {}-{}",
                                n1, n2, deepnes1, deepnes2
                            );
                        }
                        return deepnes1 < deepnes2;
                    }
                    deepnes1 -= 1;
                    deepnes2 -= 1;
                }
                ('[', _) => {
                    if debug {
                        println!("{}-{}: {} vs {} deeper in 1", n1, n2, char1, char2);
                    }
                    deepnes1 += 1;
                    n2 -= 1;
                }
                (_, '[') => {
                    if debug {
                        println!("{}-{}: {} vs {} deeper in 2", n1, n2, char1, char2);
                    }
                    deepnes2 += 1;
                    n1 -= 1;
                }
                (_, ']') => {
                    if debug {
                        println!("{}-{}: {} vs {} is {}", n1, n2, char1, char2, false);
                    }
                    return false;
                }
                (']', _) => {
                    if debug {
                        println!("{}-{}: {} vs {} is {}", n1, n2, char1, char2, true);
                    }
                    return true;
                }
                (a, b) => {
                    if compared_different_deepnes && deepnes1 != deepnes2 {
                        if debug {
                            println!(
                                "{}-{}: {} vs {} at different deepnes {}-{} is {}",
                                n1, n2, char1, char2, deepnes1, deepnes2, true
                            );
                        }
                        return deepnes1 < deepnes2;
                    }
                    let mut number1 = a.to_digit(RADIX).unwrap();
                    let mut number2 = b.to_digit(RADIX).unwrap();

                    while is_num(numbers1.get(n1 + 1).unwrap()) {
                        number1 =
                            number1 * 10 + numbers1.get(n1 + 1).unwrap().to_digit(RADIX).unwrap();
                        n1 = n1 + 1;
                    }
                    while is_num(numbers2.get(n2 + 1).unwrap()) {
                        number2 =
                            number2 * 10 + numbers2.get(n2 + 1).unwrap().to_digit(RADIX).unwrap();
                        n2 = n2 + 1;
                    }

                    if number1 < number2 {
                        if debug {
                            println!("{}-{}: {} vs {} is {}", n1, n2, char1, char2, true);
                        }
                        return true;
                    } else if number1 > number2 {
                        if debug {
                            println!("{}-{}: {} vs {} is {}", n1, n2, char1, char2, false);
                        }
                        return false;
                    }
                    if deepnes1 != deepnes2 {
                        compared_different_deepnes = true;
                    }
                }
            }
            n1 += 1;
            n2 += 1;
        }
        if n1 == numbers1.len() && n2 == numbers2.len() {
            panic!("pairs both at end? {:?} {:?}", numbers1, numbers2)
        }

        if n1 == numbers1.len() {
            return true;
        }
        if n2 == numbers2.len() {
            return false;
        }
        panic!("wtf {} {} {:?} {:?}", n1, n2, numbers1, numbers2)
    }
}

impl fmt::Display for Pair {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}\n{}", self.part1, self.part2)
    }
}

#[derive(Debug, Clone)]
struct PairBuilder {
    part1: Option<String>,
    part2: Option<String>,
}

impl PairBuilder {
    fn read(&mut self, line: &str) {
        match self.part1 {
            Some(_) => self.part2 = Some(line.to_string()),
            None => self.part1 = Some(line.to_string()),
        }
    }

    fn clean(&mut self) {
        self.part1 = None;
        self.part2 = None;
    }

    fn build(&mut self) -> Pair {
        let x = self.clone();

        let p = Pair {
            part1: x.part1.unwrap(),
            part2: x.part2.unwrap(),
        };
        self.clean();
        p
    }

    fn ready(&self) -> bool {
        self.part1 != None && self.part2 != None
    }
}

fn aoc13_1() {
    println!("solving AOC day 13 part 1");
    let reader = BufReader::new(File::open("input-13").unwrap());

    let mut pb = PairBuilder {
        part1: None,
        part2: None,
    };

    let mut sum = 0;
    let mut i = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line == "" {
            continue;
        }
        pb.read(&line);
        if pb.ready() {
            i += 1;
            let pair = pb.build();
            let result = pair.compare(i == 19);
            println!("{} {}", i, result);
            if i == 19 {
                println!("\n\n{}", pair);
            }
            // println!("\n\n{} {}\n{}", i, result, pair);
            if result {
                sum += i;
            }
        }
    }
    // 5857 is too high
    println!("solution: {}", sum);
}

fn aoc13_2() {
    println!("solving AOC day 13 part 2");
    let reader = BufReader::new(File::open("input-13").unwrap());

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
    }

    println!("solution: {}", 0);
}

pub fn aoc13() {
    aoc13_1();
    aoc13_2();
}
