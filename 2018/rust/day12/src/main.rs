extern crate aoc_utils;
#[macro_use] extern crate lazy_static;
extern crate regex;

mod part1;
mod part2;

use aoc_utils::prelude::*;
use regex::Regex;
use std::{collections::VecDeque, env};

const DAY: u32 = 12;
type InputTypeSingle = Rule;
type InputType = (String, Vec<InputTypeSingle>);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rule {
    pub from: [char; 5],
    pub to: char,
}

impl Rule {
    pub fn new() -> Self {
        Rule {
            from: [' '; 5],
            to: ' '
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

    let regex_init = Regex::new(
        r"initial state: ([.|\#]+)"
    ).unwrap();
    let caps = regex_init.captures(&input[0]).unwrap();
    let initial: String = caps[1].to_owned();

    if verbose {
        println!("init: {}", initial);
    }

    // PARSE input
    let data: Vec<Rule> = input.iter().skip(2).map(|line| {
        // Parsing logic
        // regex parsing stuff
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"([.|\#]{5}) => (.|\#)"
            ).unwrap();
        }

        let caps = RE.captures(&line).unwrap();
        if verbose {
            println!("caps: {:?} & {:?}", &caps[1], &caps[2]);
        }
        let mut froms = caps[1].chars();
        Rule {
            from: [
                froms.next().unwrap(),
                froms.next().unwrap(),
                froms.next().unwrap(),
                froms.next().unwrap(),
                froms.next().unwrap(),
            ],
            to: caps[2].chars().next().unwrap(),
        }
    })
    .collect();

    if verbose {
        println!("input parsed: {:?}", data);
        println!("config: {:?}", puzzle_config);
    }
    ((initial, data), puzzle_config)
}
