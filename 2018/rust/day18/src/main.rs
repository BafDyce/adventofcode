extern crate aoc_utils;
#[macro_use]
extern crate ndarray;

mod part1;
mod part2;

use aoc_utils::prelude::*;
use ndarray::prelude::*;
use std::{env, fmt, str::FromStr};

const DAY: u32 = 18;
type InputType = Array2<Acre>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Acre {
    Open,
    Tree,
    Lumberyard,
    OutOfBounds,
}

#[derive(Debug)]
pub enum ParseError {
    EmptyString,
    InvalidChar,
}

impl FromStr for Acre {
    type Err = ParseError;

    fn from_str(ss: &str) -> Result<Self, Self::Err> {
        match ss.chars().next() {
            Some('.') => Ok(Acre::Open),
            Some('|') => Ok(Acre::Tree),
            Some('#') => Ok(Acre::Lumberyard),
            Some(_) => Err(ParseError::InvalidChar),
            None => Err(ParseError::EmptyString)
        }
    }
}

impl fmt::Display for Acre {
    fn fmt(&self, ff: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Acre::Open => write!(ff, "."),
            Acre::Tree => write!(ff, "|"),
            Acre::Lumberyard => write!(ff, "#"),
            Acre::OutOfBounds => write!(ff, " "),
        }
    }
}

pub fn update(area: InputType) -> InputType  {
    let new_acres: Vec<_> = area.windows((3, 3))
        .into_iter()
        .map(|fields| {
            match fields.get( (1, 1) ).unwrap() {
                Acre::Open if fields.iter().filter(|&&item| item == Acre::Tree).count() >= 3 => {
                    Acre::Tree
                }
                Acre::Tree if fields.iter().filter(|&&item| item == Acre::Lumberyard).count() >= 3 => {
                    Acre::Lumberyard
                }
                Acre::Lumberyard => {
                    if fields.iter().filter(|&&item| item == Acre::Lumberyard).count() >= 2
                    && fields.iter().filter(|&&item| item == Acre::Tree).count() >= 1 {
                        Acre::Lumberyard
                    } else {
                        Acre::Open
                    }
                }
                &other => other,
            }
        })
        .collect();

    let (xx, yy) = area.dim();
    let new_area = Array::from_vec(new_acres).into_shape( (xx - 2, yy - 2)).unwrap();


    let mut new = InputType::from_elem( area.dim(), Acre::OutOfBounds );
    new.slice_mut(s![1..-1, 1..-1]).assign(&new_area);

    new
}

pub fn print(area: &InputType) {
    for row in area.genrows() {
        for acre in row {
            print!("{}", acre);
        }
        println!();
    }
}

pub fn calc_score(area: &InputType) -> usize {
    let trees = area.iter().filter(|&&item| item == Acre::Tree).count();
    let lumberyards = area.iter().filter(|&&item| item == Acre::Lumberyard).count();

    trees * lumberyards
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

    let mm = input.len();
    let nn = input[0].len();

    let data: Vec<Acre> = input.into_iter()
        .map(|line| {
            (0 .. line.len()).map(|idx| (&line[idx .. idx+1]).parse().unwrap()).collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    if verbose {
        println!("Data parsed: {:?}", data);
    }

    match Array2::from_shape_vec((mm, nn), data) {
        Ok(area) => {
            let mut map = InputType::from_elem( ((mm + 2), (nn + 2)), Acre::OutOfBounds );
            map.slice_mut(s![1..-1, 1..-1]).assign(&area);
            if verbose {
                println!("input parsed: {:?}", map);
                println!("config: {:?}", puzzle_config);
            }
            (map, puzzle_config)
        },
        Err(ee) => panic!("Input cannot be parsed ({})", ee),
    }
}
