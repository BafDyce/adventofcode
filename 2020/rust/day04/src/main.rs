/*

BENCHMARK RESULTS

*/

// allow bench feature when using unstable flag
// use: $ rustup run nightly cargo bench --features unstable
#![cfg_attr(feature = "unstable", feature(test))]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

use aoc_import_magic::{import_magic, PuzzleOptions};
use regex::Regex;
use std::{
    collections::{HashMap, VecDeque},
    io,
};

const DAY: u32 = 4;
type InputTypeSingle = usize;
type InputType = Vec<Vec<String>>;
type OutputType1 = usize;
type OutputType2 = OutputType1;
type TodaysPuzzleOptions = PuzzleOptions<InputType>;

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
struct Data {}

impl Data {
    pub fn new() -> Self {
        Data {}
    }
}

impl From<()> for Data {
    fn from(from: ()) -> Data {
        Data {}
    }
}

fn parse_input(input: Vec<String>, config: &HashMap<String, String>, verbose: bool) -> InputType {
    /*let mut data = HashMap::new();

    for line in input {
        for item in line.split_whitespace() {
            if item.starts_with("byr:") {

            }
        }
    }*/
    input.join("\n").split("\n\n").map(|line| line.split_whitespace().map(ToOwned::to_owned).collect()).collect()


    /*
    input
        .into_iter()
        .map(|line| {
            // regex parsing stuff
            /*lazy_static! {
                // (?x)
                // (?P<name>xxx)
                static ref RE: Regex = Regex::new(
                    r"()*"
                ).unwrap();
            }

            let caps = RE.captures(&line).unwrap();
            // let thingy = &caps["thingy"];
            // let xx = caps["xx"].chars().next().unwrap();
            caps.len()*/
            line.split_whitespace()
        })
        .collect()*/
}

fn part1(po: &TodaysPuzzleOptions) -> OutputType1 {
    //println!("{:?}", po.get_data());

    let mut count = 0;
    for passport in po.get_data().into_iter() {
        let mut data: HashMap<&str, bool> = [
            ("byr", false),
            ("iyr", false),
            ("eyr", false),
            ("hgt", false),
            ("hcl", false),
            ("ecl", false),
            ("pid", false),
            //("cid", false),
        ].iter().cloned().collect();

        for item in passport {
            match &item[0..3] {
                xx @ "byr" |
                xx @ "iyr" |
                xx @ "eyr" |
                xx @ "hgt" |
                xx @ "hcl" |
                xx @ "ecl" |
                xx @ "pid" => {
                    data.insert(xx, true);
                }
                _ => {}
            }
        }

        if data.values().into_iter().all(|&x| x) {
            count += 1;
        }
    }

    count
}

fn part2(po: &TodaysPuzzleOptions, res1: Option<OutputType1>) -> OutputType2 {
    let mut count = 0;
    for passport in po.get_data().into_iter() {
        let mut data: HashMap<&str, bool> = [
            ("byr", false),
            ("iyr", false),
            ("eyr", false),
            ("hgt", false),
            ("hcl", false),
            ("ecl", false),
            ("pid", false),
            //("cid", false),
        ].iter().cloned().collect();

        for item in passport {
            dbg!(item);
            match &item[0..3] {
                xx @ "byr" => {
                    match &item[4..].parse() {
                        Ok(1920 ..= 2002) => {
                            println!("correct");
                            data.insert(xx, true);
                        },
                        _ => {},
                    }
                }
                xx @ "iyr" => {
                    match &item[4..].parse() {
                        Ok(2010 ..= 2020) => {
                            println!("correct");
                            data.insert(xx, true);
                        },
                        _ => {},
                    }
                }
                xx @ "eyr" => {
                    match &item[4..].parse() {
                        Ok(2020 ..= 2030) => {
                            println!("correct");
                            data.insert(xx, true);
                        },
                        _ => {},
                    }
                }
                xx @ "hgt" => {
                    //dbg!(item);
                    if item.ends_with("cm") {
                        match &item[4..item.len()-2].parse() {
                            Ok(150 ..= 193) => {
                                println!("correct");
                                data.insert(xx, true);
                            },
                            _ => {},
                        }
                    } else if item.ends_with("in") {
                        match &item[4..item.len()-2].parse() {
                            Ok(59 ..= 76) => {
                                println!("correct");
                                data.insert(xx, true);
                            },
                            _ => {},
                        }
                    }

                }
                xx @ "hcl" => {
                    if item.chars().nth(4).unwrap() == '#' && item[5..].chars().all(|cc| cc.is_ascii_hexdigit()) {
                        println!("correct");
                        data.insert(xx, true);
                    }
                }
                xx @ "ecl" => {
                    match dbg!(&item[4..]) {
                        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {
                            println!("correct");
                            data.insert(xx, true);
                        }
                        _ => {},
                    }
                }
                xx @ "pid" => {
                    if item.len() == 13 && item[5..].chars().all(|cc| cc.is_ascii_digit()) {
                        println!("correct");
                        data.insert(xx, true);
                    }
                }
                _ => {}
            }
        }

        if data.values().into_iter().all(|&x| x) {
            count += 1;
        }
    }

    count
}


// =================================================================================================
// End of actual logic
// What follows is the main function glue as well as tests + benchmarking code
// =================================================================================================
fn main() -> Result<(), io::Error> {
    println!("AoC 2020 | Day {}", DAY);

    // This function is pure magic (see ../../aoc_import_magic/lib.rs) because it
    // 1. parses command line arguments
    // 2. reads the input file for the correct day
    // 3. uses `parse_input` as a parsing function
    // 4. returns a nice usable struct which contains everything which we need for the actual puzzle
    let puzzle = import_magic(DAY, parse_input)?;
    let res1 = if puzzle.skip_p1 {
        None
    } else {
        let res1 = part1(&puzzle);
        println!("Part 1 result: {}", res1);
        Some(res1)
    };

    let res2 = part2(&puzzle, res1);
    println!("Part 2 result: {}", res2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_import_magic::{import_magic_with_params, PuzzleOptions};

    pub(super) fn import_helper(inputname: &str) -> PuzzleOptions<InputType> {
        let params = ["appname", "--input", inputname];
        import_magic_with_params(DAY, parse_input, &params).unwrap()
    }

    fn test_case_helper(inputname: &str, sol1: OutputType1, sol2: OutputType2) {
        let po = import_helper(inputname);
        let res1 = part1(&po);
        assert_eq!(sol1, res1, "part1");
        let res2 = part2(&po, Some(res1));
        assert_eq!(sol2, res2, "part2");
    }

    #[test]
    fn example_1() {
        test_case_helper("example1", 8, 8)
    }
}

#[cfg(all(feature = "unstable", test))]
mod bench {
    extern crate test;

    use super::*;
    use aoc_import_magic::test_helper_import_config;
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };
    use test::Bencher;

    fn helper_read_file(fname: &str) -> Vec<String> {
        BufReader::new(File::open(fname).unwrap())
            .lines()
            .map(|line| line.unwrap())
            .collect()
    }

    #[bench]
    fn bench_parsing(bb: &mut Bencher) {
        let input = helper_read_file(&format!("../../_inputs/day{:02}/real1.input", DAY));
        let config = test_helper_import_config(DAY, "real1");

        bb.iter(|| test::black_box(parse_input(input.to_owned(), &config, false)));
    }

    #[bench]
    fn bench_part1(bb: &mut Bencher) {
        let puzzle_options = tests::import_helper("real1");
        bb.iter(|| test::black_box(part1(&puzzle_options)));
    }

    #[bench]
    fn bench_part2(bb: &mut Bencher) {
        let puzzle_options = tests::import_helper("real1");
        bb.iter(|| test::black_box(part2(&puzzle_options, None)));
    }
}
