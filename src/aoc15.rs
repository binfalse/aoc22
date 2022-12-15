use min_max::*;
use parse_int::parse;
use regex::Regex;
use std::collections::BTreeSet;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread::JoinHandle;
use std::time::Instant;
use std::{fmt, thread, time};

use fasthash::spooky::Hash32;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Sensor {
    x: isize,
    y: isize,
    dist: usize,
}
impl Sensor {
    fn dist(&self, x: isize, y: isize) -> usize {
        self.x.abs_diff(x) + self.y.abs_diff(y)
    }
    fn dist_p(&self, other: &Position) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
    fn get_options(&self, other_sensors: &Vec<Sensor>) -> Vec<Position> {
        let mut options = vec![];
        let coverage = self.dist as isize;

        for x in self.x - coverage - 1..=self.x + coverage + 1 {
            if x < 0 || x > 4000000 {
                continue;
            }
            let dist = self.dist(x, self.y) as isize;
            let y1 = self.y - (coverage - dist) - 1;
            if y1 >= 0 && y1 <= 4000000 {
                options.push(Position { x, y: y1 })
            }

            let y2 = self.y + (coverage - dist) + 1;
            if y2 >= 0 && y2 <= 4000000 {
                options.push(Position { x, y: y2 });
            }
        }
        let x: Vec<Position> = options
            .iter()
            .filter(|o| !other_sensors.iter().any(|s| s.dist_p(o) < s.dist))
            .map(|o| o.clone())
            .collect();
        x
    }
}

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
    fn get_options(&self, other: &Position) -> Vec<Position> {
        let mut options = vec![];
        let coverage = self.dist_p(other) as isize;

        for x in self.x - coverage - 1..=self.x + coverage + 1 {
            if x < 0 || x > 4000000 {
                continue;
            }
            let dist = self.dist(x, self.x) as isize;
            let y1 = self.y - (coverage - dist) - 1;
            if y1 >= 0 && y1 <= 4000000 {
                options.push(Position { x, y: y1 })
            }

            let y2 = self.y + (coverage - dist) + 1;
            if y2 >= 0 && y2 <= 4000000 {
                options.push(Position { x, y: y2 });
            }
            // let y1 = sensor.y - (sensor_coverage - dist) - 1;
            // let y2 = sensor.y - (sensor_coverage - dist) - 1;

            // if y1 >= miny && y1 <= maxy && Self::uncovered(&sensor, &beacon, x, y1) {
            //     return Some(x * 4000000 + y1);
            // }
            // if y2 >= miny && y2 <= maxy && Self::uncovered(x, y2) {
            //     return Some(x * 4000000 + y2);
            // }
        }
        //     }
        options
    }
}

#[derive(Debug, Clone)]
struct BoundingBox {
    minx: isize,
    maxx: isize,
}

impl BoundingBox {
    fn overlaps(&self, other: &BoundingBox) -> bool {
        let other_first = self.minx <= other.maxx && self.maxx >= other.minx;
        let this_first = other.minx <= self.maxx && other.maxx >= self.minx;
        other_first || this_first
    }

    fn extend(&mut self, other: &BoundingBox) {
        self.minx = min!(self.minx, other.minx);
        self.maxx = max!(self.maxx, other.maxx);
    }

    fn plus(&self, other: &BoundingBox) -> BoundingBox {
        BoundingBox {
            minx: min!(self.minx, other.minx),
            maxx: max!(self.maxx, other.maxx),
        }
    }
    fn size(&self) -> isize {
        return self.maxx - self.minx;
    }
}

#[derive(Debug, Clone)]
struct Map {
    sensors: Vec<Sensor>,
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
            let beacon = Position { x: bx, y: by };

            self.sensors.push(Sensor {
                x: sx,
                y: sy,
                dist: beacon.dist(sx, sy),
            });
            self.beacons.push(beacon);
        }
    }

    fn beacon_absense(&self, line: isize) -> isize {
        let mut handles: Vec<JoinHandle<Option<BoundingBox>>> = vec![];
        let mut coverage: Vec<BoundingBox> = vec![];

        for s in 0..self.sensors.len() {
            let sensor = self.sensors.get(s).unwrap().clone();
            let beacon = self.beacons.get(s).unwrap().clone();
            handles.push(thread::spawn(move || {
                let sensor_coverage = sensor.dist_p(&beacon) as isize;

                let mut cov_start: Option<isize> = None;
                for x in sensor.x - sensor_coverage..=sensor.x + sensor_coverage {
                    if sensor.dist(x, line) as isize <= sensor_coverage {
                        if cov_start == None {
                            cov_start = Some(x)
                        }
                    } else {
                        if let Some(c) = cov_start {
                            return Some(BoundingBox {
                                minx: c,
                                maxx: x - 1,
                            });
                        }
                    }
                }
                return None;
            }));
        }

        while handles.len() > 0 {
            let handle = handles.remove(0);
            if let Some(handle_coverage) = handle.join().unwrap() {
                coverage.push(handle_coverage.clone());
            }
        }
        coverage.sort_by(|a, b| a.minx.cmp(&b.minx));

        let mut size: isize = 0;
        let mut coverage2: Vec<BoundingBox> = vec![coverage.get(0).unwrap().clone()];
        let mut pushed = false;

        for c in 1..coverage.len() {
            let cov = coverage.get(c).unwrap();
            let l = coverage2.len();
            let last_c = coverage2.get_mut(l - 1).unwrap();
            if last_c.overlaps(&cov) {
                last_c.extend(cov);
                pushed = false;
            } else {
                size += last_c.size();
                pushed = true;
                coverage2.push(cov.clone());
            }
        }

        if !pushed {
            let last_c = coverage2.get(coverage2.len() - 1).unwrap();
            size += last_c.size();
        }

        return size;
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

    fn find_distress_beacon(&self) -> isize {
        let mut handles: Vec<JoinHandle<Vec<Position>>> = vec![];

        for s in 0..self.sensors.len() {
            let sensor = self.sensors.get(s).unwrap().clone();
            let all_sensors = self.sensors.clone();
            handles.push(thread::spawn(move || {
                return sensor.get_options(&all_sensors);
            }));
        }

        while handles.len() > 0 {
            let len = handles.len();
            let handle = handles.remove(len / 2);
            let options = handle.join().unwrap();
            if options.len() > 0 {
                let option = options.get(0).unwrap();
                return option.x * 4000000 + option.y;
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
    println!(
        "--solving AOC day 15 part 1\nsolution {}",
        map.beacon_absense(2000000)
    );
}

fn aoc15_2(map: &Map) {
    println!(
        "--solving AOC day 15 part 2\ndistress sig {}",
        map.find_distress_beacon()
    );
}

pub fn aoc15() {
    let reader = BufReader::new(File::open("input-15").unwrap());
    let mut map = Map::new();
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        map.set(&line);
    }

    let map2 = map.clone();

    let a = thread::spawn(move || {
        aoc15_1(&map);
    });
    let b = thread::spawn(move || {
        aoc15_2(&map2);
    });
    a.join().unwrap();
    b.join().unwrap();
}
