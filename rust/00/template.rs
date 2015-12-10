// adventofcode - day 0
// part 0

use std::io::prelude::*;
use std::fs::File;

fn main(){
    println!("Advent of Code - day 0 | part 0");

    // import data
    let data = import_data();

}

// This function simply imports the data set from a file called input.txt
fn import_data() -> String {
    let mut file = match File::open("../../inputs/00.txt") {
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
