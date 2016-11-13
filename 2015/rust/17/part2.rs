// adventofcode - day 17
// part 2

use std::io::prelude::*;
use std::fs::File;
use std::str::FromStr;
use std::i32::MAX;

fn main(){
    println!("Advent of Code - day 17 | part 2");

    // import data
    let data = import_data();

    let mut containers = Vec::new();
    for line in data.lines() {
        containers.push( i32::from_str(line).unwrap() );
    }

    containers.sort();
    containers.reverse();

    let store = 150;
    let (combi, times) = count_min_combination(store, &containers[0..]);

    println!("Minimum combi: {} ({} times)", combi, times);

}

// recursive approach to find the number of combinations
// NOTE: This assumes a descending sorted array of containers
fn count_min_combination(store: i32, containers: &[i32]) -> (i32, i32) {

    if store == 0 {
        // if there's nothing left to fill we found a valid combination
        return (1, 1);
    } else if store < 0 {
        // if we are below 0 we did something wrong. so we return something
        // "invalid"
        return (std::i32::MAX, 0);
    }

    let mut min_combi = std::i32::MAX;
    let mut min_combi_times = 0;
    for ii in 0..containers.len() {
        if containers[ii] > store {
            // if the container is too large for our remaining eggnag, we'll
            // skip it
            continue;
        }

        // after "adding" the container to our list (we dont keep track of our
        // combination-list because we just want to have the lowest amount of
        // containers required; and how many possibilities there are for this)
        // we check how many remaining combinations are left for the other
        // containers (and the reduced amount to fill up)
        let (mut combi, times) = count_min_combination(store - containers[ii],
                                                &containers[ii+1..]);

        // if we found combination we have to add one because of the container
        // we just added.
        // std::i32::MAX is only returned in case of an error or if no
        // combination could be found
        if combi != std::i32::MAX {
            combi += 1;
        }

        if combi == min_combi {
            // if we already found this combination"score" we need to add the
            // ones we just found
            min_combi_times += times;
        } else if combi < min_combi {
            // otherwise, if we found a new lowest combination score we'll take
            // this one
            min_combi = combi;
            min_combi_times = times;
        }

    }

    // in the end, return everything
    (min_combi, min_combi_times)
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
