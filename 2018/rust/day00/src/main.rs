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

const DAY: u32 = 0;
type InputTypeSingle = usize;
type InputType = Vec<InputTypeSingle>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Data {
    dtype: DataType
}

impl Data {
    pub fn new() -> Self {
        Data {
            dtype: DataType::Aaaaaaaaaa
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataType {
    Aaaaaaaaaa
}

/*
template TODO:
- [x] Regular expression template stuff
- [x] take input name from command line parameter
- [ ] stuff for building a graph, and then:
    - [ ] dijkstra
    - [ ] TSP
- [x] crypto stuff (md5, sha1)
- [ ] base stuff for "assembly"
- [x] convert input into [[char; xx]; xx]
    --> in snippets
*/

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
        line.parse::<InputTypeSingle>().unwrap_or(0)
        ; // <-- REMOVE THIS IF NECESSARY!!

        // regex parsing stuff
        lazy_static! {
            // (?x)
            // (?P<name>xxx)
            static ref RE: Regex = Regex::new(
                r""
            ).unwrap();
        }

        let caps = RE.captures(&line).unwrap();
        // let thingy = &caps["thingy"];
        // let xx = caps["xx"].chars().next().unwrap();
        caps.len()
    })
    .collect();

    if verbose {
        println!("input parsed: {:?}", data);
        println!("config: {:?}", puzzle_config);
    }
    (data, puzzle_config)
}
