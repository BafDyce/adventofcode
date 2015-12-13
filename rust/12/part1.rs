// adventofcode - day 12
// part 1

use std::io::prelude::*;
use std::fs::File;

fn main(){
    println!("Advent of Code - day 12 | part 1");

    // import data
    let data = import_data();

    let mut value = 0i32;
    let mut tmp_val = 0i32;
    let mut last: char = '\x00';
    let mut multiplier = 1;
    for ch in data.chars() {
        value += match ch {
            '0'...'9' => {
                if last == '-' {
                    multiplier = -1;
                }
                tmp_val = tmp_val * 10 + match ch.to_string().parse::<i32>() {
                    Ok(x) => x,
                    Err(e) => panic!("Help! {}", e),
                };

                0
            },
            _ if last.is_digit(10) => {
                let tmp = tmp_val * multiplier;
                tmp_val = 0;
                multiplier = 1;

                tmp
            }
            _ => 0,
        };
        last = ch;
    }

    println!("Value: {}", value);

}

// This function simply imports the data set from a file called input.txt
fn import_data() -> String {
    let mut file = match File::open("../../inputs/12.txt") {
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
