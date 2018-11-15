extern crate aocutils;
extern crate regex;

mod part1;
mod part2;

fn main() {
    let day: i32 = 15;

    let input = aocutils::import(day, Some("puzzle1"));
    assert_eq!(input.len(), 2);

    let a = get_value(&input[0]);
    let b = get_value(&input[1]);

    let starts = (a, b);

    let res1 = part1::solve(&starts);
    let res2 = part2::solve(&starts);

    println!("Results: {} and {}", res1, res2);
}

fn get_value(line: &String) -> u64 {
    let elements = line.split_whitespace();
    elements.last().unwrap().parse::<u64>().unwrap()
}
