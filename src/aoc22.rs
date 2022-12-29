use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::multi::many0;
use nom::IResult;
use parse_int::parse;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
struct Position {
    x: u16,
    y: u16,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

impl Direction {
    fn facing_value(&self) -> u16 {
        match self {
            Direction::LEFT => 2,
            Direction::RIGHT => 0,
            Direction::UP => 3,
            Direction::DOWN => 1,
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
            Field::OPEN(Some(Direction::DOWN)) => "v",
            Field::OPEN(Some(Direction::UP)) => "^",
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
    position_map: HashMap<(i16, i16, Direction), (u16, u16, Direction)>,
}
impl Map {
    fn new() -> Map {
        Map {
            map: vec![],
            current_position: None,
            current_direction: Direction::RIGHT,
            max_len: 0,
            position_map: HashMap::new(),
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

    fn gen_positionmap_part1(&mut self) {
        use Direction::*;
        self.position_map.clear();

        let s = (self.max_len / 3) as u16;
        assert_eq!(self.map.len() as u16, s * 4);

        for y in 0..s {
            self.position_map
                .insert((s as i16 - 1, y as i16, LEFT), (s * 3 - 1, y, LEFT));
            self.position_map
                .insert((s as i16 * 3, y as i16, RIGHT), (s, y, RIGHT));
        }
        for y in s..2 * s {
            self.position_map
                .insert((s as i16 - 1, y as i16, LEFT), (s * 2 - 1, y, LEFT));
            self.position_map
                .insert((s as i16 * 2, y as i16, RIGHT), (s, y, RIGHT));
        }
        for y in 2 * s..3 * s {
            self.position_map
                .insert((-1, y as i16, LEFT), (s * 2 - 1, y, LEFT));
            self.position_map
                .insert((s as i16 * 2, y as i16, RIGHT), (0, y, RIGHT));
        }
        for y in 3 * s..4 * s {
            self.position_map
                .insert((-1, y as i16, LEFT), (s - 1, y, LEFT));
            self.position_map
                .insert((s as i16, y as i16, RIGHT), (0, y, RIGHT));
        }

        for x in 0..s {
            self.position_map
                .insert((x as i16, s as i16 * 2 - 1, UP), (x, s * 4 - 1, UP));
            self.position_map
                .insert((x as i16, s as i16 * 4, DOWN), (x, s * 2, DOWN));
        }
        for x in s..2 * s {
            self.position_map
                .insert((x as i16, -1, UP), (x, s * 3 - 1, UP));
            self.position_map
                .insert((x as i16, s as i16 * 3, DOWN), (x, 0, DOWN));
        }
        for x in 2 * s..3 * s {
            self.position_map.insert((x as i16, -1, UP), (x, s - 1, UP));
            self.position_map
                .insert((x as i16, s as i16, DOWN), (x, 0, DOWN));
        }
    }

    fn gen_positionmap_part2(&mut self) {
        use Direction::*;
        self.position_map.clear();

        let s = (self.max_len / 3) as u16;
        assert_eq!(self.map.len() as u16, s * 4);

        for y in 0..s {
            self.position_map
                .insert((s as i16 - 1, y as i16, LEFT), (0, 3 * s - y - 1, RIGHT));
            self.position_map.insert(
                (s as i16 * 3, y as i16, RIGHT),
                (2 * s - 1, 3 * s - y - 1, LEFT),
            );
        }
        for y in s..2 * s {
            let field_pos = y - s;
            self.position_map
                .insert((s as i16 - 1, y as i16, LEFT), (field_pos, 2 * s, DOWN));
            self.position_map.insert(
                (s as i16 * 2, y as i16, RIGHT),
                (2 * s + field_pos, s - 1, UP),
            );
        }
        for y in 2 * s..3 * s {
            let field_pos = y - 2 * s;
            self.position_map
                .insert((-1, y as i16, LEFT), (s, s - field_pos - 1, RIGHT));
            self.position_map.insert(
                (s as i16 * 2, y as i16, RIGHT),
                (s * 3 - 1, s - field_pos - 1, LEFT),
            );
        }
        for y in 3 * s..4 * s {
            let field_pos = y - 3 * s;
            self.position_map
                .insert((-1, y as i16, LEFT), (s + field_pos, 0, DOWN));
            self.position_map
                .insert((s as i16, y as i16, RIGHT), (field_pos + s, 3 * s - 1, UP));
        }

        for x in 0..s {
            self.position_map
                .insert((x as i16, s as i16 * 2 - 1, UP), (s, s + x, RIGHT));
            self.position_map
                .insert((x as i16, s as i16 * 4, DOWN), (x + s * 2, 0, DOWN));
        }
        for x in s..2 * s {
            let field_pos = x - s;
            self.position_map
                .insert((x as i16, -1, UP), (0, 3 * s + field_pos, RIGHT));
            self.position_map.insert(
                (x as i16, s as i16 * 3, DOWN),
                (s - 1, 3 * s + field_pos, LEFT),
            );
        }
        for x in 2 * s..3 * s {
            let field_pos = x - 2 * s;
            self.position_map
                .insert((x as i16, -1, UP), (field_pos, 4 * s - 1, UP));
            self.position_map
                .insert((x as i16, s as i16, DOWN), (s * 2 - 1, s + field_pos, LEFT));
        }
    }

    fn fill_up(&mut self) {
        for row in 0..self.map.len() {
            while self.map[row].len() < self.max_len {
                self.map[row].push(Field::EMPTY);
            }
        }
    }

    fn next_position(
        &self,
        position: &Position,
        change_x: i16,
        change_y: i16,
        direction: Direction,
    ) -> (Position, Direction) {
        let key = (
            position.x as i16 + change_x,
            position.y as i16 + change_y,
            direction,
        );
        if self.position_map.contains_key(&key) {
            let p = self.position_map.get(&key).unwrap();
            return (Position { x: p.0, y: p.1 }, p.2);
        }
        return (
            Position {
                x: (position.x as i16 + change_x) as u16,
                y: (position.y as i16 + change_y) as u16,
            },
            direction,
        );
    }

    fn go_next_position(
        &mut self,
        position: &Position,
        direction: &Direction,
    ) -> Option<(Position, Direction)> {
        use Direction::*;
        use Field::*;
        match direction {
            LEFT => {
                let (quest, direction_update) = self.next_position(position, -1, 0, LEFT);
                match self.map[quest.y as usize][(quest.x) as usize] {
                    OPEN(_) => {
                        self.map[quest.y as usize][quest.x as usize] = OPEN(Some(LEFT));
                        return Some((quest, direction_update));
                    }
                    SOLID => return None,
                    _ => panic!(
                        "how can there be an empty field!? {:?} {:?} => {:?} {:?}",
                        position, direction, quest, direction_update
                    ),
                }
            }
            RIGHT => {
                let (quest, direction_update) = self.next_position(position, 1, 0, RIGHT);
                match self.map[quest.y as usize][(quest.x) as usize] {
                    OPEN(_) => {
                        self.map[quest.y as usize][quest.x as usize] = OPEN(Some(RIGHT));
                        return Some((quest, direction_update));
                    }
                    SOLID => return None,
                    _ => panic!(
                        "how can there be an empty field!? {:?} {:?} => {:?} {:?}",
                        position, direction, quest, direction_update
                    ),
                }
            }
            UP => {
                let (quest, direction_update) = self.next_position(position, 0, -1, UP);
                match self.map[quest.y as usize][(quest.x) as usize] {
                    OPEN(_) => {
                        self.map[quest.y as usize][quest.x as usize] = OPEN(Some(UP));
                        return Some((quest, direction_update));
                    }
                    SOLID => return None,
                    _ => panic!(
                        "how can there be an empty field!? {:?} {:?} => {:?} {:?}",
                        position, direction, quest, direction_update
                    ),
                }
            }
            DOWN => {
                let (quest, direction_update) = self.next_position(position, 0, 1, DOWN);
                match self.map[quest.y as usize][(quest.x) as usize] {
                    OPEN(_) => {
                        self.map[quest.y as usize][quest.x as usize] = OPEN(Some(DOWN));
                        return Some((quest, direction_update));
                    }
                    SOLID => return None,
                    _ => panic!(
                        "how can there be an empty field!? {:?} {:?} => {:?} {:?}",
                        position, direction, quest, direction_update
                    ),
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
                            (current_position, current_direction) = p;
                        } else {
                            break;
                        }
                    }
                }
                Go::TURN(direction) => match (&current_direction, direction) {
                    (Direction::LEFT, Direction::LEFT) => current_direction = Direction::DOWN,
                    (Direction::LEFT, Direction::RIGHT) => current_direction = Direction::UP,
                    (Direction::RIGHT, Direction::LEFT) => current_direction = Direction::UP,
                    (Direction::RIGHT, Direction::RIGHT) => current_direction = Direction::DOWN,
                    (Direction::UP, Direction::LEFT) => current_direction = Direction::LEFT,
                    (Direction::UP, Direction::RIGHT) => current_direction = Direction::RIGHT,
                    (Direction::DOWN, Direction::LEFT) => current_direction = Direction::RIGHT,
                    (Direction::DOWN, Direction::RIGHT) => current_direction = Direction::LEFT,
                    _ => panic!(
                        "unexpected directions {:?} => {:?}",
                        current_direction, direction
                    ),
                },
            }
            // self.current_position = Some(current_position);
        }

        println!(
            "final position: {:?} with direction: {:?}",
            current_position, current_direction
        );

        println!(
            "{}*1000 + {}*4 + {} = {}",
            current_position.y + 1,
            current_position.x + 1,
            current_direction.facing_value(),
            1000 * (current_position.y as u32 + 1)
                + 4 * (current_position.x as u32 + 1)
                + current_direction.facing_value() as u32
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
        "B" => Direction::DOWN,
        "T" => Direction::UP,
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
    map.gen_positionmap_part1();
    map.fill_up();
    map.go(path);
}

fn aoc22_2() {
    println!("solving AOC day 22 part 2");
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
    map.gen_positionmap_part2();
    map.fill_up();
    map.go(path);
}

pub fn aoc22() {
    aoc22_1();
    aoc22_2();
}
