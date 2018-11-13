extern crate aocutils;
extern crate regex;

mod part1;
mod part2;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Packet {
    grid: Vec<Vec<char>>,
    xx: usize,
    yy: usize,
    dir: Direction,
    letters: Vec<char>,
    steps: usize,
}

impl Packet {
    pub fn step(&mut self) -> bool {
        match self.dir {
            Direction::Up => {
                self.xx -= 1;
            },
            Direction::Down => {
                self.xx += 1;
            },
            Direction::Left => {
                self.yy -= 1;
            },
            Direction::Right => {
                self.yy += 1;
            }
        }

        match self.get_field_value() {
            ' ' => return true,
            '|' | '-' => {},
            '+' => {
                self.turn();
            },
            other => self.letters.push(other),
        }

        self.steps += 1;

        false
    }

    pub fn get_field_value(&self) -> char {
        self.get_field_value_by_pos(self.xx, self.yy)
    }

    fn get_field_value_by_pos(&self, xx: usize, yy: usize) -> char {
        if xx < self.grid.len() {
            if yy < self.grid[xx].len() {
                self.grid[xx][yy]
            } else {
                ' '
            }
        } else {
            ' '
        }
    }

    fn get_field_value_left(&self) -> char {
        self.get_field_value_by_pos(self.xx, self.yy - 1)
    }

    fn get_field_value_right(&self) -> char {
        self.get_field_value_by_pos(self.xx, self.yy + 1)
    }

    fn get_field_value_up(&self) -> char {
        self.get_field_value_by_pos(self.xx - 1, self.yy)
    }

    fn get_field_value_down(&self) -> char {
        self.get_field_value_by_pos(self.xx + 1, self.yy)
    }

    fn turn(&mut self) {
        self.dir = match self.dir {
            Direction::Up => {
                if self.get_field_value_left() != ' ' {
                    Direction::Left
                } else if self.get_field_value_right() != ' ' {
                    Direction::Right
                } else {
                    panic!("Invalid grid detected!");
                }
            },
            Direction::Down => {
                if self.get_field_value_left() != ' ' {
                    Direction::Left
                } else if self.get_field_value_right() != ' ' {
                    Direction::Right
                } else {
                    panic!("Invalid grid detected!");
                }
            },
            Direction::Left => {
                if self.get_field_value_up() != ' ' {
                    Direction::Up
                } else if self.get_field_value_down() != ' ' {
                    Direction::Down
                } else {
                    panic!("Invalid grid detected!");
                }
            },
            Direction::Right => {
                if self.get_field_value_up() != ' ' {
                    Direction::Up
                } else if self.get_field_value_down() != ' ' {
                    Direction::Down
                } else {
                    panic!("Invalid grid detected!");
                }
            }
        }
    }
}

fn main() {
    let day: i32 = 19;

    let grid: Vec<Vec<char>> = aocutils::import(day, Some("puzzle1"))
        .iter()
        .map(|line| {
            line.chars().collect()
        })
        .collect();

    let res1 = part1::solve(grid.clone());
    let res2 = part2::solve(grid);
    println!("Results: {} and {}", res1, res2);
}
