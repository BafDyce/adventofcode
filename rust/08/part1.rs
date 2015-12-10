// adventofcode - day 8
// part 1

use std::io::prelude::*;
use std::fs::File;

fn main(){
    println!("Advent of Code - day 8 | part 1");

    // import data
    let data = import_data();

    // logic is, that we don't compute the length of either of the strings,
    // however, we simply calculate differences on the fly
    // i.e. if we discover a '\' we know that this is one character "too much"
    // -> we simply increment our counter ;)
    let mut total_char_diff = 0u32;
    for line in data.lines() {
        // substract first and last "
        let mut line_char_diff = 2u32;

        let mut skip = 0u8;
        let mut last_char = '\x00';
        for ch in line.chars(){
            match ch {
                '\\' => {
                    if skip > 0 {
                        skip -= 1;
                    } else {
                        line_char_diff += 1;
                        skip = 1;
                    }
                },
                // we only consider \x++ not \xx+
                'x' if skip == 1 && last_char == '\\' => {
                    line_char_diff += 2;
                    skip = 2;
                },
                _ => {
                    if skip > 0 {
                        skip -= 1;
                    }
                },
            }
            last_char = ch;
        }

        total_char_diff += line_char_diff;
    }

    println!("Total difference: {} chars", total_char_diff);
}

// This function simply imports the data set from a file called input.txt
fn import_data() -> String {
    let mut file = match File::open("input.txt") {
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
