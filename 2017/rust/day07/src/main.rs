extern crate aocutils;
extern crate regex;

mod part1;
mod part2;

fn main() {
    let day: i32 = 7;
    let input = aocutils::import(day, Some("puzzle1"));

    let res1 = part1::solve(&input);
    // WARNING: part2 implementation is NOT useable for different inputs!!
    let res2 = part2::solve(&input);

    println!("Results: {} and {}", res1, res2);
}
