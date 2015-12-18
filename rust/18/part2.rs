// adventofcode - day 18
// part 2

use std::io::prelude::*;
use std::fs::File;

fn main(){
    println!("Advent of Code - day 18 | part 2");

    // import data
    let data = import_data();
    let mut grid = parse_data_into_grid( data );

    let iterations = 100;
    for _ in 0..iterations {
        update_field(&mut grid);
    }

    let lights = count_lights(&grid);
    println!("After {} iterations, {} lights are lightening!", iterations,
                                                                lights);
}

fn count_lights(grid: &Vec<Vec<bool>>) -> i32 {

    let mut ctr = 0;
    for ii in 0..grid.len() {
        for jj in 0..grid[ii].len(){
            if grid[ii][jj] {
                ctr += 1;
            }
        }
    }

    ctr
}

fn update_field(grid: &mut Vec<Vec<bool>>) {
    // save the state of the grid before we begin to modify it
    let before = grid.clone();

    for ii in 0..grid.len() {
        for jj in 0..grid[ii].len(){
            grid[ii][jj] = on_or_off(&before, ii, jj);
        }
    }

    // after updating everything we need to "reset" our corners to `on`
    let size = grid.len() - 1;
    grid[0][0] = true;
    grid[0][ size ] = true;
    grid[ size ][0] = true;
    grid[ size ][ size ] = true;
}

fn on_or_off(grid: &Vec<Vec<bool>>, x: usize, y: usize) -> bool {

    let mut count = 0;

    // generate the iterator for our x-axis
    // the same procedure applies to the iterator for the y-axis
    let xiter = match x {
        // we're at the left border: we only need the values 0 and 1
        // NOTE: `0..2` creates an iterator containing `0` and `1` (NOT `2`)
        0 => 0..2,
        // we're at the right border: we only need the last two columns
        x @ _ if x == grid.len() -1 => x-1..x+1,
        x @ _   => x-1..x+2,
    };

    let yiter = match y {
        0 => 0..2,
        y @ _ if y == grid.len() -1 => y-1..y+1,
        y @ _   => y-1..y+2,
    };

    // then, we just count how many lights are on in that area
    for xx in xiter {
        for yy in yiter.clone() {
            if grid[xx][yy] {
                count += 1;
            }
        }
    }

    // if we counted ourself towards the counter, we need to decrement it again
    if grid[x][y] {
        count -= 1;
    }

    // then just return whether this lights should be on or off
    match count {
        2 | 3 if grid[x][y] => true,
        _ if grid[x][y] => false,

        3 if ! grid[x][y] => true,
        _   => false,
    }
}

fn parse_data_into_grid(data: String) -> Vec<Vec<bool>> {

    let lines = match data.find('\n') {
        Some(x) => x,
        None    => panic!("Invalid file format!"),
    };

    let mut grid = Vec::with_capacity(lines);
    for line in data.lines() {
        if line.len() != lines {
            panic!("Invalid file format!");
        }

        let mut row = Vec::with_capacity(lines);
        for ch in line.chars() {

            row.push( match ch {
                '#' => true,
                '.' => false,
                _   => panic!("Invalid file format!"),
            } );
        }

        grid.push(row);
    }

    // set the corners to on, regardless of our input
    let size = grid.len() - 1;
    grid[0][0] = true;
    grid[0][ size ] = true;
    grid[ size ][0] = true;
    grid[ size ][ size ] = true;

    grid
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<bool>>) {

    for ii in 0..grid.len() {
        for jj in 0..grid[ii].len() {
            print!("{}", if grid[ii][jj] { '#' } else { '.' });
        }
        println!("");
    }
}

// This function simply imports the data set from a file called input.txt
fn import_data() -> String {
    let mut file = match File::open("../../inputs/18.txt") {
        Ok(f) => f,
        Err(e) => panic!("file error: {}", e),
    };

    let mut data = String::new();
    match file.read_to_string(&mut data){
        Ok(_) => {},
        Err(e) => panic!("file error: {}", e),
    };

    data.pop();
    data
}
