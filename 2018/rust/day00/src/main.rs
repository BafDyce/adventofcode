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

const DAY: u32 = 0;
type InputType = Vec<usize>;

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
    let config = ImportConfig::new(2018, DAY, "../../_inputs/day{day}/");

    let input_name = if args.len() > 1 {
        &args[1]
    } else {
        "puzzle1"
    };

    let verbose = args.contains(&String::from("-v")) || args.contains(&String::from("--verbose"));

    if verbose {
        println!("Loading data from input file {}", input_name);
    }

    let input = import(&config, input_name).unwrap();
    if verbose {
        println!("raw input: {:?}", input);
    }

    // PARSE input
    let data: InputType = input.into_iter().map(|line| {
        // Parsing logic
        lazy_static! {
            // (?x)
            // (?P<name>xxx)
            static ref RE: Regex = Regex::new(r"").unwrap();
        }

        let caps = RE.captures(&line).unwrap();
        // let thingy = &caps["thingy"];
        caps.len()
    })
    .collect();

    if verbose {
        println!("input parsed: {:?}", data);
    }

    // SOLVE puzzles
    let res1 = part1::solve(&data);
    let res2 = part2::solve(&data);

    println!("results: {} and {}", res1, res2);
}
