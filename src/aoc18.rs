use min_max::*;
// use parse_int::parse;
// use substring::Substring;
use parse_int::parse;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
#[derive(Debug, Clone, PartialEq, Eq)]
struct Cube {
    x: isize,
    y: isize,
    z: isize,
}

impl Cube {
    fn new(line: &str) -> Cube {
        let coord = line.split(",").collect::<Vec<&str>>();
        assert!(coord.len() == 3);
        Cube {
            x: parse::<isize>(coord.get(0).unwrap()).unwrap(),
            y: parse::<isize>(coord.get(1).unwrap()).unwrap(),
            z: parse::<isize>(coord.get(2).unwrap()).unwrap(),
        }
    }

    fn fingerprint(&self) -> isize {
        Cube::fingerprint_position(self.x, self.y, self.z)
    }

    fn fingerprint_position(x: isize, y: isize, z: isize) -> isize {
        x + y * 100 + z * 10000
    }

    fn get_neighbor_fingerprints(&self) -> Vec<isize> {
        vec![
            Cube::fingerprint_position(self.x + 1, self.y, self.z),
            Cube::fingerprint_position(self.x - 1, self.y, self.z),
            Cube::fingerprint_position(self.x, self.y + 1, self.z),
            Cube::fingerprint_position(self.x, self.y - 1, self.z),
            Cube::fingerprint_position(self.x, self.y, self.z + 1),
            Cube::fingerprint_position(self.x, self.y, self.z - 1),
        ]
    }
}
struct BoundingBox {
    x: isize,
    y: isize,
    z: isize,
}
impl BoundingBox {
    fn new() -> BoundingBox {
        BoundingBox { x: 0, y: 0, z: 0 }
    }
    fn update(&mut self, cube: &Cube) {
        self.x = max!(self.x, cube.x);
        self.y = max!(self.y, cube.y);
        self.z = max!(self.z, cube.z);
    }
}

fn aoc18_1(cubes: &Vec<Cube>, coords: &HashSet<isize>) {
    println!("solving AOC day 18 part 1");

    let mut score = 0;
    for cube in cubes.iter() {
        let neighbors = cube.get_neighbor_fingerprints();
        for neighbor in neighbors.iter() {
            if !coords.contains(neighbor) {
                score += 1;
            }
        }
    }
    println!("cubes exposed to air: {}", score);
}

fn aoc18_2(cubes: &Vec<Cube>, cube_coords: &HashSet<isize>, bounding_box: &BoundingBox) {
    println!("solving AOC day 18 part 2");

    let mut space_coords = HashSet::new();

    let mut change_smt = true;
    while change_smt {
        change_smt = false;
        for x in -1..=bounding_box.x + 1 {
            for y in -1..=bounding_box.y + 1 {
                for z in -1..=bounding_box.z + 1 {
                    let c = Cube { x, y, z };
                    let cfp = Cube::fingerprint_position(x, y, z);
                    if cube_coords.contains(&cfp) || space_coords.contains(&cfp) {
                        continue;
                    }
                    if x == 0 || y == 0 || z == 0 {
                        space_coords.insert(cfp);
                        change_smt = true;
                        continue;
                    }

                    for neighbor in c.get_neighbor_fingerprints().iter() {
                        if space_coords.contains(neighbor) {
                            space_coords.insert(cfp);
                            change_smt = true;
                        }
                    }
                }
            }
        }
    }

    let mut score = 0;
    for cube in cubes.iter() {
        let neighbors = cube.get_neighbor_fingerprints();
        for neighbor in neighbors.iter() {
            if space_coords.contains(neighbor) {
                score += 1;
            }
        }
    }
    println!("cubes exposed to fresh air: {}", score);
}

pub fn aoc18() {
    let reader = BufReader::new(File::open("input-18").unwrap());

    let mut cubes: Vec<Cube> = vec![];
    let mut coords = HashSet::new();
    let mut bounding_box = BoundingBox::new();

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let cube = Cube::new(&line);
        coords.insert(cube.fingerprint());
        bounding_box.update(&cube);
        cubes.push(cube);
    }

    aoc18_1(&cubes, &coords);
    aoc18_2(&cubes, &coords, &bounding_box);
}
