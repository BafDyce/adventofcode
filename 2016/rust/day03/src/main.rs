#[macro_use] extern crate lazy_static;

mod part1;
mod part2;

use aoc_utils::prelude::*;
use regex::Regex;
use std::env;

const DAY: u32 = 3;
type InputTypeSingle = Triangle;
type InputType = Vec<InputTypeSingle>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle {
    aa: u32,
    bb: u32,
    cc: u32,
}

impl Triangle {
    pub fn new(aa: u32, bb: u32, cc: u32) -> Self {
        Triangle {
            aa,
            bb,
            cc,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.aa + self.bb > self.cc
        && self.aa + self.cc > self.bb
        && self.bb + self.cc > self.aa
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
    //let (input2, puzzle_config) = parse_input_2(input_name, verbose);

    // SOLVE puzzles
    let res1 = part1::solve(&input, &puzzle_config);
    let res2 = part2::solve(&input, &puzzle_config);

    println!("results: {} and {}", res1, res2);
}

fn parse_input(input_name: &str, verbose: bool) -> (InputType, PuzzleConfig) {
    let config = ImportConfig::new(2016, DAY, "../../_inputs/day{day}/");
    let (input, puzzle_config) = import_with_puzzle_config(&config, input_name).unwrap();
    if verbose {
        println!("raw input: {:?}", input);
    }

    // PARSE input
    let data: InputType = input.into_iter().map(|line| {
        // regex parsing stuff
        lazy_static! {
            // (?x)
            // (?P<name>xxx)
            static ref RE: Regex = Regex::new(
                r"\s+(?P<aa>\d+)\s+(?P<bb>\d+)\s+(?P<cc>\d+)"
            ).unwrap();
        }

        let caps = RE.captures(&line).unwrap();
        // let thingy = &caps["thingy"];
        // let xx = caps["xx"].chars().next().unwrap();
        Triangle::new(
            caps["aa"].parse().unwrap(),
            caps["bb"].parse().unwrap(),
            caps["cc"].parse().unwrap(),
        )
    })
    .collect();

    if verbose {
        println!("input parsed: {:?}", data);
        println!("config: {:?}", puzzle_config);
    }
    (data, puzzle_config)
}