extern crate aoc_utils;
#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate md5;
extern crate sha1;

mod part1;
mod part2;

use aoc_utils::prelude::*;
use aoc_utils::grid2d::*;
use regex::Regex;
use std::{collections::{HashMap, VecDeque}, env};

const DAY: u32 = 17;
type InputTypeSingle = Location2D;
type InputType = InfiniteGrid<Element>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Element {
    Sand,
    Clay,
    WaterStill,
    WaterFlowing,
    Spring,
}

impl Default for Element {
    fn default() -> Element {
        Element::Sand
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
    let (res1, tmp) = part1::solve(&input, &puzzle_config);
    let res2 = part2::solve(tmp, &puzzle_config);

    println!("results: {} and {}", res1, res2);
}

fn parse_input(input_name: &str, verbose: bool) -> (InputType, PuzzleConfig) {
    let config = ImportConfig::new(2018, DAY, "../../_inputs/day{day}/");
    let (input, puzzle_config) = import_with_puzzle_config(&config, input_name).unwrap();
    if verbose {
        println!("raw input: {:?}", input);
    }

    // PARSE input
    let data: Vec<Location2D> = input.into_iter().map(|line| {
        // Parsing logic
        // regex parsing stuff
        lazy_static! {
            // (?x)
            // (?P<name>xxx)
            static ref RE: Regex = Regex::new(
                r"([x|y])=(\d+), ([x|y])=(\d+)\.\.(\d+)"
            ).unwrap();
        }

        let caps = RE.captures(&line).unwrap();
        let xxs = if &caps[1] == "x" {
            vec![caps[2].parse::<usize>().unwrap()]
        } else if &caps[3] == "x" {
            let xx_start = caps[4].parse::<usize>().unwrap();
            let xx_end = caps[5].parse::<usize>().unwrap();
            (xx_start ..= xx_end).collect()
        } else {
            panic!("No x found in {}", line);
        };

        let yys = if &caps[1] == "y" {
            vec![caps[2].parse::<usize>().unwrap()]
        } else if &caps[3] == "y" {
            let yy_start = caps[4].parse::<usize>().unwrap();
            let yy_end = caps[5].parse::<usize>().unwrap();
            (yy_start ..= yy_end).collect()
        } else {
            panic!("No y found in {}", line);
        };

        let mut clays = Vec::new();
        for xx in xxs {
            for yy in &yys {
                clays.push(Location2D::new(xx as isize, *yy as isize));
            }
        }

        clays
    })
    .flatten()
    .collect();

    let mut grid: InfiniteGrid<Element> = InfiniteGrid::new();
    grid.set_value(&Location2D::new(500, 0), Element::Spring);
    for clay in data {
        grid.set_value(&clay, Element::Clay);
    }

    if verbose {
        //println!("input parsed: {:?}", data);
        fancy_print(&grid);
        println!("config: {:?}", puzzle_config);
    }
    (grid, puzzle_config)
}

pub fn fancy_print(grid: &InfiniteGrid<Element>) -> bool {
    if grid.iter().count() == 0 {
        //println!("Grid is empty!");
        false
    } else {
        let [
            _,
            loc_max,
            loc_min,
            _
        ] = grid.get_boundaries();

        //if loc_max.xx() - loc_min.yy() < 250 || loc_max.xx() - loc_min.xx() < 250 {
            for yy in loc_min.yy() ..= loc_max.yy() {
                print!("[{:4}]", yy);
                for xx in loc_min.xx() ..= loc_max.xx() {
                    match grid.get_value(&Location2D::new(xx, yy)) {
                        None => print!(" "),
                        Some(elem) => match elem {
                            Element::Sand => print!("."),
                            Element::Clay => print!("#"),
                            Element::WaterStill => print!("~"),
                            Element::WaterFlowing => print!("|"),
                            Element::Spring => print!("+"),
                        }
                    }
                }
                println!()
            }

            true
        //} else {
        //    false
        //}
    }

}
