extern crate aocutils;
mod part1;
mod part2;

fn main() {
    let day: i32 = 2;

    let input: i32 = aocutils::import(day, Some("puzzle1")).remove(0).parse::<i32>().unwrap();

    let res1 = part1::solve(input);
    let res2 = part2::solve(input);

    println!("Results: {} and {}", res1, res2);
}
