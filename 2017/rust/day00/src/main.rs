extern crate aocutils;
mod part1;
mod part2;

fn main() {
    let day: i32 = 0;

    let input = aocutils::import(day, Some("puzzle1"));
    //let input = aocutils::import(day, Some("puzzle1")).remove(0);

    let r1 = part1::solve(&input);
    let r2 = part2::solve(&input);

    println!("Results: {} and {}", r1, r2);
}
