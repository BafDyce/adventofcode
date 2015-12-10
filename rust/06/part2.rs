// adventofcode - day 6
// part 2

use std::io::prelude::*;
use std::fs::File;

enum Operation {
    TurnOn,
    TurnOff,
    Toggle
}

fn main(){
    println!("Advent of Code - day 6 | part 2");

    // import data
    let instructions = import_data();

    let mut lights: [[u32; 1000]; 1000] = [[0u32; 1000]; 1000];

// https://www.reddit.com/r/adventofcode/comments/3vmltn/day_6_solutions/cxptu4a
    for ref mut line in instructions.lines() {
        let operation = if eat(line, "turn on ") {
            Operation::TurnOn
        } else if eat(line, "turn off ") {
            Operation::TurnOff
        } else if eat(line, "toggle ") {
            Operation::Toggle
        } else {
            panic!("Invalid instruction: '{}'", line)
        };

        // some magic happening here
        let coords = line.split(',')
            .flat_map(|s| s.split(" through "))
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        match operation {
            Operation::TurnOn =>
                turn_on(&mut lights, coords[0], coords[1],
                                    coords[2], coords[3]),
            Operation::TurnOff =>
                turn_off(&mut lights, coords[0], coords[1],
                                    coords[2], coords[3]),
            Operation::Toggle =>
                toggle(&mut lights, coords[0], coords[1], coords[2], coords[3]),
        }
    }

    println!("Total brightness: {}", count_lightning(lights) );

}

fn turn_on(lights: &mut [[u32; 1000]; 1000], x1: usize, y1: usize,
                                            x2: usize, y2: usize) {
    for ii in x1..x2+1 {
        for jj in y1..y2+1 {
            lights[ii][jj] += 1;
        }
    }
}

fn turn_off(lights: &mut [[u32; 1000]; 1000], x1: usize, y1: usize,
                                            x2: usize, y2: usize) {
    for ii in x1..x2+1 {
        for jj in y1..y2+1 {
            if lights[ii][jj] > 0 {
                lights[ii][jj] -= 1;
            }
        }
    }
}

fn toggle(lights: &mut [[u32; 1000]; 1000], x1: usize, y1: usize,
                                            x2: usize, y2: usize) {
    for ii in x1..x2+1 {
        for jj in y1..y2+1 {
            lights[ii][jj] += 2;
        }
    }
}

fn count_lightning(lights: [[u32; 1000]; 1000]) -> u32 {

    let mut counter = 0u32;
    for ii in 0..1000 {
        for jj in 0..1000 {
            counter += lights[ii][jj];
        }
    }

    counter
}

// copied from
// https://www.reddit.com/r/adventofcode/comments/3vmltn/day_6_solutions/cxptu4a
fn eat(s: &mut &str, expect: &str) -> bool {
    if s.starts_with(expect) {
        *s = &s[expect.len()..];
        true
    } else {
        false
    }
}

// This function simply imports the data set from a file called input.txt
fn import_data() -> String {
    let mut file = match File::open("../../inputs/06.txt") {
        Ok(f) => f,
        Err(e) => panic!("file error: {}", e),
    };

    let mut data = String::new();
    match file.read_to_string(&mut data){
        Ok(_) => {},
        Err(e) => panic!("file error: {}", e),
    };

	data
}
