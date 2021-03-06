extern crate aoc_utils;

mod part1;
mod part2;

use aoc_utils::prelude::*;
use std::env;

const DAY: u32 = 1;
type InputTypeSingle = i64;
type InputType = Vec<InputTypeSingle>;


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
    let input = parse_input(input_name, verbose);

    // SOLVE puzzles
    let res1 = part1::solve(&input);
    let res2 = part2::solve(&input);

    println!("results: {} and {}", res1, res2);
}

fn parse_input(input_name: &str, verbose: bool) -> InputType {
    let config = ImportConfig::new(2018, DAY, "../../_inputs/day{day}/");
    let input = import(&config, input_name).unwrap();
    if verbose {
        println!("raw input: {:?}", input);
    }

    // PARSE input
    let data: InputType = input.into_iter().map(|line| {
        // Parsing logic
        line.parse::<InputTypeSingle>().unwrap()
    })
    .collect();

    if verbose {
        println!("input parsed: {:?}", data);
    }
    data
}
