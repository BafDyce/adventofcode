extern crate aoc_utils;

mod part1;
mod part2;

use aoc_utils::prelude::*;
use std::env;

const DAY: u32 = 11;
type InputType = isize;

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
    let (res1_xx, res1_yy) = part1::solve(&input, &puzzle_config);
    let (res2_xx, res2_yy, res2_size) = part2::solve(&input, &puzzle_config);

    println!("results: {},{} and {},{},{}", res1_xx, res1_yy, res2_xx, res2_yy, res2_size);
}

fn parse_input(input_name: &str, verbose: bool) -> (InputType, PuzzleConfig) {
    let config = ImportConfig::new(2018, DAY, "../../_inputs/day{day}/");
    let (input, puzzle_config) = import_with_puzzle_config(&config, input_name).unwrap();
    if verbose {
        println!("raw input: {:?}", input);
    }

    // PARSE input
    let data = input[0].parse::<InputType>().unwrap_or(0);

    if verbose {
        println!("input parsed: {:?}", data);
        println!("config: {:?}", puzzle_config);
    }
    (data, puzzle_config)
}
