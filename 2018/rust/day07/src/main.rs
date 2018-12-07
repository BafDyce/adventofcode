extern crate aoc_utils;
#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate md5;
extern crate sha1;
extern crate petgraph;

mod part1;
mod part2;

use aoc_utils::prelude::*;
use regex::Regex;
use std::{collections::HashMap, env};

const DAY: u32 = 7;
type ParseResult = (HashMap<char, Vec<char>>, HashMap<char, bool>);

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
    let res1 = part1::solve(&input.0, &input.1);
    let res2 = part2::solve(&input.0, &input.1);

    println!("results: {} and {}", res1, res2);
}

fn parse_input(input_name: &str, verbose: bool) -> ParseResult {
    let config = ImportConfig::new(2018, DAY, "../../_inputs/day{day}/");
    let input = import(&config, input_name).unwrap();
    if verbose {
        println!("raw input: {:?}", input);
    }

    // PARSE input
    let data: Vec<_> = input.into_iter().map(|line| {
        // Parsing logic

        // regex parsing stuff
        lazy_static! {
            // (?x)
            // (?P<name>xxx)
            static ref RE: Regex = Regex::new(
                r"Step (?P<start>[A-Z]) must be finished before step (?P<end>[A-Z]) can begin."
            ).unwrap();
        }

        let caps = RE.captures(&line).unwrap();
        let start = caps["start"].chars().next().unwrap();
        let end = caps["end"].chars().next().unwrap();
        //println!("{}, {}", start, end);
        (start, end)
    })
    .collect();

    let mut visited: HashMap<char, bool> = HashMap::new();
    let mut deps: HashMap<char, Vec<char>> = HashMap::new();

    for (start, end) in data.iter() {
        visited.insert(*start, false);
        if deps.get(start).is_none() {
            deps.insert(*start, Vec::new());
        }
        visited.insert(*end, false);
        let node_deps = deps.entry(*end).or_insert(Vec::new());
        (*node_deps).push(*start);
    }

    if verbose {
        println!("input parsed:\ndeps: {:?}\nvisited: {:?}", deps, visited);
    }

    (deps, visited)
}
