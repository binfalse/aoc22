use std::fmt::format;
use std::fs::File;
use std::io::{BufRead, BufReader};
use substring::Substring;

use parse_int::parse;
struct EFile {
    name: String,
    size: usize,
}

struct EDir<'a> {
    path: String,
    name: String,
    // parent: Option<Box<&'a EDir<'a>>>,
    dirs: Vec<&'a EDir<'a>>,
    files: Vec<EFile>,
    size: usize,
}

fn find_dir<'a>(root: &'a mut EDir<'a>, path: &str) -> Option<&'a mut EDir<'a>> {
    /*for f in root.files.iter() {
        if f == of {
            return root;
        }
    }*/
    if root.path == path {
        return Some(root);
    }
    for d in root.dirs.iter_mut() {
        return find_dir(d, path);
    }
    return None;
}

fn find_parend<'a>(root: &'a EDir<'a>, of: &'a EDir<'a>) -> Option<&'a EDir<'a>> {
    /*for f in root.files.iter() {
        if f == of {
            return root;
        }
    }*/
    for d in root.dirs.iter() {
        if d.path == of.path {
            return Some(d);
        }
        return find_parend(d, of);
    }
    return None;
}

fn aoc07_1() {
    println!("solving AOC day 7 part 1");
    let reader = BufReader::new(File::open("input-07").unwrap());

    let mut score: usize = 0;
    let mut current_path: &str = "/";

    let mut root_dir = EDir {
        path: "/".to_string(),
        name: "".to_string(),
        dirs: vec![],
        files: vec![],
        // parent: None,
        size: 0,
    };
    // let mut current_dir: &mut EDir; // = &mut root_dir;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line == "$ cd /" {
            // current_dir = &mut root_dir;
            current_path = "/";
            continue;
        }
        if line == "$ ls" {
            continue;
        }
        let mut dir = find_dir(&mut root_dir, &current_path).unwrap();
        if line.starts_with("$ cd ") {
            let d = line.substring(5, line.len());
            if d == ".." {
                let parent = find_parend(&root_dir, dir).unwrap();
                current_path = &parent.path; //current_dir.parent.unwrap().as_mut();
            } else {
                current_path = &format!("{}/{}", current_path, d);
                /*for d in current_dir.dirs.iter_mut() {
                    if d.name == dir {
                        current_dir = &mut d;
                        break;
                    }
                }*/
            }
            continue;
        }
        if line.starts_with("dir ") {
            let filename = line.substring(4, line.len()).to_string();
            let nd = EDir {
                path: format!("{}/{}", current_path, filename),
                name: filename,
                dirs: vec![],
                files: vec![],
                // parent: Option::Some(Box::new(current_dir)),
                size: 0,
            };

            dir.dirs.push(&nd);
            continue;
        }
        let parts = line.split(" ").collect::<Vec<&str>>();
        dir.files.push(EFile {
            name: parts[1].to_string(),
            size: parse::<usize>(parts[0]).unwrap(),
        })
    }

    println!("score: {}", score);
}

fn aoc07_2() {
    println!("solving AOC day 7 part 2");
    let reader = BufReader::new(File::open("input-07").unwrap());

    let mut score: usize = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
    }

    println!("score: {}", score);
}

pub fn aoc07() {
    aoc07_1();
    aoc07_2();
}
