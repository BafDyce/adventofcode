extern crate aocutils;
mod part1;
mod part2;

fn main() {
    let day: i32 = 6;

    let input: Vec<i32> = aocutils::import(day, Some("puzzle1"))
                                    .remove(0)
                                    .split('\t')
                                    .map(|x| x.parse::<i32>().unwrap())
                                    .collect();

    let res1 = part1::solve(&input);
    let res2 = part2::solve(&input);

    println!("Results: {} and {}", res1, res2);
}
