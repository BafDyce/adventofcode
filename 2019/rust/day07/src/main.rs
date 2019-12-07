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

use aoc_import_magic::{import_magic, PuzzleOptions};
use regex::Regex;
use std::{
    collections::{HashMap, VecDeque},
    io,
};
use std::convert::TryInto;

const DAY: i32 = 7;
type InputTypeSingle = i32;
type InputType = Vec<InputTypeSingle>;
type OutputType1 = i32;
type OutputType2 = OutputType1;
type TodaysPuzzleOptions = PuzzleOptions<InputType>;

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
    use permutohedron::{
        Heap,
        heap_recursive,
    };

    let mut phases = vec![0, 1, 2, 3, 4];
    let mut permutations = Heap::new(&mut phases);

    let mut highest = 0;
    while let Some(permutations) = permutations.next_permutation() {
        //println!("{:?}", permutations);

        let aa = run_intcode_program(po.data.as_ref().unwrap(), vec![ permutations[0], 0]);
        let bb = run_intcode_program(po.data.as_ref().unwrap(), vec![ permutations[1] ,aa]);
        let cc = run_intcode_program(po.data.as_ref().unwrap(), vec![ permutations[2], bb]);
        let dd = run_intcode_program(po.data.as_ref().unwrap(), vec![ permutations[3], cc]);
        let ee = run_intcode_program(po.data.as_ref().unwrap(), vec![ permutations[4], dd]);

        highest = i32::max(highest, ee);
    }

    highest
}

fn part2(po: &TodaysPuzzleOptions, _res1: Option<OutputType1>) -> OutputType2 {
    run_feedback_loop(po.data.as_ref().unwrap())
}

fn run_feedback_loop(program: &Vec<i32>) -> OutputType2 {
    use permutohedron::{
        Heap,
        heap_recursive,
    };

    let mut phases = vec![5, 6, 7, 8, 9];
    let mut permutations = Heap::new(&mut phases);
    let mut highest = 0;
    while let Some(permutations) = permutations.next_permutation() {
        let mut controllers = [
            program.to_owned(),
            program.to_owned(),
            program.to_owned(),
            program.to_owned(),
            program.to_owned(),
        ];

        let mut ips = [0; 5];

        let mut io = [
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
        ];


        // let permutations = vec![9,8,7,6,5];
        io[0].push_back(permutations[0]);
        io[0].push_back(0);
        io[1].push_back(permutations[1]);
        io[2].push_back(permutations[2]);
        io[3].push_back(permutations[3]);
        io[4].push_back(permutations[4]);

            loop {
                let aa = run_intcode_program2(&mut controllers[0], &mut ips[0], &mut io, 0, 1);
                let bb = run_intcode_program2(&mut controllers[1], &mut ips[1], &mut io, 1, 2);
                let cc = run_intcode_program2(&mut controllers[2], &mut ips[2], &mut io, 2, 3);
                let dd = run_intcode_program2(&mut controllers[3], &mut ips[3], &mut io, 3, 4);
                if let Some(ee) = run_intcode_program2(&mut controllers[4], &mut ips[4], &mut io, 4, 0) {
                    //print!("new val: {}", ee);
                    highest = i32::max(highest, ee);
                    //println!(" ==> highest == {}", highest);
                    break;
                }
            }


    }

    highest
}

// inputs: &mut VecDeque<i32>, outputs: &mut VecDeque<i32>
fn run_intcode_program2(memory: &mut Vec<i32>, ip: &mut usize, ios: &mut [VecDeque<i32>; 5], input_idx: usize, output_idx: usize) -> Option<OutputType1> {
    let mut output = 0;
    loop {
        let modes = [
            memory[*ip] / 10_000,
            (memory[*ip] % 10_000) / 1_000,
            (memory[*ip] % 1_000) / 100,
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
                    let addr: usize = memory[*ip + param_idx].try_into().unwrap();
                    memory[addr]
                }
                1 => memory[*ip + param_idx],
                other => panic!("get_value_of_parameter: Invalid mode ({})", other),
            }
        };

        let get_addr_from_param = |param_idx| {
            let mode_idx = get_mode_idx(param_idx);

            match modes[mode_idx] {
                0 => {
                    memory[*ip + param_idx] as usize
                }
                other => panic!("get_addr_from_param: Invalid mode ({})", other),
            }
        };

        //println!("memory[{}] = {}", ip, memory[ip]);
        *ip += match memory[*ip] % 100 {
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
                match ios[input_idx].pop_front() {
                    Some(input) => memory[addr] = input,
                    None => break None,
                }

                2
            }
            4 => {
                // get output
                output = get_value_of_parameter(1);
                ios[output_idx].push_back(output);

                2
            }
            5 => {
                // jump if true
                let param_1 = get_value_of_parameter(1);
                let param_2 = get_value_of_parameter(2);

                if param_1 != 0 {
                    *ip = param_2.try_into().unwrap();
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
                    *ip = param_2.try_into().unwrap();
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
                break Some(output);
            }
            other => {
                panic!(
                    "Invalid opcode {} @ {} ({})",
                    other, ip, memory[*ip]
                );
            }
        }
    }
}

fn run_intcode_program(program: &Vec<i32>, inputs: Vec<i32>) -> OutputType1 {
    let mut memory = program.to_owned();
    let mut output = 0;
    let mut inputs = inputs.into_iter();

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

            match modes[0] {
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
                memory[addr] = inputs.next().unwrap();

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

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_import_magic::{import_magic_with_params, PuzzleOptions};

    pub(in super) fn import_helper(inputname: &str) -> PuzzleOptions<InputType> {
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
    fn example_4() {
        let program = vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];

        assert_eq!(run_feedback_loop(&program), 139629729, "7");
    }

    #[test]
    fn example_5() {
        let program = vec![3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10];

        assert_eq!(run_feedback_loop(&program), 18216, "7");
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
