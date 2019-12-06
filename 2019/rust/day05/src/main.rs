/*
      -------Part 1--------   -------Part 2--------
Day       Time  Rank  Score       Time  Rank  Score
  5   00:24:38   384      0   00:36:18   390      0

BENCHMARK RESULTS
test bench::bench_parsing ... bench:       8,933 ns/iter (+/- 1,378)
test bench::bench_part1   ... bench:         345 ns/iter (+/- 39)
test bench::bench_part2   ... bench:         582 ns/iter (+/- 39)
*/

// allow bench feature when using unstable flag
// use: $ cargo +nightly bench --features unstable
#![cfg_attr(feature = "unstable", feature(test))]

use aoc_import_magic::{import_magic, PuzzleOptions};
use std::{collections::HashMap, io};

const DAY: i32 = 5;
type InputTypeSingle = i32;
type InputType = Vec<InputTypeSingle>;
type OutputType1 = i32;
type OutputType2 = OutputType1;
type TodaysPuzzleOptions = PuzzleOptions<InputType>;

fn run_intcode_program(program: &Vec<i32>, input: i32) -> OutputType1 {
    let mut memory = program.to_owned();
    let mut output = 0;

    let mut ip = 0;
    loop {
        let modes = [
            memory[ip] / 10_000,
            (memory[ip] % 10_000) / 1_000,
            (memory[ip] % 1_000) / 100,
        ];

        // Tbh, I created these three closures when cleaning up the code in the evening.
        // In my original solution I had copy & pasted these `match modes[ip] {}` blocks all over
        // the place (and fortunately changed all offsets correctly on first try :D)
        let get_mode_idx = |param_idx| match param_idx {
            1 => 2,
            2 => 1,
            3 => 0,
            _ => panic!("Invalid mode idx"),
        };

        let get_value_of_parameter = |param_idx| {
            let mode_idx = get_mode_idx(param_idx);

            match modes[mode_idx] {
                0 => {
                    let addr = memory[ip + param_idx] as usize;
                    memory[addr]
                }
                1 => memory[ip + param_idx],
                other => panic!("get_value_of_parameter: Invalid mode ({})", other),
            }
        };

        let get_addr_from_param = |param_idx| {
            let mode_idx = get_mode_idx(param_idx);

            match modes[mode_idx] {
                0 => {
                    memory[ip + param_idx] as usize
                }
                other => panic!("get_addr_from_param: Invalid mode ({})", other),
            }
        };

        ip += match memory[ip] % 100 {
            1 => {
                // add
                let param_1 = get_value_of_parameter(1);
                let param_2 = get_value_of_parameter(2);

                let dst = get_addr_from_param(3);
                memory[dst] = param_1 + param_2;

                4
            }
            2 => {
                // multiply
                let param_1 = get_value_of_parameter(1);
                let param_2 = get_value_of_parameter(2);

                let dst = get_addr_from_param(3);
                memory[dst] = param_1 * param_2;

                4
            }
            3 => {
                // store input
                let addr = get_addr_from_param(1);
                memory[addr] = input;

                2
            }
            4 => {
                // get output
                output = get_value_of_parameter(1);

                2
            }
            5 => {
                // jump if true
                let param_1 = get_value_of_parameter(1);
                let param_2 = get_value_of_parameter(2);

                if param_1 != 0 {
                    ip = param_2 as usize;
                    0
                } else {
                    3
                }
            }
            6 => {
                // jump if false
                let param_1 = get_value_of_parameter(1);
                let param_2 = get_value_of_parameter(2);

                if param_1 == 0 {
                    ip = param_2 as usize;
                    0
                } else {
                    3
                }
            }
            7 => {
                // less than
                let param_1 = get_value_of_parameter(1);
                let param_2 = get_value_of_parameter(2);

                let addr = get_addr_from_param(3);
                memory[addr] = if param_1 < param_2 { 1 } else { 0 };

                4
            }
            8 => {
                // less than
                let param_1 = get_value_of_parameter(1);
                let param_2 = get_value_of_parameter(2);

                let addr = get_addr_from_param(3);
                memory[addr] = if param_1 == param_2 { 1 } else { 0 };

                4
            }
            99 => {
                break output;
            }
            other => {
                panic!(
                    "Invalid opcode {} @ {} ({})",
                    other, ip, memory[ip]
                );
            }
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

fn parse_input(input: Vec<String>, _config: &HashMap<String, String>, _verbose: bool) -> InputType {
    input[0]
        .split(",")
        .map(|xx| xx.parse::<InputTypeSingle>().unwrap())
        .collect()
}

fn part1(po: &TodaysPuzzleOptions) -> OutputType1 {
    run_intcode_program(po.data.as_ref().unwrap(), 1)
}

fn part2(po: &TodaysPuzzleOptions, _res1: Option<OutputType1>) -> OutputType2 {
    run_intcode_program(po.data.as_ref().unwrap(), 5)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_import_magic::{import_magic_with_params, PuzzleOptions};

    pub(in super) fn import_helper(inputname: &str) -> PuzzleOptions<InputType> {
        let params = ["appname", "--input", inputname];
        import_magic_with_params(DAY, parse_input, &params).unwrap()
    }

    #[test]
    fn example_1() {
        let program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];

        assert_eq!(run_intcode_program(&program, 7), 0, "7");
        assert_eq!(run_intcode_program(&program, 8), 1, "8");
        assert_eq!(run_intcode_program(&program, 9), 0, "9");
    }

    #[test]
    fn example_2() {
        let program = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];

        assert_eq!(run_intcode_program(&program, 7), 1, "7");
        assert_eq!(run_intcode_program(&program, 8), 0, "8");
        assert_eq!(run_intcode_program(&program, 9), 0, "9");
    }

    #[test]
    fn example_3() {
        let program = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];

        assert_eq!(run_intcode_program(&program, 7), 0, "7");
        assert_eq!(run_intcode_program(&program, 8), 1, "8");
        assert_eq!(run_intcode_program(&program, 9), 0, "9");
    }

    #[test]
    fn example_4() {
        let program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];

        assert_eq!(run_intcode_program(&program, 7), 1, "7");
        assert_eq!(run_intcode_program(&program, 8), 0, "8");
        assert_eq!(run_intcode_program(&program, 9), 0, "9");
    }

    #[test]
    fn example_5() {
        let program = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];

        assert_eq!(run_intcode_program(&program, 0), 0, "0");
        assert_eq!(run_intcode_program(&program, 1), 1, "1");
    }

    #[test]
    fn example_6() {
        let program = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];

        assert_eq!(run_intcode_program(&program, 0), 0, "0");
        assert_eq!(run_intcode_program(&program, 1), 1, "1");
    }

    #[test]
    fn example_7() {
        let program = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];

        assert_eq!(run_intcode_program(&program, 7), 999, "7");
        assert_eq!(run_intcode_program(&program, 8), 1000, "8");
        assert_eq!(run_intcode_program(&program, 9), 1001, "9");
    }
}

#[cfg(all(feature = "unstable", test))]
mod bench {
    extern crate test;

    use super::*;
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };
    use test::Bencher;

    fn helper_read_file(fname: &str) -> Vec<String> {
        BufReader::new(File::open(fname).unwrap()).lines().map(|line| line.unwrap()).collect()
    }

    #[bench]
    fn bench_parsing(bb: &mut Bencher) {
        let input = helper_read_file(&format!("../../_inputs/day{:02}/real1.input", DAY));
        bb.iter(|| parse_input(input.to_owned(), &HashMap::new(), false));
    }

    #[bench]
    fn bench_part1(bb: &mut Bencher) {
        let puzzle_options = tests::import_helper("real1");
        bb.iter(|| part1(&puzzle_options));
    }

    #[bench]
    fn bench_part2(bb: &mut Bencher) {
        let puzzle_options = tests::import_helper("real1");
        bb.iter(|| part2(&puzzle_options, None));
    }
}
