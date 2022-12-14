extern crate queues;
use chrono::{Datelike, Utc};
use parse_int::parse;
use std::env;
use std::error::Error;

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
mod aoc12;
mod aoc13;
mod aoc14;
mod aoc15;
mod aoc16;
mod aoc17;
mod aoc18;
mod aoc19;
mod aoc20;
mod aoc21;
mod aoc22;
mod aoc23;
mod aoc24;
mod aoc25;

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
use crate::aoc12::aoc12;
use crate::aoc13::aoc13;
use crate::aoc14::aoc14;
use crate::aoc15::aoc15;
use crate::aoc16::aoc16;
use crate::aoc17::aoc17;
use crate::aoc18::aoc18;
use crate::aoc19::aoc19;
use crate::aoc20::aoc20;
use crate::aoc21::aoc21;
use crate::aoc22::aoc22;
use crate::aoc23::aoc23;
use crate::aoc24::aoc24;
use crate::aoc25::aoc25;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let now = Utc::now();
    let mut today = now.day();

    if args.len() > 1 {
        today = parse::<u32>(&args[1])?;
        // if let Ok(d) = parse::<u32>(&args[1]) {
        //     today = d;
        // }
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
        12 => aoc12(),
        13 => aoc13(),
        14 => aoc14(),
        15 => aoc15(),
        16 => aoc16(),
        17 => aoc17(),
        18 => aoc18(),
        19 => aoc19(),
        20 => aoc20(),
        21 => aoc21(),
        22 => aoc22(),
        23 => aoc23(),
        24 => aoc24(),
        25 => aoc25(),

        _ => panic!(),
    };
    Ok(())
}
