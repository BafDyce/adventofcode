// ================================================================================================
// 2d array
// ================================================================================================
// Read input into 2 dimensional array of things (i.e. `char`s)
type InputType = Vec<Vec<char>>;
let data: InputType = input.into_iter()
    .map(|line| {
        line.chars().collect()
    })
    .collect()

// ================================================================================================
// H A S H I N G
// ================================================================================================

// if everything to hash is available
let hash = Md5::digest(b"Entire message");
let hash = Sha1::digest(b"Entire message");

// if lot of stuff must be hashed (which is not available as single array/string:)

let mut hasher = Md5::new();
hasher.input(b"hello world"); // can be repeated
let hash = hasher.result();

// create hex digest
let hex = format!("{:x}", hash);

// ================================================================================================
// I N F I N I T E   G R I D S
// ================================================================================================
use aoc_utils::grid2d::*;

use std::sync::{Mutex, Arc};
use std::cell::RefCell;

let mut grid: InfiniteGrid<i32> = InfiniteGrid::new();
let grid_container = RefCell::new(Mutex::new(&mut grid as &mut Grid<i32>));

let mut walker: InfiniteGridWalker<i32> = InfiniteGridWalker::new();
let mut walker2: InfiniteGridWalker<i32> = InfiniteGridWalker::new();

walker.assign_grid(&grid_container);
walker2.assign_grid(&grid_container);
walker2.turn_reverse();

print_grid(&grid_container);
println!("walker 1: {:?}\nwalker 2: {:?}", walker, walker2);

let increment = |old: &i32| -> i32 {old + 1};

walker.operate(increment);
walker2.operate(increment);
walker.step_forward();
walker2.step_forward();

print_grid(&grid_container);
println!("walker 1: {:?}\nwalker 2: {:?}", walker, walker2);

walker.operate(increment);
walker2.operate(increment);
walker.step_forward();
walker2.step_forward();

print_grid(&grid_container);
println!("walker 1: {:?}\nwalker 2: {:?}", walker, walker2);

// ================================================================================================
use std::str::FromStr;

enum ParseError {
    Invalid,
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(ss: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+)\s+(\d+)\s+(\d+)\s+(\d+)").unwrap();
        }

        match RE.captures(ss) {
            Some(caps) => {
                Ok(Instruction {
                    opcode: Opcode::Unknown(caps[1].parse()?),
                    aa: caps[2].parse()?,
                    bb: caps[3].parse()?,
                    cc: caps[4].parse()?,
                })
            }
            None => Err(ParseError::InvalidInstructionFormat)
        }
    }
}
