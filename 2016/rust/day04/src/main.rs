#[macro_use] extern crate lazy_static;

mod part1;
mod part2;

use aoc_utils::prelude::*;
use regex::Regex;
use std::{cmp::Ordering, collections::{HashMap, VecDeque}, env};

const DAY: u32 = 4;
type InputTypeSingle = Room;
type InputType = Vec<InputTypeSingle>;

#[derive(Debug, Clone, PartialEq)]
pub struct Room {
    name: String,
    sid: usize,
    checksum: String,
}

impl Room {
    pub fn new(name: String, sid: usize, checksum: String) -> Self {
        Room {
            name,
            sid,
            checksum,
        }
    }

    pub fn is_real(&self) -> bool {
        let mut chars = HashMap::new();

        for ch in self.name.chars() {
            if ch != '-' {
                let entry = chars.entry(ch).or_insert(0);
                *entry += 1;
            }
        }

        let mut chars: Vec<(char, usize)>  = chars.into_iter().collect();
        chars.sort_by(|aa, bb| {
            // first order by count, descending
            match bb.1.cmp(&aa.1) {
                // if equal, sort by character ascending
                Ordering::Equal => aa.0.cmp(&bb.0),
                other => other,
            }
        });

        let check = chars[..5].into_iter().map(|(ch, _)| ch).collect::<String>();
        check == self.checksum
    }

    pub fn get_sid(&self) -> usize {
        self.sid
    }

    pub fn decrypt_name(&self) -> String {
        let offset = (self.sid % 26) as u8;
        self.name.chars().map(|ch| {
            match ch {
                '-' => ' ',
                'a'..='z' => {
                    ((ch as u8 - ('a' as u8) + offset) % 26 + 'a' as u8) as char
                }
                other => other,
            }
        }).collect::<String>()
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
        // regex parsing stuff
        lazy_static! {
            // (?x)
            // (?P<name>xxx)
            static ref RE: Regex = Regex::new(
                r"(?x)
                (?P<name>.+)
                -
                (?P<sid>[0-9]{3})
                \[
                (?P<checksum>[a-z]{5})
                \]
                "
            ).unwrap();
        }

        let caps = RE.captures(&line).unwrap();
        // let thingy = &caps["thingy"];
        // let xx = caps["xx"].chars().next().unwrap();
        Room::new(
            caps["name"].to_string(),
            caps["sid"].parse().unwrap(),
            caps["checksum"].to_string(),
        )
    })
    .collect();

    if verbose {
        println!("input parsed: {:?}", data);
        println!("config: {:?}", puzzle_config);
    }
    (data, puzzle_config)
}
