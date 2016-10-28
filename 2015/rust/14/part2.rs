// adventofcode - day 14
// part 2

use std::io::prelude::*;
use std::fs::File;

struct Reindeer {
    name: String,           // name of this reindeer
    distance: u32,          // distance this reindeer travelled (so far)
    points: u32,            // points this reindeer scored (so far)
    speed: u32,             // how fast can this reindeer fly?
    flytime: u32,           // how long can this reindeer fly?
    resttime: u32,          // how long must the reindeer rest?
    flying_time_left: u32,
    resting_time_left: u32,
}

impl Reindeer {
    fn step(&mut self) -> u32 {
        // check whether the reindeer is in his running or resting period
        if self.flying_time_left > 0 {
            // run for a second
            self.distance += self.speed;
            self.flying_time_left -= 1;

            // check whether he has to rest now
            if self.flying_time_left == 0 {
                self.resting_time_left = self.resttime;
            }
        } else if self.resting_time_left > 0 {
            // rest for a second
            self.resting_time_left -= 1;

            // and check whether he can run next second again
            if self.resting_time_left == 0 {
                self.flying_time_left = self.flytime;
            }
        }

        // return the distance this reindeer travelled so far
        self.distance
    }

    fn award_point(&mut self) {
        // be happy for scoring a point :-)
        self.points += 1;
    }
}

fn main(){
    println!("Advent of Code - day 14 | part 2");

    // import data
    let data = import_data();

    let mut reindeers = Vec::new();

    // parse the input file and create a Vector of all participating reindeers
    for line in data.lines() {
        let values = line.split(" can fly ")
                    .flat_map(|s| s.split(" km/s for "))
                    .flat_map(|s| s.split(" seconds, but then must rest for "))
                    .flat_map(|s| s.split(" seconds."))
                    .map(|s| s.parse::<String>().unwrap())
                    .collect::<Vec<String>>();


        let speed = values[1].parse::<u32>().unwrap();
        let flytime = values[2].parse::<u32>().unwrap();
        let resttime = values[3].parse::<u32>().unwrap();

        reindeers.push( Reindeer{   name: values[0].clone(),
                                    distance: 0,
                                    points: 0,
                                    speed: speed,
                                    flytime: flytime,
                                    resttime: resttime,
                                    flying_time_left: flytime,
                                    resting_time_left: 0} );
    }

    // race lasts for 2503 seconds
    for _ in 0..2503 {

        // every reindeer makes a step
        // also keep track of the distance the current leader has travelled
        let mut max = 0u32;
        for ii in 0..reindeers.len() {
            let dist = reindeers[ii].step();
            if dist > max {
                max = dist;
            }
        }

        // award a point to every reindeer currently in the lead
        for ii in 0..reindeers.len() {
            if reindeers[ii].distance >= max {
                reindeers[ii].award_point();
            }
        }
    }

    // check who has won
    let mut winner = Reindeer{ name: "N/A".to_string(),
                                points: 0,
                                distance: 0,
                                speed: 0,
                                flytime: 0,
                                resttime: 0,
                                flying_time_left: 0,
                                resting_time_left: 0};

    for reindeer in reindeers {
        if reindeer.points > winner.points {
            winner = reindeer;
        }
    }

    println!("{} has won with {} points.", winner.name, winner.points);

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
