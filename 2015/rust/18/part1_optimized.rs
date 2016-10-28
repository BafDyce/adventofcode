// adventofcode - day 18
// part 1 - optimized
//
// the optimized variants use two grids with a ring of lights (set to off)
// around the real grid. This brings two optimizations:
// 1) the ring around the grid:
//      no check which iterators are needed is required -> a little bit of time
//      saved.
// 2) we use 2 grids and swap between them. As a result, we don't need to clone
//      the grid over and over again (a lot of time saved)
//
// In total (based on 10-20 runs each) the execution time (both compiled with
// -O) was reduced from 15-16ms to 8-9ms (on my machine - 4 GHz i7, NO ssd)

use std::io::prelude::*;
use std::fs::File;

fn main(){
    println!("Advent of Code - day 18 | part 1 | optimized");

    // import data
    let data = import_data();
    let mut grid = parse_data_into_grid( data );
    let mut grid2 = grid.clone();

    let iterations = 100/2;
    for _ in 0..iterations {
        update_field(&grid, &mut grid2);
        update_field(&grid2, &mut grid);
    }

    let lights = count_lights(&grid);
    println!("After {} iterations, {} lights are lightening!", iterations * 2,
                                                                lights);
}

fn count_lights(grid: &Vec<Vec<bool>>) -> i32 {

    let mut ctr = 0;
    for ii in 1..grid.len() - 1 {
        for jj in 1..grid[ii].len() - 1 {
            if grid[ii][jj] {
                ctr += 1;
            }
        }
    }

    ctr
}

fn update_field(current: &Vec<Vec<bool>>, new: &mut Vec<Vec<bool>>) {

    for ii in 1..current.len() - 1 {
        for jj in 1..current[ii].len() - 1 {
            new[ii][jj] = on_or_off(&current, ii, jj);
        }
    }
}

fn on_or_off(grid: &Vec<Vec<bool>>, x: usize, y: usize) -> bool {

    let mut count = 0;

    // count how many lights are on in that area
    for xx in x-1..x+2 {
        for yy in y-1..y+2 {
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

    let mut grid = Vec::with_capacity(lines + 2);

    let mut buffer = Vec::with_capacity( lines + 2 );
    for _ in 0..lines + 2 {
        buffer.push( false );
    }
    grid.push( buffer.clone() );

    for line in data.lines() {
        if line.len() != lines {
            panic!("Invalid file format!");
        }

        let mut row = Vec::with_capacity(lines);
        row.push(false);
        for ch in line.chars() {

            row.push( match ch {
                '#' => true,
                '.' => false,
                _   => panic!("Invalid file format!"),
            } );
        }
        row.push(false);

        grid.push(row);
    }

    grid.push(buffer);
    grid
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<bool>>) {

    for ii in 1..grid.len() - 1 {
        for jj in 1..grid[ii].len() - 1 {
            print!("{}", if grid[ii][jj] { '#' } else { '.' });
        }
        println!("");
    }
    println!("");
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
