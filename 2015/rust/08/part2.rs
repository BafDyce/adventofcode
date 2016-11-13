// adventofcode - day 8
// part 2

use std::io::prelude::*;
use std::fs::File;

fn main(){
    println!("Advent of Code - day 8 | part 2");

    // import data
    let data = import_data();

    // logic is, that we don't compute the length of either of the strings,
    // however, we simply calculate differences on the fly
    // this is easer than in part 1 because we know that we only have to escape
    // '\' and '"' characters -> everytime we see such a character, we increment
    // our counter :P
    let mut total_char_diff = 0u32;
    for line in data.lines() {
        let backslashes: Vec<&str> = line.matches('\\').collect();
        let doublequotes: Vec<&str> = line.matches('\"').collect();

        // '2' because we need to append double quotes at the beginning and the
        // end of every string
        total_char_diff += 2
                            + backslashes.len() as u32
                            + doublequotes.len() as u32;
    }

    println!("Total difference: {} chars", total_char_diff);
}

// This function simply imports the data set from a file called input.txt
fn import_data() -> String {
    let mut file = match File::open("../../inputs/08.txt") {
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
