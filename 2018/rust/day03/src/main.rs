extern crate aoc_utils;
#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate md5;
extern crate sha1;

mod part1;
mod part2;

use aoc_utils::prelude::*;
use regex::Regex;
use std::env;

const DAY: u32 = 3;
type InputTypeSingle = Claim;
type InputType = Vec<InputTypeSingle>;

#[derive(Debug)]
pub struct Claim {
    pub id: usize,
    pub left: usize,
    pub top: usize,
    pub width: usize,
    pub height: usize,
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

        // regex parsing stuff
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"\#(?P<id>[0-9]+) @ (?P<left>\d+),(?P<top>\d+): (?P<width>\d+)x(?P<height>\d+)"
            ).unwrap();
        }

        let caps = RE.captures(&line).unwrap();
        let id = caps["id"].parse::<usize>().unwrap();
        let left = caps["left"].parse::<usize>().unwrap();
        let top = caps["top"].parse::<usize>().unwrap();
        let width = caps["width"].parse::<usize>().unwrap();
        let height = caps["height"].parse::<usize>().unwrap();
        Claim {
            id,
            left,
            top,
            width,
            height,
        }
    })
    .collect();

    if verbose {
        println!("input parsed: {:?}", data);
    }
    data
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
    let input = parse_input(input_name, verbose);

    // SOLVE puzzles
    let res1 = part1::solve(&input);
    let res2 = part2::solve(&input);

    println!("results: {} and {}", res1, res2);
}
