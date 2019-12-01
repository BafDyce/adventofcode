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

const DAY: i32 = 0;
type InputTypeSingle = usize;
type InputType = Vec<InputTypeSingle>;
type OutputType1 = usize;
type OutputType2 = OutputType1;
type TodaysPuzzleOptions = PuzzleOptions<InputType>;

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
struct Data {

}

impl Data {
    pub fn new() -> Self {
        Data {

        }
    }
}

impl From<()> for Data {
    fn from(from: ()) -> Data {
        Data {

        }
    }
}

fn main() -> Result<(), io::Error> {
    println!("AoC 2019 | Day {}", DAY);

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

fn parse_input(input: Vec<String>, config: &HashMap<String, String>, verbose: bool) -> InputType {
    // PARSE input
    input
        .into_iter()
        .map(|line| {
            // Parsing logic
            // single numeric types

            line.parse::<InputTypeSingle>().unwrap_or_default()
            ; // <-- REMOVE THIS IF NECESSARY!!

            // regex parsing stuff
            lazy_static! {
                // (?x)
                // (?P<name>xxx)
                static ref RE: Regex = Regex::new(
                    r"([[:alpha:]])*"
                ).unwrap();
            }

            let caps = RE.captures(&line).unwrap();
            // let thingy = &caps["thingy"];
            // let xx = caps["xx"].chars().next().unwrap();
            caps.len()
        })
        .collect()
}

fn part1(po: &TodaysPuzzleOptions) -> OutputType1 {
    po.data.as_ref().unwrap().into_iter().sum()
}

fn part2(po: &TodaysPuzzleOptions, res1: Option<OutputType1>) -> OutputType2 {
    po.data.as_ref().unwrap().into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_import_magic::{import_magic_with_params, PuzzleOptions};

    fn import_helper(inputname: &str) -> PuzzleOptions<InputType> {
        let params = [
            "appname",
            "--input",
            inputname,
        ];
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
