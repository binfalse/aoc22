use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::multi::many0;
use nom::IResult;
use parse_int::parse;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
struct Position {
    x: u16,
    y: u16,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Direction {
    LEFT,
    RIGHT,
    TOP,
    BOTTOM,
}

impl Direction {
    fn facing_value(&self) -> u16 {
        match self {
            Direction::LEFT => 2,
            Direction::RIGHT => 0,
            Direction::TOP => 3,
            Direction::BOTTOM => 1,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Field {
    EMPTY,
    OPEN(Option<Direction>),
    SOLID,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Go {
    STEPS(u8),
    TURN(Direction),
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Field::EMPTY => " ",
            Field::OPEN(Some(Direction::LEFT)) => "<",
            Field::OPEN(Some(Direction::RIGHT)) => ">",
            Field::OPEN(Some(Direction::BOTTOM)) => "v",
            Field::OPEN(Some(Direction::TOP)) => "^",
            Field::OPEN(None) => ".",
            Field::SOLID => "#",
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, Clone)]
struct Map {
    map: Vec<Vec<Field>>,
    max_len: usize,
    current_position: Option<Position>,
    current_direction: Direction,
}
impl Map {
    fn new() -> Map {
        Map {
            map: vec![],
            current_position: None,
            current_direction: Direction::RIGHT,
            max_len: 0,
        }
    }

    fn add_line(&mut self, line: &str) {
        let row: Vec<Field> = line
            .chars()
            .enumerate()
            .map(|(i, c)| match c {
                ' ' => Field::EMPTY,
                '.' => {
                    if self.current_position == None {
                        self.current_position = Some(Position {
                            x: i as u16,
                            y: self.map.len() as u16,
                        });
                    }
                    Field::OPEN(None)
                }
                '#' => Field::SOLID,
                _ => panic!("don't understand field value {}", c),
            })
            .collect();
        if row.len() > self.max_len {
            self.max_len = row.len()
        }
        self.map.push(row);
    }

    fn fill_up(&mut self) {
        for row in 0..self.map.len() {
            while self.map[row].len() < self.max_len {
                self.map[row].push(Field::EMPTY);
            }
        }
    }

    fn go_next_position(&mut self, position: &Position, direction: &Direction) -> Option<Position> {
        match direction {
            Direction::LEFT => {
                let quest = if position.x > 0 {
                    Position {
                        x: position.x - 1,
                        y: position.y,
                    }
                } else {
                    Position {
                        x: (self.map[position.y as usize].len() - 1) as u16,
                        y: position.y,
                    }
                };
                match self.map[quest.y as usize][(quest.x) as usize] {
                    Field::EMPTY => {
                        return self.go_next_position(&quest, direction);
                    }
                    Field::OPEN(_) => {
                        self.map[quest.y as usize][quest.x as usize] =
                            Field::OPEN(Some(Direction::LEFT));
                        return Some(quest);
                    }
                    Field::SOLID => return None,
                }
            }
            Direction::RIGHT => {
                let mut quest = Position {
                    x: position.x + 1,
                    y: position.y,
                };
                if quest.x >= self.map[position.y as usize].len() as u16 {
                    quest.x = 0
                }
                match self.map[quest.y as usize][(quest.x) as usize] {
                    Field::EMPTY => {
                        return self.go_next_position(&quest, direction);
                    }
                    Field::OPEN(_) => {
                        self.map[quest.y as usize][quest.x as usize] =
                            Field::OPEN(Some(Direction::RIGHT));
                        return Some(quest);
                    }
                    Field::SOLID => return None,
                }
            }
            Direction::TOP => {
                let quest = if position.y > 0 {
                    Position {
                        x: position.x,
                        y: position.y - 1,
                    }
                } else {
                    Position {
                        x: position.x,
                        y: (self.map.len() - 1) as u16,
                    }
                };

                match self.map[quest.y as usize][(quest.x) as usize] {
                    Field::EMPTY => {
                        return self.go_next_position(&quest, direction);
                    }
                    Field::OPEN(_) => {
                        self.map[quest.y as usize][quest.x as usize] =
                            Field::OPEN(Some(Direction::TOP));
                        return Some(quest);
                    }
                    Field::SOLID => return None,
                }
            }
            Direction::BOTTOM => {
                let mut quest = Position {
                    x: position.x,
                    y: position.y + 1,
                };
                if quest.y >= self.map.len() as u16 {
                    quest.y = 0
                }
                match self.map[quest.y as usize][(quest.x) as usize] {
                    Field::EMPTY => {
                        return self.go_next_position(&quest, direction);
                    }
                    Field::OPEN(_) => {
                        self.map[quest.y as usize][quest.x as usize] =
                            Field::OPEN(Some(Direction::BOTTOM));
                        return Some(quest);
                    }
                    Field::SOLID => return None,
                }
            }
        }
    }

    fn go(&mut self, path: Vec<Go>) {
        let mut current_position = if let Some(p) = self.current_position {
            p.clone()
        } else {
            panic!("unknown current postion")
        };
        let mut current_direction = self.current_direction.clone();
        for path_element in path.iter() {
            match path_element {
                Go::STEPS(n_steps) => {
                    for _ in 0..*n_steps {
                        if let Some(p) =
                            self.go_next_position(&mut current_position, &current_direction)
                        {
                            current_position = p;
                        } else {
                            break;
                        }
                    }
                }
                Go::TURN(direction) => match (&current_direction, direction) {
                    (Direction::LEFT, Direction::LEFT) => current_direction = Direction::BOTTOM,
                    (Direction::LEFT, Direction::RIGHT) => current_direction = Direction::TOP,
                    (Direction::RIGHT, Direction::LEFT) => current_direction = Direction::TOP,
                    (Direction::RIGHT, Direction::RIGHT) => current_direction = Direction::BOTTOM,
                    (Direction::TOP, Direction::LEFT) => current_direction = Direction::LEFT,
                    (Direction::TOP, Direction::RIGHT) => current_direction = Direction::RIGHT,
                    (Direction::BOTTOM, Direction::LEFT) => current_direction = Direction::RIGHT,
                    (Direction::BOTTOM, Direction::RIGHT) => current_direction = Direction::LEFT,
                    _ => panic!(
                        "unexpected directions {:?} => {:?}",
                        current_direction, direction
                    ),
                },
            }
            self.current_position = Some(current_position);
        }

        println!(
            "final position: {:?} with direction: {:?}",
            self.current_position, self.current_direction
        );

        println!(
            "{}*1000 + {}*4 + {} = {}",
            self.current_position.unwrap().y + 1,
            self.current_position.unwrap().x + 1,
            self.current_direction.facing_value(),
            1000 * (self.current_position.unwrap().y as u32 + 1)
                + 4 * (self.current_position.unwrap().x as u32 + 1)
                + self.current_direction.facing_value() as u32
        );
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                if let Some(p) = self.current_position {
                    if p.x == x as u16 && p.y == y as u16 {
                        write!(f, "O")?;
                        continue;
                    }
                }
                write!(f, "{}", self.map[y][x])?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn parser_digit(input: &str) -> IResult<&str, Go> {
    let recognised: IResult<&str, &str> = digit1(input);

    if let Ok((input, result)) = recognised {
        let number = parse::<u8>(&result);
        if let Ok(n) = number {
            return Ok((input, Go::STEPS(n)));
        }
    }

    return Err(nom::Err::Error(nom::error::Error::new(
        "unrecognised digit",
        nom::error::ErrorKind::NoneOf,
    )));
}

fn parser_direction(input: &str) -> IResult<&str, Go> {
    let (input, result) = alt((tag("L"), tag("R"), tag("T"), tag("B")))(input)?;
    let direction = match result {
        "L" => Direction::LEFT,
        "R" => Direction::RIGHT,
        "B" => Direction::BOTTOM,
        "T" => Direction::TOP,
        _ => {
            return Err(nom::Err::Error(nom::error::Error::new(
                "unrecognised direction...",
                nom::error::ErrorKind::Fail,
            )))
        }
    };
    Ok((input, Go::TURN(direction)))
}

fn parser(input: &str) -> IResult<&str, Vec<Go>> {
    many0(alt((parser_direction, parser_digit)))(input)
}

fn aoc22_1() {
    println!("solving AOC day 22 part 1");
    let reader = BufReader::new(File::open("input-22").unwrap());
    let mut map = Map::new();
    let mut parse_map = true;
    let mut path: Vec<Go> = vec![];
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line == "" {
            parse_map = false;
            continue;
        }
        if parse_map {
            map.add_line(&line);
        } else {
            let result = parser(&line);
            if let Ok(x) = result {
                path = x.1;
            } else {
                panic!("couldn't parse move vector");
            }
        }
    }
    map.fill_up();
    map.go(path);
}

fn aoc22_2() {
    println!("solving AOC day 22 part 2");
    println!("solution: LOL no...");
}

pub fn aoc22() {
    aoc22_1();
    aoc22_2();
}
