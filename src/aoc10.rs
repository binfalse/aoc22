use parse_int::parse;
use std::fs::File;
use std::io::{BufRead, BufReader};
use substring::Substring;

fn check_cycle(cycle: isize, n: isize) -> isize {
    match cycle {
        20 | 60 | 100 | 140 | 180 | 220 => n,
        _ => 0,
    }
}

fn print_crt(cycle: isize, n: isize) {
    let c_mod = cycle - 1 % 40;
    if n.abs_diff(c_mod % 40) < 2 {
        print!("#");
    } else {
        print!(" ");
    }
    if cycle % 40 == 0 {
        println!("");
    }
}

pub fn aoc10() {
    println!("solving AOC day 10 part 2");
    let reader = BufReader::new(File::open("input-10").unwrap());

    let mut cycle: isize = 0;
    let mut register: isize = 1;
    let mut sum: isize = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        cycle += 1;
        sum += cycle * check_cycle(cycle, register);
        print_crt(cycle, register);
        if line == "noop" {
            continue;
        }
        cycle += 1;
        sum += cycle * check_cycle(cycle, register);
        print_crt(cycle, register);
        let n = parse::<isize>(line.substring(5, line.len())).unwrap();
        register += n;
    }
    println!("solving AOC day 10 part 1");
    println!("score: {}", sum);
}
