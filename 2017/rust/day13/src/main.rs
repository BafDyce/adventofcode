extern crate aocutils;
extern crate regex;

mod part1;
mod part2;

use regex::Regex;

fn main() {
    let day: i32 = 13;

    let input = aocutils::import(day, Some("puzzle1"));

    let mut firewalls: Vec<(i32, i32)> = Vec::new();
    let re = Regex::new(
        r"(?P<pos>\d+): (?P<size>\d+)").unwrap();
    for line in input {
        let things = re.captures(&line).unwrap();
        let fw = (things["pos"].parse::<i32>().unwrap(), things["size"].parse::<i32>().unwrap());
        firewalls.push(fw);
    }


    let res1 = part1::solve(&firewalls);
    let res2 = part2::solve(&firewalls);

    println!("Results: {} and {}", res1, res2);
}
