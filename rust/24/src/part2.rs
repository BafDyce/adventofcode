// adventofcode - day 24
// part 2
extern crate rand;

use std::io::prelude::*;
use std::fs::File;
use rand::distributions::{IndependentSample, Range};

pub fn run(){
    println!("Advent of Code - day 24 | part 2");

    // import data
    let data = import_data();
    let packages = parse_data(data);

    let sumcheck = sum(&packages) / 4;


    let mut smallest_size: usize = !0;
    let mut smallest_qe: i64 = !0; // quantum entanglement
    loop {
        let arrangement = arrange(&packages, sumcheck);
        //println!("{:?}", arrangement);

        let smallest = get_smallest_group_from(arrangement);

        let size = smallest.len();
        let qe = compute_quantum_entanglement(&smallest);
        if size < smallest_size {
            smallest_size = size;
            smallest_qe = qe;
            println!("Best so far: size: {}, qe: {}",
                        smallest_size, smallest_qe);
        } else if size == smallest_size && qe < smallest_qe {
            smallest_qe = qe;
            println!("Best so far: size: {}, qe: {}",
                        smallest_size, smallest_qe);
        }
    }

}

fn compute_quantum_entanglement(src: &Vec<i64>) -> i64 {
    let mut qe = 1;
    for elem in src {
        qe *= *elem;
    }

    qe
}

fn get_smallest_group_from(src: Vec<Vec<i64>>) -> Vec<i64> {

    if src[0].len() < src[1].len() {
        if src[0].len() < src[2].len() {
            if src[0].len() < src[3].len() {
                src[0].clone()
            } else {
                src[3].clone()
            }
        } else {
            if src[2].len() < src[3].len() {
                src[2].clone()
            } else {
                if src[0].len() < src[3].len() {
                    src[0].clone()
                } else {
                    src[3].clone()
                }
            }
        }
    } else {
        // 1 < 0
        if src[1].len() < src[2].len() {
            if src[1].len() < src[3].len() {
                src[1].clone()
            } else {
                src[3].clone()
            }
        } else {
            // 2 < 1 , 1 < 0
            if src[2].len() < src[3].len() {
                src[2].clone()
            } else {
                src[3].clone()
            }
        }
    }
}

fn arrange(src: &Vec<i64>, check: i64) -> Vec<Vec<i64>> {
    let mut arrangement = Vec::new();

    let range = Range::new(0, 4);
    let mut rng = rand::thread_rng();

    loop {
        arrangement.clear();

        arrangement.push( Vec::<i64>::new() );
        arrangement.push( Vec::<i64>::new() );
        arrangement.push( Vec::<i64>::new() );
        arrangement.push( Vec::<i64>::new() );

        for x in src {
            let dst = range.ind_sample(&mut rng);
            arrangement[ dst ].push( x.clone() );
        }

        if sum(&arrangement[0]) == check && sum(&arrangement[1]) == check
                && sum(&arrangement[2]) == check {
            // if first, second and third have correct value, the fourth one
            // also has a correct value
            break;
        }
    }

    arrangement
}

fn sum(data: &Vec<i64>) -> i64 {
    let mut sum = 0i64;

    for elem in data {
        sum += *elem;
    }

    sum
}

fn parse_data(data: String) -> Vec<i64> {

    let mut packages = Vec::new();
    for line in data.lines() {
        packages.push( line.parse::<i64>().unwrap() );
    }

    packages
}

// This function simply imports the data set from a file called input.txt
fn import_data() -> String {
    let mut file = match File::open("../../inputs/24.txt") {
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
