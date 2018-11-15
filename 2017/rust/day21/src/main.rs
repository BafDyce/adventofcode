extern crate aocutils;
#[macro_use]
extern crate lazy_static;
extern crate regex;

mod part1;
mod part2;

use regex::Regex;
use std::slice::Chunks;

#[derive(Debug)]
pub enum ExpansionRule {
    ExpandTwo( [[char; 2]; 2], [[char; 3]; 3]),
    ExpandThree( [[char; 3]; 3], [[char; 4]; 4] ),
}

#[derive(Debug)]
struct Grid {
    pub gg: Vec<Vec<char>>,
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            gg: vec![
                vec!['.', '#', '.'],
                vec!['.', '.', '#'],
                vec!['#', '#', '#'],
            ],
        }
    }

    pub fn expand_with_rules(&mut self, rules: &[ExpansionRule]) {
        let splits = self.split();

        let splits: Vec<Vec<GridSegment>> = splits.into_iter().map(|splitrow| {
            splitrow.into_iter().map(|split| {
                let mut result = None;
                for rule in rules {
                    if let Ok(expanded) = split.expand(rule) {
                        result = Some(expanded);
                        break;
                    }
                }

                if let Some(result) = result {
                    result
                } else {
                    panic!("RIP");
                }
            })
            .collect()
        })
        .collect();

        let mut new_grid = Vec::new();
        for splitrow in splits {
            for ii in 0..splitrow[0].get_size() {
                let mut row: Vec<char> = Vec::new();
                for split in &splitrow {
                    row.extend(split.get_row(ii).unwrap())
                }
                new_grid.push(row);
            }
        }

        self.gg = new_grid;
    }

    fn split(&self) -> Vec<Vec<GridSegment>> {
        if self.gg.len() % 2 == 0 {
            self.split_n(2)
        } else {
            self.split_n(3)
        }
    }

    fn split_n(&self, splitsize: usize) -> Vec<Vec<GridSegment>> {
        self.gg
            .chunks(splitsize)
            .map(|chunkrows| {
                chunkrows.iter().map(|row| row.chunks(splitsize)).collect::<Vec<Chunks<char>>>()
            })
            .map(|mut chunkrows| {
                let mut results = Vec::new();
                let length = chunkrows[0].len();
                for _ in 0..length {
                    let mut segment: Vec<Vec<char>> = Vec::new();
                    for ii in 0..chunkrows.len() {
                        segment.push(chunkrows[ii].next().unwrap().to_vec())
                    }
                    results.push(GridSegment::from_vec(&segment));
                }
                results
            })
            .collect()
    }

    pub fn count_on(&self) -> u64 {
        let mut counter = 0;
        for row in &self.gg {
            for cell in row {
                if *cell == '#' {
                    counter += 1;
                }
            }
        }

        counter
    }
}

#[derive(Debug)]
enum GridSegment {
    Two( [[char; 2]; 2] ),
    Three( [[char; 3]; 3] ),
    Four( [[char; 4]; 4] ),
}

impl GridSegment {
    pub fn expand(&self, rule: &ExpansionRule) -> Result<GridSegment, ()> {
        match (self, rule) {
            (GridSegment::Two( cells ), ExpansionRule::ExpandTwo( exp_from, exp_to )) => {
                let mut exp_from: [[char; 2]; 2] = exp_from.clone();
                let mut exp_to: [[char; 3]; 3] = exp_to.clone();
                for _ in 1..=4 {
                    if *cells == exp_from || *cells == Self::flip_two(exp_from) {
                        return Ok(GridSegment::Three(exp_to));
                    }
                    exp_from = Self::rotate_two(exp_from);
                }
                Err(())
            }
            (GridSegment::Three( cells ), ExpansionRule::ExpandThree( exp_from, exp_to )) => {
                let mut exp_from: [[char; 3]; 3] = exp_from.clone();
                let mut exp_to: [[char; 4]; 4] = exp_to.clone();
                for _ in 1..=4 {
                    if *cells == exp_from || *cells == Self::flip_three(exp_from) {
                        return Ok(GridSegment::Four(exp_to));
                    }
                    exp_from = Self::rotate_three(exp_from);
                }
                Err(())
            },
            _ => Err(())
        }
    }

    pub fn from_vec(from: &[Vec<char>]) -> GridSegment {
        match from.len() {
            2 => {
                GridSegment::Two([
                    [from[0][0], from[0][1]],
                    [from[1][0], from[1][1]],
                ])
            },
            3 => {
                GridSegment::Three([
                    [from[0][0], from[0][1], from[0][2]],
                    [from[1][0], from[1][1], from[1][2]],
                    [from[2][0], from[2][1], from[2][2]],
                ])
            },
            4 => {
                GridSegment::Four([
                    [from[0][0], from[0][1], from[0][2], from[0][3]],
                    [from[1][0], from[1][1], from[1][2], from[1][3]],
                    [from[2][0], from[2][1], from[2][2], from[2][3]],
                    [from[3][0], from[3][1], from[3][2], from[3][3]],
                ])
            }
            _ => panic!("Invalid grid segment size: {:?}", from)
        }
    }

    pub fn get_row(&self, row: usize) -> Result<&[char], ()> {
        match self {
            GridSegment::Two(grid) if row < 2 => {
                Ok(&grid[row])
            }
            GridSegment::Three(grid) if row < 3 => {
                Ok(&grid[row])
            }
            GridSegment::Four(grid) if row < 4 => {
                Ok(&grid[row])
            }
            _ => Err(())
        }
    }

    pub fn get_size(&self) -> usize {
        match self {
            GridSegment::Two(_) => {
                2
            }
            GridSegment::Three(_) => {
                3
            }
            GridSegment::Four(_) => {
                4
            }
        }
    }

    // rotates the grid by 90 degrees clockwise
    fn rotate(self) -> GridSegment {
        match self {
            GridSegment::Two(old) => {
                GridSegment::Two(
                    [
                        [old[1][0], old[0][0]],
                        [old[1][1], old[0][1]]
                    ]
                )
            }
            GridSegment::Three(old) => {
                GridSegment::Three(
                    [
                        [old[2][0], old[1][0], old[0][0]],
                        [old[2][1], old[1][1], old[0][1]],
                        [old[2][2], old[1][2], old[0][2]],
                    ]
                )
            }
            GridSegment::Four(old) => {
                GridSegment::Four(
                    [
                        [old[3][0], old[2][0], old[1][0], old[0][0]],
                        [old[3][1], old[2][1], old[1][1], old[0][1]],
                        [old[3][2], old[2][2], old[1][2], old[0][2]],
                        [old[3][3], old[2][3], old[1][3], old[0][3]],
                    ]
                )
            }
        }
    }

    fn flip_two(old: [[char; 2]; 2]) -> [[char; 2]; 2] {
        [
            [old[1][0], old[1][1]],
            [old[0][0], old[0][1]],
        ]
    }

    fn flip_three(old: [[char; 3]; 3]) -> [[char; 3]; 3] {
        [
            [old[2][0], old[2][1], old[2][2]],
            [old[1][0], old[1][1], old[1][2]],
            [old[0][0], old[0][1], old[0][2]],
        ]
    }

    fn rotate_two(old: [[char; 2]; 2]) -> [[char; 2]; 2] {
        [
            [old[1][0], old[0][0]],
            [old[1][1], old[0][1]]
        ]
    }

    fn rotate_three(old: [[char; 3]; 3]) -> [[char; 3]; 3] {
        [
            [old[2][0], old[1][0], old[0][0]],
            [old[2][1], old[1][1], old[0][1]],
            [old[2][2], old[1][2], old[0][2]],
        ]
    }

    fn rotate_four(old: [[char; 4]; 4]) -> [[char; 4]; 4] {
        [
            [old[3][0], old[2][0], old[1][0], old[0][0]],
            [old[3][1], old[2][1], old[1][1], old[0][1]],
            [old[3][2], old[2][2], old[1][2], old[0][2]],
            [old[3][3], old[2][3], old[1][3], old[0][3]],
        ]
    }
}

fn main() {
    let day: i32 = 21;

    let rules: Vec<ExpansionRule> = aocutils::import(day, Some("puzzle1")).into_iter().map(|line| {
        lazy_static!{
            static ref RE_rule: Regex = Regex::new(
                r"(?x)
                (?P<expand_2>
                    ^
                    (?P<x2_from_1>[.\#]{2})/(?P<x2_from_2>[.\#]{2})
                    \s=>\s
                    (?P<x2_to_1>[.\#]{3})/(?P<x2_to_2>[.\#]{3})/(?P<x2_to_3>[.\#]{3})
                    $
                )
                |
                (?P<expand_3>
                    ^
                    (?P<x3_from_1>[.\#]{3})/(?P<x3_from_2>[.\#]{3})/(?P<x3_from_3>[.\#]{3})
                    \s=>\s
                    (?P<x3_to_1>[.\#]{4})/(?P<x3_to_2>[.\#]{4})/(?P<x3_to_3>[.\#]{4})/(?P<x3_to_4>[.\#]{4})
                    $
                )"
            ).unwrap();
        }

        //println!("line: {}", line);
        if let Some(captures) = RE_rule.captures(&line) {
            if let Some(_) = captures.name("expand_2") {
                let mut from_1 = captures["x2_from_1"].chars();
                let mut from_2 = captures["x2_from_2"].chars();
                let mut to_1 = captures["x2_to_1"].chars();
                let mut to_2 = captures["x2_to_2"].chars();
                let mut to_3 = captures["x2_to_3"].chars();
                //println!("{:?}/{} => {}/{}/{}", from_1, from_2, to_1, to_2, to_3);
                ExpansionRule::ExpandTwo(
                    [
                        [from_1.next().unwrap(), from_1.next().unwrap()],
                        [from_2.next().unwrap(), from_2.next().unwrap()],
                    ],
                    [
                        [to_1.next().unwrap(), to_1.next().unwrap(), to_1.next().unwrap()],
                        [to_2.next().unwrap(), to_2.next().unwrap(), to_2.next().unwrap()],
                        [to_3.next().unwrap(), to_3.next().unwrap(), to_3.next().unwrap()],
                    ]
                )
            } else if let Some(_) = captures.name("expand_3") {
                let mut from_1 = captures["x3_from_1"].chars();
                let mut from_2 = captures["x3_from_2"].chars();
                let mut from_3 = captures["x3_from_3"].chars();
                let mut to_1 = captures["x3_to_1"].chars();
                let mut to_2 = captures["x3_to_2"].chars();
                let mut to_3 = captures["x3_to_3"].chars();
                let mut to_4 = captures["x3_to_4"].chars();
                ExpansionRule::ExpandThree(
                    [
                        [from_1.next().unwrap(), from_1.next().unwrap(), from_1.next().unwrap()],
                        [from_2.next().unwrap(), from_2.next().unwrap(), from_2.next().unwrap()],
                        [from_3.next().unwrap(), from_3.next().unwrap(), from_3.next().unwrap()],
                    ],
                    [
                        [to_1.next().unwrap(), to_1.next().unwrap(), to_1.next().unwrap(), to_1.next().unwrap()],
                        [to_2.next().unwrap(), to_2.next().unwrap(), to_2.next().unwrap(), to_2.next().unwrap()],
                        [to_3.next().unwrap(), to_3.next().unwrap(), to_3.next().unwrap(), to_3.next().unwrap()],
                        [to_4.next().unwrap(), to_4.next().unwrap(), to_4.next().unwrap(), to_4.next().unwrap()],
                    ]
                )
            } else {
                panic!("Invalid input or invalid regex!");
            }
        } else {
            panic!("Invalid input or invalid regex!");
        }

    })
    .collect();

    let res1 = part1::solve(&rules);
    let res2 = part2::solve(&rules);
    println!("Results: {} and {}", res1, res2);
}
