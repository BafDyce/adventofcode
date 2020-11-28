/*

BENCHMARK RESULTS

*/

// allow bench feature when using unstable flag
// use: $ rustup run nightly cargo bench --features unstable
#![cfg_attr(feature = "unstable", feature(test))]

mod intcode;
use intcode::*;

use aoc_import_magic::{import_magic, PuzzleOptions};
use combinations::Combinations;
use regex::Regex;
use std::{
    collections::{HashMap, VecDeque},
    io,
};

const DAY: i32 = 25;
type InputTypeSingle = IntcodeNumber;
type InputType = Vec<InputTypeSingle>;
type OutputType1 = usize;
type OutputType2 = OutputType1;
type TodaysPuzzleOptions = PuzzleOptions<InputType>;

const PROMPT: &str = "Command?\n";

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
    let program = po.data.as_ref().unwrap();

    let mut droid = IntcodeProcessor::new(program);
    let mut inputs = ascii_to_intcode_numbers("east
take sand
west
south
take ornament
north
west
north
take wreath
east
take fixed point
west
south
south
south
take candy cane
north
east
east
east
take space law space brochure
south
take fuel cell
south\n".to_string());
    let mut outputs = VecDeque::new();

    let is_cmd_prompt = |outputs: &VecDeque<IntcodeNumber>| {
        outputs.len() >= 9
        && outputs[outputs.len() - 1] == IntcodeNumber::from('\n' as u8)
        && outputs[outputs.len() - 1 - 1] == IntcodeNumber::from('?' as u8)
        && outputs[outputs.len() - 1 - 2] == IntcodeNumber::from('d' as u8)
        && outputs[outputs.len() - 1 - 3] == IntcodeNumber::from('n' as u8)
        && outputs[outputs.len() - 1 - 4] == IntcodeNumber::from('a' as u8)
        && outputs[outputs.len() - 1 - 5] == IntcodeNumber::from('m' as u8)
        && outputs[outputs.len() - 1 - 6] == IntcodeNumber::from('m' as u8)
        && outputs[outputs.len() - 1 - 7] == IntcodeNumber::from('o' as u8)
        && outputs[outputs.len() - 1 - 8] == IntcodeNumber::from('C' as u8)
    };

    let mut items = [
        "space law space brochure",
        "candy cane",
        "sand",
        "ornament",
        "fuel cell",
        "fixed point",
        "wreath",
    ];

    let mut combinations = Combinations::new(items.to_vec(), 1).chain(
        Combinations::new(items.to_vec(), 2)
    ).chain(
        Combinations::new(items.to_vec(), 3)
    ).chain(
        Combinations::new(items.to_vec(), 4)
    ).chain(
        Combinations::new(items.to_vec(), 5)
    ).chain(
        Combinations::new(items.to_vec(), 6)
    )/*.chain(
        Combinations::new(items.to_vec(), 7)
    )*/;
    //println!("combinations: {:?}", combinations);
    //println!("# of combinations: {}", combinations.len());

    enum State {
        Collecting,
        Loading,
        Checking,
        Unloading,
        Manual,
    };
    let mut state = State::Collecting;
    let mut unload_cmd = String::new();

    let too_heavy = "Droids on this ship are lighter than the detected value!";
    let too_light = "Droids on this ship are heavier than the detected value!";

    loop {
        //println!("remaining input: {:?}", inputs);
        match droid.run(&mut inputs, &mut outputs, 1) {
            None => {
                if is_cmd_prompt(&outputs) {
                    print!("{}", intcode_numbers_to_ascii(&outputs));

                    match state {
                        State::Collecting if inputs.len() == 0 => {
                            inputs = ascii_to_intcode_numbers("drop space law space brochure
drop candy cane
drop sand
drop ornament
drop fuel cell
drop fixed point
drop wreath\n".to_string());
                            state = State::Unloading;
                        }
                        State::Collecting => {}
                        State::Unloading if inputs.len() == 0 => {
                            if let Some(next_combi) = combinations.next() {
                                let mut load_cmd = String::new();

                                for item in next_combi {
                                    load_cmd.push_str("take ");
                                    load_cmd.push_str(item);
                                    load_cmd.push_str("\n");

                                    unload_cmd.push_str("drop ");
                                    unload_cmd.push_str(item);
                                    unload_cmd.push_str("\n");
                                }

                                inputs = ascii_to_intcode_numbers(load_cmd);
                                state = State::Loading;
                            } else {
                                println!("Ran out of combinations :(");
                                let mut cmd = String::new();
                                io::stdin().read_line(&mut cmd).unwrap();
                                inputs.append(&mut ascii_to_intcode_numbers(cmd));
                                state = State::Manual;
                            }
                        }
                        State::Loading if inputs.len() == 0 => {
                            inputs = ascii_to_intcode_numbers("west\n".to_string());
                            state = State::Checking;
                        }
                        State::Checking if inputs.len() == 0 => {
                            let outstring = intcode_numbers_to_ascii(&outputs);

                            if outstring.find(too_heavy).is_some() || outstring.find(too_light).is_some() {
                                inputs = ascii_to_intcode_numbers(unload_cmd);
                                unload_cmd = String::new();
                                state = State::Unloading;
                            } else {
                                println!("Yeah!");
                                let mut cmd = String::new();
                                io::stdin().read_line(&mut cmd).unwrap();
                                inputs.append(&mut ascii_to_intcode_numbers(cmd));
                                State::Manual;
                            }
                        }
                        State::Manual if inputs.len() == 0 => {
                            let mut cmd = String::new();
                            io::stdin().read_line(&mut cmd).unwrap();
                            inputs.append(&mut ascii_to_intcode_numbers(cmd));
                        }
                        _ => {}
                    }

                    outputs.clear();
                }
            }
            Some(_) => {
                print!("{}", intcode_numbers_to_ascii(&outputs));
                break;
            }
        }
        //print!("{}", outputs[outputs.len() - 1] as u8 as char);

    }

    0
}

fn part2(po: &TodaysPuzzleOptions, res1: Option<OutputType1>) -> OutputType2 {
    0
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
