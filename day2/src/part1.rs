
use anyhow::Result;
use std::fs::read_to_string;

#[derive(Debug)]
enum Direction {
    Forward(i32),
    Down(i32),
}

use Direction::{Down, Forward};

#[derive(Debug)]
enum DirectionParseError {
    WrongLen,
    Nan,
    NoDir,
}
use DirectionParseError::{Nan, NoDir, WrongLen};

impl std::error::Error for DirectionParseError {}

impl std::fmt::Display for DirectionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "displayparseerror")
    }
}


impl Direction {
    fn get(val: &str) -> Result<Self, DirectionParseError> {
        let sp: Vec<&str> = val.split(' ').collect();
        if sp.len() != 2 {
           return Err(WrongLen);
        }

        let v: i32 = match sp[1].parse() {
            Ok(i) => i,
            Err(_) => return Err(Nan),
        };
        
        let rc = match sp[0] {
            "forward" => Forward(v),
            "down" => Down(v),
            "up" => Down(-v),
            _ => return Err(NoDir),
        };

        Ok(rc)
    }
}

struct Position {
    y: i32,
    z: i32,
}

impl Position {
    fn new() -> Self {
        Position {
            y: 0,
            z: 0,
        }
    }

    fn update(&mut self, dir: Direction) {
       match dir {
           Forward(amt) => self.y += amt,
           Down(amt) => self.z += amt,
       }
    }

    fn answer(&self) -> i32 {
        self.y * self.z
    }
}

fn main() -> Result<()> {

    let file_data = read_to_string("input")?;

    let mut cur_pos = Position::new();

    for line in file_data.lines() {
        let dir = Direction::get(line)?;

        cur_pos.update(dir);
    }

    println!("answer is {}", cur_pos.answer());

    Ok(())
}
