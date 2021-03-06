// adventofcode - day 10
// part 1
use std::io::prelude::*;
use std::fs::File;

pub fn main(){
    println!("Advent of Code - day 10 | part 1");

    // import data
    let mut sequence = import_data();
    // remove trailing new line character
    sequence.pop();
    println!("Input: {}", sequence);

    for _ in 1..41 {
        sequence = look_and_say(sequence);
        //println!("Round {}, length: {}", ii, sequence.len());
    }

    println!("Length after 40 iterations: {}", sequence.len());
}

fn look_and_say(input: String) -> String {
    // Conway's constant is 1.3035, however we use a factor of 1.4 to avoid
    // re-allocation
    let mut output = String::with_capacity((input.len() as f64 * 1.4) as usize);

    let mut last_char = '\x00';
    let mut char_count = 1u32;
    for ch in input.chars() {
        if ch == last_char {
            char_count += 1;
        } else {
            if last_char != '\x00' {
                let char_count_str = char_count.to_string();
                output.push_str( &char_count_str[0..char_count_str.len()] );
                output.push( last_char );
                char_count = 1;
            }
            last_char = ch;
        }
    }

    // also store the last character(s)
    let char_count_str = char_count.to_string();
    output.push_str( &char_count_str[0..char_count_str.len()] );
    output.push( last_char );

    output
}

// This function simply imports the data set from a file called input.txt
fn import_data() -> String {
    let mut file = match File::open("../../inputs/10.txt") {
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
