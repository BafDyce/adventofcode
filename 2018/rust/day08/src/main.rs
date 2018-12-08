extern crate aoc_utils;

mod part1;
mod part2;

use aoc_utils::prelude::*;
use std::env;

const DAY: u32 = 8;
type InputType = Node;


#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    pub fn new(data: &[usize]) -> (Self, usize) {
        //println!("Creating Node from {:?}", data);
        let mut node = Node {
            children: Vec::with_capacity(data[0]),
            metadata: Vec::with_capacity(data[1]),
        };

        let mut next = 2;
        for __ in 0 .. data[0] {
            let (child, new_next) = Node::new(&data[next.. data.len() - data[1]]);
            //println!("{:?}, {} @ {}", child, data[new_next], new_next);
            node.children.push(child);
            next += new_next
        }

        for ii in 0 .. data[1] {
            node.metadata.push(data[next + ii])
        }

        (node, next + data[1])
    }

    // PART 1 SOLUTION!!
    pub fn metasum(&self) -> usize {
        let xx = self.children.iter().map(|child| child.metasum()).sum::<usize>();
        xx + self.metadata.iter().sum::<usize>()
    }

    // PART 2 SOLUTION!!
    pub fn metasum2(&self) -> usize {
        if self.children.is_empty() {
            self.metadata.iter().sum::<usize>()
        } else {
            let mut sum = 0;
            for reference in &self.metadata {
                if *reference == 0 || *reference > self.children.len() {
                    continue
                }
                sum += self.children[reference-1].metasum2()
            }

            sum
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

    // PARSE input
    let data: Vec<_> = input[0].split(" ").map(|num| {
        num.parse::<usize>().unwrap()
    })
    .collect();

    let (result, _) = Node::new(&data);

    if verbose {
        println!("input parsed: {:?}", data);
        println!("config: {:?}", puzzle_config);
        println!("Nodes: {:?}", result);
    }
    (result, puzzle_config)
}
