extern crate aocutils;
mod part1;
mod part2;

fn main() {
    let day: i32 = 2;

    let input = aocutils::import(day, Some("puzzle1"));

    let res1 = part1::solve(&input);
    let res2 = part2::solve(&input);

    println!("Results: {} and {}", res1, res2);
}
