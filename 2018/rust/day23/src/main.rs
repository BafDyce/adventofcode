extern crate aoc_utils;
#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate md5;
extern crate sha1;

mod part1;
mod part2;

use aoc_utils::prelude::*;
use regex::Regex;
use std::{collections::{HashMap, VecDeque}, env};
use std::str::FromStr;

const DAY: u32 = 23;
type InputTypeSingle = NanoBot;
type InputType = Vec<InputTypeSingle>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NanoBot {
    xx: isize,
    yy: isize,
    zz: isize,
    range: isize,
}

impl NanoBot {
    fn get_range(&self) -> isize {
        self.range
    }

    fn in_range(&self, other: &NanoBot) -> bool {
        self.manhatten(other) <= self.range
    }

    fn manhatten(&self, other: &NanoBot) -> isize {
        isize::abs(self.xx - other.xx)
        + isize::abs(self.yy - other.yy)
        + isize::abs(self.zz - other.zz)
    }

    fn xx(&self) -> isize {
        self.xx
    }

    fn yy(&self) -> isize {
        self.yy
    }

    fn zz(&self) -> isize {
        self.zz
    }

    fn reduce(&self, reduce_by: isize) -> Self {
        NanoBot {
            xx: self.xx / reduce_by,
            yy: self.yy / reduce_by,
            zz: self.zz / reduce_by,
            range: self.range / reduce_by,
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    InvalidFormat
}

impl FromStr for NanoBot {
    type Err = ParseError;

    fn from_str(ss: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"pos=<(?P<xx>-?\d+),(?P<yy>-?\d+),(?P<zz>-?\d+)>, r=(?P<range>\d+)"
            ).unwrap();
        }

        match RE.captures(ss) {
            Some(caps) => {
                Ok(NanoBot {
                    xx: caps["xx"].parse().unwrap(),
                    yy: caps["yy"].parse().unwrap(),
                    zz: caps["zz"].parse().unwrap(),
                    range: caps["range"].parse().unwrap(),
                })
            }
            None => Err(ParseError::InvalidFormat)
        }
    }
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
    let data: InputType = input.into_iter().map(|line| {
        // Parsing logic
        // single numeric types
        line.parse::<InputTypeSingle>().unwrap()
    })
    .collect();

    if verbose {
        println!("input parsed: {:?}", data);
        println!("config: {:?}", puzzle_config);
    }
    (data, puzzle_config)
}
