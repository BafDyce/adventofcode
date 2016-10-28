// adventofcode - day 17
// part 1

use std::io::prelude::*;
use std::fs::File;
use std::str::FromStr;

fn main(){
    println!("Advent of Code - day 17 | part 1");

    // import data
    let data = import_data();

    let mut containers = Vec::new();
    for line in data.lines() {
        containers.push( i32::from_str(line).unwrap() );
    }

    containers.sort();
    containers.reverse();

    let store = 150;
    let combinations = count_combinations(store, &containers[0..]);

    println!("There are {} combinations!", combinations);

}

// recursive approach to find the number of combinations
// NOTE: This assumes a descending sorted array of containers
fn count_combinations(store: i32, containers: &[i32]) -> i32 {

    if store == 0 {
        // if there's nothing left to fill we found a valid combination
        return 1;
    } else if store < 0 {
        // if we are below 0 we did something wrong. so, we'll just return 0
        return 0;
    }

    let mut combinations_found = 0;
    for ii in 0..containers.len() {
        if containers[ii] > store {
            // if the container is too large for our remaining eggnag, we'll
            // skip it
            continue;
        }

        // after "adding" the container to our list (we dont keep track of our
        // combination-list because we just want to have the number of
        // combinations) we check how many remaining combinations are left for
        // the other containers (and the reduced amount to fill up)
        combinations_found += count_combinations(store - containers[ii],
                                                &containers[ii+1..]);

    }

    // return the number of combinations we found so far
    combinations_found
}

// This function simply imports the data set from a file called input.txt
fn import_data() -> String {
    let mut file = match File::open("../../inputs/17.txt") {
        Ok(f) => f,
        Err(e) => panic!("file error: {}", e),
    };

    let mut data = String::new();
    match file.read_to_string(&mut data){
        Ok(_) => {},
        Err(e) => panic!("file error: {}", e),
    };

    // remove trailing '\n'
    data.pop();
    data
}
