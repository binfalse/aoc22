use parse_int::parse;
use std::fs::File;
use std::io::{self, Write};
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct GrooveCoordinate {
    initial_index: usize,
    value: isize,
}

impl GrooveCoordinate {
    fn new(initial_index: usize, value: isize) -> GrooveCoordinate {
        GrooveCoordinate {
            initial_index,
            value,
        }
    }
}

struct GrooveCoordinates {
    coordinate_list: Vec<GrooveCoordinate>,
}

impl GrooveCoordinates {
    fn mix(&mut self) {
        for i in 0..self.coordinate_list.len() {
            if i % 50 == 0 {
                print!(".");
                io::stdout().flush().unwrap();
            }
            self.mix1(i);
        }
    }

    fn mix1(&mut self, next_index: usize) {
        let element_index = self
            .coordinate_list
            .iter()
            .enumerate()
            .find(|a| a.1.initial_index == next_index)
            .map(|a| a.0)
            .unwrap();
        let element = self.coordinate_list.remove(element_index);
        let list_len = self.coordinate_list.len();
        let target_index = (element_index as isize + element.value) % list_len as isize;
        let target_index = (target_index + list_len as isize) as usize % list_len;
        self.coordinate_list.insert(target_index, element);
    }

    fn find_groove(&self) {
        let element_index = self
            .coordinate_list
            .iter()
            .enumerate()
            .find(|a| a.1.value == 0)
            .map(|a| a.0)
            .unwrap();
        let k1 = (element_index + 1000) % self.coordinate_list.len();
        let k2 = (element_index + 2000) % self.coordinate_list.len();
        let k3 = (element_index + 3000) % self.coordinate_list.len();
        println!(
            "found 0 at {} -- 1k: {} -- 2k: {} -- 3k: {} -- sum: {}",
            element_index,
            self.coordinate_list[k1].value,
            self.coordinate_list[k2].value,
            self.coordinate_list[k3].value,
            self.coordinate_list[k1].value
                + self.coordinate_list[k2].value
                + self.coordinate_list[k3].value
        );
    }
}

fn aoc20_1(input: &Vec<GrooveCoordinate>) {
    println!("solving AOC day 20 part 1");

    let mut list = GrooveCoordinates {
        coordinate_list: input.clone(),
    };
    list.mix();
    println!("");
    list.find_groove();
}

fn aoc20_2(input: &Vec<GrooveCoordinate>) {
    println!("\nsolving AOC day 20 part 2");

    let decryption_key = 811589153;
    let mut list = GrooveCoordinates {
        coordinate_list: input
            .iter()
            .map(|e| {
                let mut e = e.clone();
                e.value = e.value * decryption_key;
                e
            })
            .collect(),
    };

    for i in 0..10 {
        print!("round {} ", i);
        list.mix();
        println!("");
    }

    list.find_groove();
}

pub fn aoc20() {
    let reader = BufReader::new(File::open("input-20").unwrap());

    let input = reader
        .lines()
        .enumerate()
        .map(|(index, line)| GrooveCoordinate::new(index, parse::<isize>(&line.unwrap()).unwrap()))
        .collect();

    aoc20_1(&input);
    aoc20_2(&input);
}
