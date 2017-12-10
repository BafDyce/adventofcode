extern crate aocutils;
extern crate regex;

mod part1;
mod part2;

fn main() {
    let day: i32 = 10;

    //let input = aocutils::import(day, Some("puzzle1"));
    let input_string = aocutils::import(day, Some("puzzle1")).remove(0);
    let input_numbers = input_string.split(",").map(|x| x.parse::<usize>().unwrap()).collect();
    // let input: Vec<i32> = aocutils::import(day, Some("puzzle1")).iter().map(|x| x.parse::<i32>().unwrap()).collect();
    //let input: Vec<i32> = aocutils::import(day, Some("puzzle1")).remove(0).split('\t').map(|x| x.parse::<i32>().unwrap()).collect();

    let res1 = part1::solve(&input_numbers);
    let res2 = part2::solve(&input_string);

    println!("Results: {} and {}", res1, res2);
}
