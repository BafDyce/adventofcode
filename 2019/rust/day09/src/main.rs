/*
      -------Part 1--------   -------Part 2--------
Day       Time  Rank  Score       Time  Rank  Score
  9   03:23:35  3232      0   03:25:02  3172      0
BENCHMARK RESULTS
test bench::bench_parsing ... bench:     403,885 ns/iter (+/- 20,371)
test bench::bench_part1   ... bench:      33,794 ns/iter (+/- 1,743)
test bench::bench_part2   ... bench:  59,368,435 ns/iter (+/- 3,808,183)
*/

// allow bench feature when using unstable flag
// use: $ cargo +nightly bench --features unstable
#![cfg_attr(feature = "unstable", feature(test))]

use aoc_import_magic::{import_magic, PuzzleOptions};
use std::{collections::HashMap, io};

type IntcodeInteger = i128;

const DAY: i32 = 9;
type InputTypeSingle = IntcodeInteger;
type InputType = Vec<InputTypeSingle>;
type OutputType1 = IntcodeInteger;
type OutputType2 = OutputType1;
type TodaysPuzzleOptions = PuzzleOptions<InputType>;

fn load_from_addr(
    memory: &Vec<IntcodeInteger>,
    more_memory: &mut HashMap<usize, IntcodeInteger>,
    addr: usize,
) -> IntcodeInteger {
    if addr < memory.len() {
        memory[addr]
    } else {
        let entry = more_memory.entry(addr).or_insert(0);
        *entry
    }
}

fn write_into_addr(
    memory: &mut Vec<IntcodeInteger>,
    more_memory: &mut HashMap<usize, IntcodeInteger>,
    addr: usize,
    value: IntcodeInteger,
) {
    if addr < memory.len() {
        memory[addr] = value;
    } else {
        let entry = more_memory.entry(addr).or_insert(0);
        *entry = value;
    }
}

fn get_value_of_parameter(
    memory: &Vec<IntcodeInteger>,
    more_memory: &mut HashMap<usize, IntcodeInteger>,
    ip: usize,
    relative_base: &mut IntcodeInteger,
    param_idx: usize,
) -> IntcodeInteger {
    let modes = [
        memory[ip] / 10_000,
        (memory[ip] % 10_000) / 1_000,
        (memory[ip] % 1_000) / 100,
    ];

    let get_mode_idx = |param_idx| match param_idx {
        1 => 2,
        2 => 1,
        3 => 0,
        _ => panic!("Invalid mode idx"),
    };

    let mode_idx = get_mode_idx(param_idx);
    match modes[mode_idx] {
        0 => {
            let addr = load_from_addr(&memory, more_memory, ip + param_idx) as usize;
            load_from_addr(&memory, more_memory, addr)
        }
        1 => load_from_addr(&memory, more_memory, ip + param_idx),
        2 => {
            let addr = load_from_addr(&memory, more_memory, ip + param_idx) + *relative_base;
            load_from_addr(&memory, more_memory, addr as usize)
        }
        other => panic!("get_value_of_parameter: Invalid mode ({})", other),
    }
}

fn get_addr_from_param(
    memory: &Vec<IntcodeInteger>,
    more_memory: &mut HashMap<usize, IntcodeInteger>,
    ip: usize,
    relative_base: IntcodeInteger,
    param_idx: usize,
) -> usize {
    let modes = [
        memory[ip] / 10_000,
        (memory[ip] % 10_000) / 1_000,
        (memory[ip] % 1_000) / 100,
    ];

    let get_mode_idx = |param_idx| match param_idx {
        1 => 2,
        2 => 1,
        3 => 0,
        _ => panic!("Invalid mode idx"),
    };

    let mode_idx = get_mode_idx(param_idx);
    match modes[mode_idx] {
        0 => load_from_addr(&memory, more_memory, ip + param_idx) as usize,
        2 => (load_from_addr(&memory, more_memory, ip + param_idx) + relative_base) as usize,
        other => panic!("get_addr_from_param: Invalid mode ({})", other),
    }
}

fn run_intcode_program(program: &Vec<IntcodeInteger>, input: IntcodeInteger) -> OutputType1 {
    let mut memory = program.to_owned();
    let mut more_memory = HashMap::<usize, IntcodeInteger>::new();
    let mut output = 0;
    let mut relative_base: IntcodeInteger = 0;

    let mut ip = 0;
    loop {
        ip += match memory[ip] % 100 {
            1 => {
                // add
                let param_1 =
                    get_value_of_parameter(&memory, &mut more_memory, ip, &mut relative_base, 1);
                let param_2 =
                    get_value_of_parameter(&memory, &mut more_memory, ip, &mut relative_base, 2);

                let dst = get_addr_from_param(&memory, &mut more_memory, ip, relative_base, 3);
                write_into_addr(&mut memory, &mut more_memory, dst, param_1 + param_2);

                4
            }
            2 => {
                // multiply
                let param_1 =
                    get_value_of_parameter(&memory, &mut more_memory, ip, &mut relative_base, 1);
                let param_2 =
                    get_value_of_parameter(&memory, &mut more_memory, ip, &mut relative_base, 2);

                let dst = get_addr_from_param(&memory, &mut more_memory, ip, relative_base, 3);
                write_into_addr(&mut memory, &mut more_memory, dst, param_1 * param_2);

                4
            }
            3 => {
                // store input
                let addr = get_addr_from_param(&memory, &mut more_memory, ip, relative_base, 1);
                write_into_addr(&mut memory, &mut more_memory, addr, input);

                2
            }
            4 => {
                // get output
                output =
                    get_value_of_parameter(&memory, &mut more_memory, ip, &mut relative_base, 1);
                println!("out: {}", output);

                2
            }
            5 => {
                // jump if true
                let param_1 =
                    get_value_of_parameter(&memory, &mut more_memory, ip, &mut relative_base, 1);
                let param_2 =
                    get_value_of_parameter(&memory, &mut more_memory, ip, &mut relative_base, 2);

                if param_1 != 0 {
                    ip = param_2 as usize;
                    0
                } else {
                    3
                }
            }
            6 => {
                // jump if false
                let param_1 =
                    get_value_of_parameter(&memory, &mut more_memory, ip, &mut relative_base, 1);
                let param_2 =
                    get_value_of_parameter(&memory, &mut more_memory, ip, &mut relative_base, 2);

                if param_1 == 0 {
                    ip = param_2 as usize;
                    0
                } else {
                    3
                }
            }
            7 => {
                // less than
                let param_1 =
                    get_value_of_parameter(&memory, &mut more_memory, ip, &mut relative_base, 1);
                let param_2 =
                    get_value_of_parameter(&memory, &mut more_memory, ip, &mut relative_base, 2);

                let addr = get_addr_from_param(&memory, &mut more_memory, ip, relative_base, 3);
                write_into_addr(
                    &mut memory,
                    &mut more_memory,
                    addr,
                    if param_1 < param_2 { 1 } else { 0 },
                );

                4
            }
            8 => {
                // equal
                let param_1 =
                    get_value_of_parameter(&memory, &mut more_memory, ip, &mut relative_base, 1);
                let param_2 =
                    get_value_of_parameter(&memory, &mut more_memory, ip, &mut relative_base, 2);

                let addr = get_addr_from_param(&memory, &mut more_memory, ip, relative_base, 3);
                write_into_addr(
                    &mut memory,
                    &mut more_memory,
                    addr,
                    if param_1 == param_2 { 1 } else { 0 },
                );

                4
            }
            9 => {
                // relative_base change
                let param_1 =
                    get_value_of_parameter(&memory, &mut more_memory, ip, &mut relative_base, 1);
                relative_base += param_1;

                2
            }
            99 => {
                break output;
            }
            other => {
                panic!("Invalid opcode {} @ {} ({})", other, ip, memory[ip]);
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
    run_intcode_program(po.data.as_ref().unwrap(), 2)
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
        test_case_helper("example1", 99, 99)
    }

    #[test]
    fn example_2() {
        test_case_helper("example2", 1219070632396864, 1219070632396864)
    }

    #[test]
    fn example_3() {
        test_case_helper("example3", 1125899906842624, 1125899906842624)
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
