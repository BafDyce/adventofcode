extern crate aoc_utils;
#[macro_use] extern crate lazy_static;
extern crate regex;

mod part1;
mod part2;

use aoc_utils::prelude::*;
use regex::Regex;
use std::{collections::VecDeque, env};

const DAY: u32 = 9;
type InputType = (usize, usize);

#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    marbles: VecDeque<usize>,
    current: usize,
}

impl Circle {
    pub fn new() -> Self {
        let mut data = VecDeque::new();
        data.push_back(0);
        Circle {
            marbles: data,
            current: 0,
        }
    }

    // Stolen from /u/ninja_tokumei (My original solution is below)
    // Reddit: https://old.reddit.com/r/adventofcode/comments/a4i97s/2018_day_9_solutions/ebeq6ba/
    // GitLab: https://gitlab.com/AGausmann/puzzles/blob/master/adventofcode/2018/09a.rs
    pub fn add_marble(&mut self, pts: usize) -> usize {
        if pts % 23 == 0 {
            for _ in 0..7 {
                let back = self.marbles.pop_back().unwrap();
                self.marbles.push_front(back);
            }
            pts + self.marbles.pop_front().unwrap()
        } else {
            for _ in 0..2 {
                let front = self.marbles.pop_front().unwrap();
                self.marbles.push_back(front);
            }
            self.marbles.push_front(pts);

            0
        }
    }

    /*
    Original solution (ran for over an hour for part2):
    pub fn add_marble(&mut self, pts: usize) -> usize {
        if pts % 23 == 0 {
            let mut idx_rem = (self.current as isize - 7) % self.marbles.len() as isize;
            if idx_rem < 0 {
                idx_rem += self.marbles.len() as isize;

            }
            self.current = idx_rem as usize;
            pts + self.marbles.remove(idx_rem as usize).unwrap()
        } else {
            if self.marbles.is_empty() {
                self.marbles.push_back(pts);
            } else {
                let new_idx = if self.current == self.marbles.len() - 1 {
                    let new_idx = 1;
                    self.marbles.insert(new_idx, pts);
                    new_idx
                } else {
                    let new_idx = self.current + 2;
                    if new_idx == self.marbles.len() {
                        self.marbles.push_back(pts);
                    } else {
                        self.marbles.insert(new_idx, pts);
                    }

                    new_idx
                };
                self.current = new_idx;
            }

            0
        }
    }
    */
}

fn main() {
    // READ input
    let args: Vec<String> = env::args().collect();

    // Parse command line arguments
    let input_name = if args.len() > 1 {
        &args[1]
    } else {
        "puzzle1"
    };
    let verbose = args.contains(&String::from("-v")) || args.contains(&String::from("--verbose"));

    if verbose {
        println!("Loading data from input file {}", input_name);
    }
    // READ & PARSE input
    let (input, puzzle_config) = parse_input(input_name, verbose);

    // SOLVE puzzles
    let res1 = part1::solve(&input, &puzzle_config);
    let res2 = part2::solve(&input, &puzzle_config);

    println!("results: {} and {}", res1, res2);
}

fn parse_input(input_name: &str, verbose: bool) -> (InputType, PuzzleConfig) {
    let config = ImportConfig::new(2018, DAY, "../../_inputs/day{day}/");
    let (input, puzzle_config) = import_with_puzzle_config(&config, input_name).unwrap();
    if verbose {
        println!("raw input: {:?}", input);
    }

    // PARSE input
    lazy_static! {
        // (?x)
        // (?P<name>xxx)
        static ref RE: Regex = Regex::new(
            r"(?P<players>\d+) players; last marble is worth (?P<points>\d+) points"
        ).unwrap();
    }

    let caps = RE.captures(&input[0]).unwrap();
    let players = caps["players"].parse::<usize>().unwrap();
    let points = caps["points"].parse::<usize>().unwrap();
    let data = (players, points);


    if verbose {
        println!("input parsed: {:?}", data);
        println!("config: {:?}", puzzle_config);
    }
    (data, puzzle_config)
}
