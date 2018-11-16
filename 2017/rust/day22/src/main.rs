extern crate aocutils;
extern crate regex;

mod part1;
mod part2;

use std::collections::HashMap;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Position {
    pub xx: i32,
    pub yy: i32,
}

impl Position {
    pub fn new(xx: i32, yy: i32) -> Position {
        Position {
            xx,
            yy,
        }
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn reverse(&mut self) {
        *self = match self {
            Direction::Up => {
                Direction::Down
            },
            Direction::Right => {
                Direction::Left
            },
            Direction::Down => {
                Direction::Up
            },
            Direction::Left => {
                Direction::Right
            },
        }
    }

    pub fn turn_left(&mut self) {
        *self = match self {
            Direction::Up => {
                Direction::Left
            },
            Direction::Right => {
                Direction::Up
            },
            Direction::Down => {
                Direction::Right
            },
            Direction::Left => {
                Direction::Down
            },
        }
    }

    pub fn turn_right(&mut self) {
        *self = match self {
            Direction::Up => {
                Direction::Right
            },
            Direction::Right => {
                Direction::Down
            },
            Direction::Down => {
                Direction::Left
            },
            Direction::Left => {
                Direction::Up
            },
        }
    }
}

pub struct Virus {
    pos: Position,
    dir: Direction,
}

impl Virus {
    pub fn new() -> Virus {
        Virus {
            pos: Position::new(0, 0),
            dir: Direction::Up,
        }
    }

    pub fn forward(&mut self) {
        match self.dir {
            Direction::Up => {
                self.pos.xx -= 1;
            },
            Direction::Down => {
                self.pos.xx += 1;
            },
            Direction::Left => {
                self.pos.yy -= 1;
            },
            Direction::Right => {
                self.pos.yy += 1;
            }
        }
    }

    pub fn get_pos(&self) -> Position {
        self.pos.clone()
    }

    pub fn reverse(&mut self) {
        self.dir.reverse();
    }

    pub fn turn_left(&mut self) {
        self.dir.turn_left()
    }

    pub fn turn_right(&mut self) {
        self.dir.turn_right()
    }
}

fn main() {
    let day: i32 = 22;

    let input = aocutils::import(day, Some("puzzle1"));

    let res1 = part1::solve(&input);
    let res2 = part2::solve(&input);

    println!("Results: {} and {}", res1, res2);
}
