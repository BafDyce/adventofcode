extern crate aoc_utils;
#[macro_use] extern crate lazy_static;
extern crate regex;

mod part1;
mod part2;

use aoc_utils::prelude::*;
use regex::Regex;
use std::{env, num::ParseIntError, str::FromStr};

const DAY: u32 = 25;
type InputTypeSingle = Point;
type InputType = Vec<InputTypeSingle>;

#[derive(Debug)]
pub enum SpaceTimeError {
    PointSpecificationError,
    ParseIntError,
}

impl From<ParseIntError> for SpaceTimeError {
    fn from(_ee: ParseIntError) -> SpaceTimeError {
        SpaceTimeError::ParseIntError
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    aa: isize,
    bb: isize,
    cc: isize,
    dd: isize,
}

impl Point {
    pub fn manhatten(&self, other: &Point) -> isize {
        isize::abs(self.aa - other.aa)
        + isize::abs(self.bb - other.bb)
        + isize::abs(self.cc - other.cc)
        + isize::abs(self.dd - other.dd)
    }
}

impl FromStr for Point {
    type Err = SpaceTimeError;

    fn from_str(ss: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(-?\d+),(-?\d+),(-?\d+),(-?\d+)").unwrap();
        }

        match RE.captures(ss) {
            Some(ref caps) if caps.len() == 5 => {
                Ok(Point {
                    aa: caps[1].parse()?,
                    bb: caps[2].parse()?,
                    cc: caps[3].parse()?,
                    dd: caps[4].parse()?,
                })
            }
            _ => Err(SpaceTimeError::PointSpecificationError)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Constellation {
    points: Vec<Point>,
}

impl Constellation {
    pub fn new(pt: Point) -> Constellation {
        Constellation {
            points: vec![pt],
        }
    }

    pub fn belongs_to(&self, pt: &Point) -> bool {
        self.points
            .iter()
            .any(|point| pt.manhatten(point) <= 3)
    }

    pub fn add(&mut self, pt: Point) /*-> Result<(), ()>*/ {
        if self.belongs_to(&pt) {
            self.points.push(pt);
            //Ok(())
        } /*else {
            Err(())
        }*/
    }

    pub fn merge(&mut self, other: Constellation) {
        self.points.extend(other.points);
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
