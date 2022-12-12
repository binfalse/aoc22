
#[macro_use]
extern crate queues;

use chrono::{Datelike,  Utc};
use std::env;
use parse_int::parse;

mod aoc01;
mod aoc02;
mod aoc03;
mod aoc04;
mod aoc05;
mod aoc06;
mod aoc07;
mod aoc08;
mod aoc09;
mod aoc10;
mod aoc11;

use crate::aoc01::aoc01;
use crate::aoc02::aoc02;
use crate::aoc03::aoc03;
use crate::aoc04::aoc04;
use crate::aoc05::aoc05;
use crate::aoc06::aoc06;
use crate::aoc07::aoc07;
use crate::aoc08::aoc08;
use crate::aoc09::aoc09;
use crate::aoc10::aoc10;
use crate::aoc11::aoc11;

fn main() {
    let args: Vec<String> = env::args().collect();
    let now = Utc::now();
    let mut today = now.day();

    if args.len() > 1 {
        // let d = parse::<u32>(&args[1])?;
        if let Ok(d) = parse::<u32>(&args[1]) {
            today = d;
        }
    }

    println!("solving AOC day {}", today);

    match today {
        1 => aoc01(),
        2 => aoc02(),
        3 => aoc03(),
        4 => aoc04(),
        5 => aoc05(),
        6 => aoc06(),
        7 => aoc07(),
        8 => aoc08(),
        9 => aoc09(),
        10 => aoc10(),
        11 => aoc11(),
_=>panic!(),
    }
}
