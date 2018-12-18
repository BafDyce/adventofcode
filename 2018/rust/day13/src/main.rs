extern crate aoc_utils;

mod part1;
mod part2;

use aoc_utils::prelude::*;
use std::env;

const DAY: u32 = 13;
type InputType = Vec<Vec<char>>;

#[derive(Debug, Clone)]
pub enum Field<'a> {
    NoCart(char),
    Cart(Cart<'a>, char),
}

impl<'a> Default for Field<'a> {
    fn default() -> Self {
        Field::NoCart(' ')
    }
}

#[derive(Debug, Clone)]
pub struct Cart<'x> {
    walker: InfiniteGridWalker<'x, char>,
    last_turn: Direction2D,
}

impl<'x> Cart<'x> {
    pub fn new_with_particle(particle: Particle2D) -> Self {
        Cart {
            walker: InfiniteGridWalker::new_with_particle(particle),
            last_turn: Direction2D::Right,
        }
    }

    pub fn get_pos(&self) -> Location2D {
        self.walker.get_particle().get_pos().to_owned()
    }

    pub fn get_dir(&self) -> Direction2D {
        self.walker.get_particle().get_direction().to_owned()
    }

    pub fn turn(&mut self) {
        self.last_turn = match self.last_turn {
            Direction2D::Left => {
                // goes straight
                Direction2D::Up
            },
            Direction2D::Up => {
                self.walker.turn_right();
                Direction2D::Right
            },
            Direction2D::Right => {
                self.walker.turn_left();
                Direction2D::Left
            },
            Direction2D::Down => Direction2D::Down,
        }
    }
}

pub fn fancy_print(grid: &InfiniteGrid<Field>) -> bool {
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

        if loc_max.yy() - loc_min.yy() < 250 || loc_max.xx() - loc_min.xx() < 250 {
            for xx in loc_min.xx() ..= loc_max.xx() {
                print!("[{:3}]", xx);
                for yy in loc_min.yy() ..= loc_max.yy() {
                    match grid.get_value(&Location2D::new(xx, yy)) {
                        None => print!(" "),
                        Some(cc) => match cc {
                            Field::NoCart(cc) => print!("{}", cc),
                            Field::Cart(cart, _) => {
                                match cart.walker.get_particle().get_direction() {
                                    Direction2D::Up => print!("^"),
                                    Direction2D::Right => print!(">"),
                                    Direction2D::Down => print!("v"),
                                    Direction2D::Left => print!("<"),
                                }
                            }
                        }
                    }
                }
                println!()
            }

            true
        } else {
            false
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

    println!("results: {},{} and {},{}", res1.yy(), res1.xx(), res2.yy(), res2.xx());
}

fn parse_input(input_name: &str, verbose: bool) -> (InputType, PuzzleConfig) {
    let config = ImportConfig::new(2018, DAY, "../../_inputs/day{day}/");
    let (input, puzzle_config) = import_with_puzzle_config(&config, input_name).unwrap();
    if verbose {
        println!("raw input: {:?}", input);
    }

    // PARSE input
    let data: Vec<Vec<char>> = input.into_iter()
        .map(|line| {
            line.chars().collect()
        })
        .collect();

    if verbose {
        println!("input parsed: {:?}", data);
        println!("config: {:?}", puzzle_config);
    }
    (data, puzzle_config)
}
