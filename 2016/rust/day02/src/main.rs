mod part1;
mod part2;

fn main() {
    let day: i32 = 2;
    let input = aocutils::import(day, Some("puzzle1"));

    let r1 = part1::solve(&input);
    println!("Result pt 1: {}", r1);

    let r2 = part2::solve(&input);
    println!("Result pt 2: {}", r2);
}
