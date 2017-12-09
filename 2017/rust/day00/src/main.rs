extern crate aocutils;
extern crate regex;

mod part1;
mod part2;

fn main() {
    let day: i32 = 0;

    //let input = aocutils::import(day, Some("puzzle1"));
    //let input = aocutils::import(day, Some("puzzle1")).remove(0);
    // let input: Vec<i32> = aocutils::import(day, Some("puzzle1")).iter().map(|x| x.parse::<i32>().unwrap()).collect();
    //let input: Vec<i32> = aocutils::import(day, Some("puzzle1")).remove(0).split('\t').map(|x| x.parse::<i32>().unwrap()).collect();

    let re = Regex::new(
        r"(?P<name>^\w*) \((?P<weight>\d*)\)(?: -> (?P<others>.*))?").unwrap();
    let things = re.captures(line).unwrap();

    let res1 = part1::solve(&input);
    let res2 = part2::solve(&input);

    println!("Results: {} and {}", res1, res2);
}
