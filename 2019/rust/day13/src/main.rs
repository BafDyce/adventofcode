/*

BENCHMARK RESULTS

*/

// allow bench feature when using unstable flag
// use: $ cargo +nightly bench --features unstable
#![cfg_attr(feature = "unstable", feature(test))]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;


mod intcode;
use intcode::*;

use aoc_import_magic::{import_magic, PuzzleOptions};
use regex::Regex;
use std::{
    collections::{HashMap, VecDeque},
    io::{self, BufRead},
};

const DAY: i32 = 13;
type InputTypeSingle = IntcodeNumber;
type InputType = Vec<InputTypeSingle>;
type OutputType1 = usize;
type OutputType2 = IntcodeNumber;
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
    input[0]
        .split(",")
        .map(|xx| xx.parse::<InputTypeSingle>().unwrap())
        .collect()
}

fn part1(po: &TodaysPuzzleOptions) -> OutputType1 {
    let mut cpu = IntcodeProcessor::new(po.data.as_ref().unwrap());

    let mut field = HashMap::new();
    let mut outputs = VecDeque::new();

    while let None = cpu.run(0, &mut outputs) {
        assert!(outputs.len() == 3);
        let xx = outputs.pop_front().unwrap();
        let yy = outputs.pop_front().unwrap();
        let id = outputs.pop_front().unwrap();

        field.insert( (xx, yy), id );

    }

    field.values().filter(|&&val| val == 2).count()
}

fn part2(po: &TodaysPuzzleOptions, res1: Option<OutputType1>) -> OutputType2 {
    let mut field = HashMap::new();
    let mut outputs = VecDeque::new();
    let mut score = 0;

    let mut memory = po.data.as_ref().unwrap().to_owned();
    memory[0] = 2;
    let mut cpu = IntcodeProcessor::new(&memory);

    let mut input = 0;
    let stdin = io::stdin();
    let mut stdin = stdin.lock().lines();

    /*print_field(&field, score);
    println!("Enter direction: ");
    input = match stdin.next().unwrap().unwrap().chars().nth(0).unwrap() {
        'a' => -1,
        'd' => 1,
        _ => 0,
    };
    println!("input = {}", input);*/
    let mut playing = false;
    let mut paddle_pos = 0;
    let mut frame_count = 0;
    while let None = cpu.run(input, &mut outputs) {
        assert!(outputs.len() == 3);
        let xx = outputs.pop_front().unwrap();
        let yy = outputs.pop_front().unwrap();
        let id = outputs.pop_front().unwrap();

        if playing {
            println!("xx == {} | yy = {} | id == {}", xx, yy, id);
        }

        if xx == -1 && yy == 0 {
            if ! playing {
                println!("GAME STARTED");
                playing = true;
            }
            score = id;
        } else {
            if id == 3 {
                println!("{} {}", xx, yy);
                paddle_pos = xx;
            } else if id == 4 {
                println!("BALL MOVED TO {}/{}", xx, yy);
            }
            field.insert( (xx, yy), id );
        }

        if playing && frame_count % 2 == 0 {
            print_field(&field, score);
            println!("Enter direction: ");
            input = match stdin.next() {
                Some(Ok(line)) => match line.chars().nth(0) {
                    Some('a') => -1,
                    Some('d') => 1,
                    _ => 0,
                },
                _ => 0,
            };
            println!("input = {}", input);
        }

        if playing {
            frame_count += 1;
        }
    }

    score
}

fn print_field(field: &HashMap<(IntcodeNumber, IntcodeNumber), IntcodeNumber>, score: IntcodeNumber) {
    let mut xx_min = std::i128::MAX;
    let mut xx_max = std::i128::MIN;
    let mut yy_min = std::i128::MAX;
    let mut yy_max = std::i128::MIN;

    for &(xx, yy) in field.keys() {
        xx_min = i128::min(xx_min, xx);
        xx_max = i128::max(xx_max, xx);
        yy_min = i128::min(yy_min, yy);
        yy_max = i128::max(yy_max, yy);
    }

    println!("=========\n> Score: {}", score);
    //for xx in xx_min ..= xx_max {
    //    for yy in yy_min ..= yy_max {
    for yy in yy_min ..= yy_max {
        for xx in xx_min ..= xx_max {
            let cc = match field.get( &(xx, yy) ) {
                Some(0) => ' ',
                Some(1) => '|',
                Some(2) => '+',
                Some(3) => '-',
                Some(4) => 'o',
                _ => ' ',
            };
            print!("{}", cc);
        }
        println!("");
    }
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
