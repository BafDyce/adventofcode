/*
      -------Part 1--------   -------Part 2--------
Day       Time  Rank  Score       Time  Rank  Score
  2   00:11:20   563      0   00:17:03   433      0

BENCHMARK RESULTS
test bench::bench_parsing ... bench:       1,933 ns/iter (+/- 261)
test bench::bench_part1   ... bench:         109 ns/iter (+/- 7)
test bench::bench_part2   ... bench:     935,529 ns/iter (+/- 70,863)
*/

// allow bench feature when using unstable flag
// use: $ cargo +nightly bench --features unstable
#![cfg_attr(feature = "unstable", feature(test))]

use aoc_import_magic::{import_magic, PuzzleOptions};
use std::{
    collections::HashMap,
    io,
};

const DAY: i32 = 2;
type InputTypeSingle = usize;
type InputType = Vec<InputTypeSingle>;
type OutputType1 = usize;
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

fn parse_input(input: Vec<String>, _config: &HashMap<String, String>, _verbose: bool) -> InputType {
    input[0].split(",").map(|xx| xx.parse::<usize>().unwrap()).collect()
}

// originally I've written this in `part1()` and then copied it into `part2()`
fn run_intcode_program(program: &[usize], noun: usize, verb: usize) -> usize {
    let mut memory = program.to_owned();
    memory[1] = noun;
    memory[2] = verb;

    let mut next_opcode_idx = 0;
    loop {
        next_opcode_idx += match memory[next_opcode_idx] {
            1 => {
                // add
                let src_1 = memory[next_opcode_idx + 1];
                let src_2 = memory[next_opcode_idx + 2];
                let dst = memory[next_opcode_idx + 3];
                memory[dst] = memory[src_1] + memory[src_2];

                4
            }
            2 => {
                // multiply
                let src_1 = memory[next_opcode_idx + 1];
                let src_2 = memory[next_opcode_idx + 2];
                let dst = memory[next_opcode_idx + 3];
                memory[dst] = memory[src_1] * memory[src_2];

                4
            }
            99 => {
                break memory[0];
            }
            other => {
                panic!("Invalid opcode {} @ {}", other, next_opcode_idx);
            }
        }
    }
}

fn part1(po: &TodaysPuzzleOptions) -> OutputType1 {
    run_intcode_program(po.data.as_ref().unwrap(), 12, 2)
}

fn part2(po: &TodaysPuzzleOptions, _res1: Option<OutputType1>) -> OutputType2 {
    for ii in 0 ..= 99 {
        for jj in 0 ..= 99 {
            let res = run_intcode_program(po.data.as_ref().unwrap(), ii, jj);

            if res == 19690720 {
                return 100 * ii + jj;
            }
        }
    }

    panic!("No solution found :(")
}

// No tests today because the test cases are not applicable due to the memory init setup..


#[cfg(all(feature = "unstable", test))]
mod bench {
    extern crate test;

    use super::*;
    use aoc_import_magic::{import_magic_with_params, PuzzleOptions};
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };
    use test::Bencher;

    fn import_helper(inputname: &str) -> PuzzleOptions<InputType> {
        let params = [
            "appname",
            "--input",
            inputname,
        ];
        import_magic_with_params(DAY, parse_input, &params).unwrap()
    }

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
        let puzzle_options = import_helper("real1");
        bb.iter(|| part1(&puzzle_options));
    }

    #[bench]
    fn bench_part2(bb: &mut Bencher) {
        let puzzle_options = import_helper("real1");
        bb.iter(|| part2(&puzzle_options, None));
    }
}