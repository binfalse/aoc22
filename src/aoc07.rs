use parse_int::parse;
use std::fs::File;
use std::io::{BufRead, BufReader};
use substring::Substring;

#[derive(Debug, Clone)]
struct Path {
    size: usize,
    path: String,
    file: bool,
}

fn get_parent<'a>(path: &'a str) -> &'a str {
    return path.substring(0, path.rfind("/").unwrap()).into();
}

fn get_dirsize(paths: &Vec<Path>, path: &str) -> usize {
    let mut s: usize = 0;
    for p in paths.iter() {
        if p.file && p.path.starts_with(path) {
            s += p.size
        }
    }
    return s;
}

fn aoc07_1(paths: &Vec<Path>) {
    println!("solving AOC day 7 part 1");

    let mut s = 0;

    for p in paths.iter() {
        if p.file {
            continue;
        }
        if p.size <= 100000 {
            s += p.size;
        }
    }

    println!("sum small sizes: {}", s);
}

fn aoc07_2(paths: &Vec<Path>) {
    println!("solving AOC day 7 part 2");

    let total_space = 70000000;
    let used_space = paths[0].size;
    let free_space = total_space - used_space;
    let need_to_free = 30000000 - free_space;

    let mut s = usize::MAX;
    for p in paths.iter() {
        if p.file {
            continue;
        }
        if p.size > need_to_free && p.size < s {
            s = p.size;
        }
    }
    println!("free that size: {}", s)
}

pub fn aoc07() {
    let reader = BufReader::new(File::open("input-07").unwrap());
    let mut paths: Vec<Path> = vec![Path {
        size: 0,
        path: "/".to_string(),
        file: false,
    }];
    let mut current_path: String = format!("/");

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line == "$ cd /" {
            current_path = format!("/");
            continue;
        }
        if line == "$ ls" {
            continue;
        }

        if line.starts_with("$ cd ") {
            let d = line.substring(5, line.len());
            if d == ".." {
                current_path = get_parent(&current_path).into();
            } else {
                current_path = current_path + "/" + d;
            }
            continue;
        }
        if line.starts_with("dir ") {
            paths.push(Path {
                path: format!(
                    "{}/{}",
                    current_path,
                    line.substring(4, line.len()).to_string()
                ),
                size: 0,
                file: false,
            });
            continue;
        }
        let parts = line.split(" ").collect::<Vec<&str>>();
        paths.push(Path {
            path: format!("{}/{}", current_path, parts[1]),
            size: parse::<usize>(parts[0]).unwrap(),
            file: true,
        })
    }

    for i in 0..paths.len() {
        let p = paths.get(i).unwrap();
        if p.file {
            continue;
        }
        let size = get_dirsize(&paths, &p.path);

        let mut p = paths.get_mut(i).unwrap();
        p.size = size;
    }

    aoc07_1(&paths);
    aoc07_2(&paths);
}
