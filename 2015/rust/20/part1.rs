// adventofcode - day 20
// part 1

use std::io::prelude::*;
use std::fs::File;

fn main(){
    println!("Advent of Code - day 20 | part 1");

    // import data
    let data = import_data();
    let goal = data.parse::<i64>().unwrap();

    println!("Looking for a house which gets {} presents.", goal);

    // let's skip the first "few" houses for speeding things up a bit
    // x / 48 actually is just derived from emperically observed, no actual
    // math invloved
    for house in (goal / 48).. {
        let mut presents = house * 10;
        // we can skip elves with numbers higher than half of our house number
        // (no divider possible)
        for elf in 1 .. house/2 + 1 {
            if house % elf == 0 {
                presents += elf * 10;
            }
        }

        if presents >= goal {
            print!("House #{} is the first one to get at least", house);
            println!(" {} presents ({})", goal, presents);
            break;
        }
    }
}

// This function simply imports the data set from a file called input.txt
fn import_data() -> String {
    let mut file = match File::open("../../inputs/20.txt") {
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
