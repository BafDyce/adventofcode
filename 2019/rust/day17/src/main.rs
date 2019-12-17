/*
      -------Part 1--------   --------Part 2--------
Day       Time  Rank  Score       Time   Rank  Score
 17   00:17:20   589      0   01:18:55    400      0
BENCHMARK RESULTS
test bench::bench_parsing ... bench:     507,071 ns/iter (+/- 25,000)
test bench::bench_part1   ... bench:   6,568,731 ns/iter (+/- 239,742)
test bench::bench_part2   ... bench:  14,216,884 ns/iter (+/- 803,040)
*/

// allow bench feature when using unstable flag
// use: $ rustup run nightly cargo bench --features unstable
#![cfg_attr(feature = "unstable", feature(test))]

mod intcode;
use intcode::*;

use aoc_import_magic::{import_magic, PuzzleOptions};
use std::{
    collections::{HashMap, VecDeque},
    io,
};

const DAY: i32 = 17;
type InputTypeSingle = IntcodeNumber;
type InputType = Vec<InputTypeSingle>;
type OutputType1 = usize;
type OutputType2 = IntcodeNumber;
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
    input[0]
        .split(",")
        .map(|xx| xx.parse::<InputTypeSingle>().unwrap())
        .collect()
}

fn part1(po: &TodaysPuzzleOptions) -> OutputType1 {
    let program = po.data.as_ref().unwrap();
    let mut robot = IntcodeProcessor::new(program);

    let mut scaffolds = Vec::new();
    let mut outputs = VecDeque::new();
    let mut inputs = VecDeque::new();
    inputs.push_back(0);
    robot.run(inputs, &mut outputs, std::usize::MAX);

    let mut line = Vec::new();
    while let Some(output) = outputs.pop_front() {
        match output {
            35 => line.push('#'),
            46 => line.push('.'),
            10 => {
                scaffolds.push(line);
                line = Vec::new();
            }
            94 => line.push('^'),
            other => panic!("Invalid item {}", other),
        }
    }

    if po.verbose {
        for row in scaffolds.iter() {
            for yy in row {
                print!("{}", yy);
            }
            println!("")
        }
    }

    let mut sum = 0;
    for xx in 0 .. scaffolds.len() {
        for yy in 0 .. scaffolds[xx].len() {
            if scaffolds[xx][yy] == '#'
            && xx > 0 && scaffolds[xx-1][yy] == '#'
            && xx < scaffolds.len()-2 && scaffolds[xx+1][yy] == '#'
            && yy > 0 && scaffolds[xx][yy-1] == '#'
            && yy < scaffolds[xx].len() - 1 && scaffolds[xx][yy+1] == '#' {
                // intersection
                sum += xx * yy;
            }
        }
    }

    sum
}

fn part2(po: &TodaysPuzzleOptions, _res1: Option<OutputType1>) -> OutputType2 {
    let mut program = po.data.as_ref().unwrap().to_owned();
    program[0] = 2;
    let mut robot = IntcodeProcessor::new(&program);
    let mut outputs = VecDeque::new();

    // R,10,L,12,R,6,R,10,L,12,R,6,R,6,R,10,R,12,R,6,R,10,L,12,L,12,R,6,R,10,R,12,R,6,R,10,L,12,L,12,R,6,R,10,R,12,R,6,R,10,L,12,L,12,R,6,R,10,R,12,R,6,R,10,L,12,R,6,

    // A = R,10,L,12,L,12,
    // B = R,10,L,12,R,6
    // C = R,6,R,10,R,12,R,6

    // main = B,B,C,A,C,A,C,A,C,B

    let main = "B,B,C,A,C,A,C,A,C,B\n";
    let aa = "R,10,L,12,L,12\n";
    let bb = "R,10,L,12,R,6\n";
    let cc = "R,6,R,10,R,12,R,6\n";

    // Rust has no simple way to convert an ascii char to an integer. Since we only need a handful of
    // characters it was faster to write this per hand than to search the docu/internet for the correct way.
    let translate = |cc| {
        match cc {
            ',' => 44,
            'A' => 65,
            'B' => 66,
            'C' => 67,
            'R' => 82,
            'L' => 76,
            '\n' => 10,
            '0' => 48,
            '1' =>  49,
            '2' => 50,
            '6' => 54,
            'n' => 110,
            'y' => 121,
            other => panic!("invalid char: {}", other)
        }
    };

    let main: VecDeque<IntcodeNumber> = main.chars().map(translate).collect();
    let aa: VecDeque<IntcodeNumber> = aa.chars().map(translate).collect();
    let bb: VecDeque<IntcodeNumber> = bb.chars().map(translate).collect();
    let cc: VecDeque<IntcodeNumber> = cc.chars().map(translate).collect();
    let mut prompt: VecDeque<IntcodeNumber> = VecDeque::new();
    prompt.push_back(translate('n'));
    prompt.push_back(translate('\n'));

    let all: VecDeque<IntcodeNumber> = main.into_iter().chain(
        aa.into_iter()
    ).chain(
        bb.into_iter()
    ).chain(
        cc.into_iter()
    ).chain(
        prompt.into_iter()
    ).collect();

    match robot.run(all, &mut outputs, std::usize::MAX) {
        Some(result) => result,
        None => {
            println!("Error encountered. Debugging info:\n{:?}", outputs);
            0
        }
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
