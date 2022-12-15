use min_max::*;
use parse_int::parse;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{fmt, thread, time};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn dist(&self, x: isize, y: isize) -> usize {
        self.x.abs_diff(x) + self.y.abs_diff(y)
    }
    fn dist_p(&self, other: &Position) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug, Clone)]
struct Map {
    sensors: Vec<Position>,
    beacons: Vec<Position>,
    minx: isize,
    maxx: isize,
    miny: isize,
    maxy: isize,
}

impl Map {
    fn new() -> Map {
        Map {
            sensors: vec![],
            beacons: vec![],
            minx: isize::MAX,
            maxx: isize::MIN,
            miny: isize::MAX,
            maxy: isize::MIN,
        }
    }

    fn update_bounding_box(&mut self) {
        for s in 0..self.sensors.len() {
            let sensor = self.sensors.get(s).unwrap();
            let beacon = self.beacons.get(s).unwrap();
            let sensor_coverage: isize = sensor.dist_p(beacon) as isize;

            self.minx = min!(
                self.minx,
                sensor.x - sensor_coverage,
                sensor.x + sensor_coverage
            );
            self.maxx = max!(
                self.maxx,
                sensor.x - sensor_coverage,
                sensor.x + sensor_coverage
            );
            self.miny = min!(
                self.miny,
                sensor.y - sensor_coverage,
                sensor.y + sensor_coverage
            );
            self.maxy = max!(
                self.maxy,
                sensor.y - sensor_coverage,
                sensor.y + sensor_coverage
            );
        }
    }

    fn set(&mut self, line: &str) {
        let re = Regex::new(
            r"^Sensor at x=([-\d]+), y=([-\d]+): closest beacon is at x=([-\d]+), y=([-\d]+)$",
        )
        .unwrap();
        for cap in re.captures_iter(line) {
            let sx = parse::<isize>(&cap[1]).unwrap();
            let sy = parse::<isize>(&cap[2]).unwrap();
            let bx = parse::<isize>(&cap[3]).unwrap();
            let by = parse::<isize>(&cap[4]).unwrap();

            self.sensors.push(Position { x: sx, y: sy });
            self.beacons.push(Position { x: bx, y: by });
        }
    }

    fn beacon_absense(&self, line: isize) -> usize {
        let mut covered = 0;
        for x in self.minx..=self.maxx {
            if self.covered(x, line) {
                covered += 1;
            }
        }
        covered
    }

    fn covered(&self, x: isize, y: isize) -> bool {
        for s in 0..self.sensors.len() {
            let sensor = self.sensors.get(s).unwrap();
            let beacon = self.beacons.get(s).unwrap();
            let sensor_coverage = sensor.dist_p(beacon);
            if x == sensor.x && y == sensor.y {
                return false;
            }
            if x == beacon.x && y == beacon.y {
                return false;
            }
            if sensor.dist(x, y) <= sensor_coverage {
                return true;
            }
        }
        false
    }

    fn uncovered(&self, x: isize, y: isize) -> bool {
        for s in 0..self.sensors.len() {
            let sensor = self.sensors.get(s).unwrap();
            let beacon = self.beacons.get(s).unwrap();
            let sensor_coverage = sensor.dist_p(beacon);
            if x == sensor.x && y == sensor.y {
                return false;
            }
            if x == beacon.x && y == beacon.y {
                return false;
            }
            if sensor.dist(x, y) <= sensor_coverage {
                return false;
            }
        }
        true
    }

    fn find_distress_beacon(&self, minx: isize, maxx: isize, miny: isize, maxy: isize) -> isize {
        print!("checking sensors: ");
        for s in (0..self.sensors.len()).rev() {
            print!(" {} ", s);
            let sensor = self.sensors.get(s).unwrap();
            let beacon = self.beacons.get(s).unwrap();
            let sensor_coverage = sensor.dist_p(beacon) as isize;

            for x in sensor.x - sensor_coverage - 1..=sensor.x + sensor_coverage + 1 {
                if x < minx || x > maxx {
                    continue;
                }
                let dist = sensor.dist(x, sensor.y) as isize;
                let y1 = sensor.y - (sensor_coverage - dist) - 1;
                let y2 = sensor.y - (sensor_coverage - dist) - 1;
                if y1 >= miny && y1 <= maxy && self.uncovered(x, y1) {
                    return x * 4000000 + y1;
                }
                if y2 >= miny && y2 <= maxy && self.uncovered(x, y2) {
                    return x * 4000000 + y2;
                }
            }
        }
        panic!("cannot find distress signal");
    }

    fn get(&self, x: isize, y: isize) -> char {
        for s in 0..self.sensors.len() {
            let sensor = self.sensors.get(s).unwrap();
            let beacon = self.beacons.get(s).unwrap();
            if sensor.x == x && sensor.y == y {
                return 'S';
            }
            if beacon.x == x && beacon.y == y {
                return 'B';
            }
        }
        if self.covered(x, y) {
            return '#';
        }
        return '.';
    }

    fn print(&self) {
        print!("    ");
        for x in self.minx..=self.maxx {
            if x % 5 == 0 {
                print!("|");
            } else {
                print!(" ");
            }
        }
        println!();
        for y in self.miny..=self.maxy {
            print!("{:>3} ", y);
            for x in self.minx..=self.maxx {
                print!("{}", self.get(x, y));
            }
            println!();
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {} {}", self.minx, self.maxx, self.miny, self.maxy)
    }
}
fn aoc15_1(map: &Map) {
    println!("solving AOC day 15 part 1");

    // println!("solution: {}", map.beacon_absense(10));
    println!("solution: {}", map.beacon_absense(2000000));
}

fn aoc15_2(map: &Map) {
    println!("solving AOC day 15 part 2");
    let signal = map.find_distress_beacon(0, 4000000, 0, 4000000);
    println!("\ndistress {}", signal);
}

pub fn aoc15() {
    let reader = BufReader::new(File::open("input-15").unwrap());
    let mut map = Map::new();
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        map.set(&line);
    }
    map.update_bounding_box();
    println!("map dimensions: {}", map);
    // map.print();
    aoc15_1(&map);
    aoc15_2(&map);
}
