extern crate aocutils;

mod part1;
mod part2;

enum Packet {
    Group(i32),
    Garbage(i32),
}

fn main() {
    let day: i32 = 9;

    let input = aocutils::import(day, Some("puzzle1")).remove(0);

    let res1 = part1::solve(&input);
    let res2 = part2::solve(&input);

    println!("Results: {} and {}", res1, res2);
}
