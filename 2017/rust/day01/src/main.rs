extern crate aocutils;
mod part1;
mod part2;

fn main() {
    let day: i32 = 1;

    let input = aocutils::import(day, Some("puzzle1"));

    let r1 = part1::solve(input.clone());
    let r2 = part2::solve(input);

    println!("Results: {} and {}", r1, r2);
}
