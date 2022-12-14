use std::cmp::Ordering;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

const RADIX: u32 = 10;

fn is_num(c: &char) -> bool {
    let digit = c.clone() as isize - 48;
    digit >= 0 && digit <= 9
}

#[derive(Debug, Clone)]
struct Packet {
    line: String,
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.line)
    }
}

impl Packet {
    fn new(line: &str) -> Packet {
        Packet {
            line: line.to_string(),
        }
    }

    fn compare(&self, other: &Packet) -> Ordering {
        let numbers1: Vec<char> = self.line.chars().into_iter().collect();
        let numbers2: Vec<char> = other.line.chars().into_iter().collect();

        let mut n1 = 0;
        let mut n2 = 0;

        let mut depth1 = 0;
        let mut depth2 = 0;

        let mut compared_different_depths = false;

        while n1 < numbers1.len() && n2 < numbers2.len() {
            let char1 = numbers1.get(n1).unwrap();
            let char2 = numbers2.get(n2).unwrap();

            if depth1 == depth2 {
                compared_different_depths = false;
            }
            match (char1, char2) {
                (',', ',') => {}
                ('[', '[') => {
                    depth1 += 1;
                    depth2 += 1;
                }
                (']', ']') => {
                    if depth1 != depth2 {
                        if depth1 < depth2 {
                            return Ordering::Less;
                        } else {
                            return Ordering::Greater;
                        }
                    }
                    depth1 -= 1;
                    depth2 -= 1;
                }
                ('[', _) => {
                    depth1 += 1;
                    n2 -= 1;
                }
                (_, '[') => {
                    depth2 += 1;
                    n1 -= 1;
                }
                (_, ']') => {
                    return Ordering::Greater;
                }
                (']', _) => {
                    return Ordering::Less;
                }
                (a, b) => {
                    if compared_different_depths && depth1 != depth2 {
                        if depth1 < depth2 {
                            return Ordering::Less;
                        } else {
                            return Ordering::Greater;
                        }
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
                        return Ordering::Less;
                    } else if number1 > number2 {
                        return Ordering::Greater;
                    }

                    if depth1 != depth2 {
                        compared_different_depths = true;
                    }
                }
            }
            n1 += 1;
            n2 += 1;
        }

        if n1 == numbers1.len() && n2 == numbers2.len() {
            return Ordering::Equal;
        }
        if n1 == numbers1.len() {
            return Ordering::Less;
        }
        if n2 == numbers2.len() {
            return Ordering::Greater;
        }
        panic!("wtf {} {} {:?} {:?}", n1, n2, numbers1, numbers2)
    }
}

fn aoc13_1() {
    println!("solving AOC day 13 part 1");
    let reader = BufReader::new(File::open("input-13").unwrap());

    let mut packets: Vec<Packet> = vec![];

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line != "" {
            packets.push(Packet::new(&line));
        }
    }

    let mut sum = 0;
    let mut pair = 0;
    while pair + 1 < packets.len() {
        let p1 = packets.get(pair).unwrap();
        let p2 = packets.get(pair + 1).unwrap();
        if p1.compare(p2) == Ordering::Less {
            sum += pair / 2 + 1;
        }
        pair += 2;
    }

    println!("solution: {}", sum);
}

fn aoc13_2() {
    println!("solving AOC day 13 part 2");
    let reader = BufReader::new(File::open("input-13").unwrap());

    let mut packets: Vec<Packet> = vec![];

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line == "" {
            continue;
        }
        packets.push(Packet::new(&line));
    }
    packets.push(Packet::new("[[2]]"));
    packets.push(Packet::new("[[6]]"));

    packets.sort_by(|a, b| a.compare(b));

    let mut solution = 1;
    for (i, el) in packets.iter().enumerate() {
        if el.line == "[[2]]" || el.line == "[[6]]" {
            solution *= i + 1;
        }
    }
    println!("solution: {}", solution);
}

pub fn aoc13() {
    aoc13_1();
    aoc13_2();
}
