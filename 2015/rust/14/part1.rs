// adventofcode - day 14
// part 1

use std::io::prelude::*;
use std::fs::File;

fn main(){
    println!("Advent of Code - day 14 | part 1");

    // import data
    let data = import_data();

    // a straightforward O(n) solution
    // scans the input, calculates the distance this reindeer will fly and saves
    // the information about the best reindeer.

    let mut winner = "Santa".to_string();
    let mut greatest_distance = 0i32;
    for line in data.lines() {
        let values = line.split(" can fly ")
                    .flat_map(|s| s.split(" km/s for "))
                    .flat_map(|s| s.split(" seconds, but then must rest for "))
                    .flat_map(|s| s.split(" seconds."))
                    .map(|s| s.parse::<String>().unwrap())
                    .collect::<Vec<String>>();

        let speed = values[1].parse::<i32>().unwrap();
        let flytime = values[2].parse::<i32>().unwrap();
        let resttime = values[3].parse::<i32>().unwrap();

        // reindeer can run & rest that many times completely
        let periods = 2503 / (flytime + resttime);
        // after running & resting this many times he got some time left
        let last_part = 2503 % (flytime + resttime);

        // check how much of this time he's able to run
        let last_flypart = if last_part > flytime {
            let diff = last_part - flytime;
            last_part - diff
        } else {
            last_part
        };

        // eventually, calculate the distance
        let distance = periods * flytime * speed + last_flypart * speed;

        // and check whether he's the best so far
        if distance > greatest_distance {
            greatest_distance = distance;
            winner = values[0].clone();
        }
    }

    println!("The winner is {} with {} km.", winner, greatest_distance);

}

// This function simply imports the data set from a file called input.txt
fn import_data() -> String {
    let mut file = match File::open("../../inputs/14.txt") {
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
