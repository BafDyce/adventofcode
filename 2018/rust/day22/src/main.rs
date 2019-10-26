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

const DAY: u32 = 22;
type InputType = Data;

#[derive(Debug, Clone, PartialEq)]
pub struct Data {
    pub depth: usize,
    pub target: Location2D,
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


    let regex_depth = Regex::new(
        r"depth: (\d+)"
    ).unwrap();

    let regex_target = Regex::new(
        r"target: (\d+),(\d+)"
    ).unwrap();

    let caps_depth = regex_depth.captures(&input[0]).unwrap();
    let caps_target = regex_target.captures(&input[1]).unwrap();

    let data = Data {
        depth: caps_depth[1].parse().unwrap(),
        target: Location2D::new(caps_target[1].parse().unwrap(), caps_target[2].parse().unwrap()),
    };

    if verbose {
        println!("input parsed: {:?}", data);
        println!("config: {:?}", puzzle_config);
    }
    (data, puzzle_config)
}
