/*
      -------Part 1--------   --------Part 2--------
Day       Time  Rank  Score       Time   Rank  Score
 21   10:47:19  2201      0   11:01:30   1813      0
BENCHMARK RESULTS
test bench::bench_parsing ... bench:     837,502 ns/iter (+/- 76,470)
test bench::bench_part1   ... bench:   5,134,821 ns/iter (+/- 627,474)
test bench::bench_part2   ... bench: 126,146,511 ns/iter (+/- 3,964,103)
*/

// allow bench feature when using unstable flag
// use: $ rustup run nightly cargo bench --features unstable
#![cfg_attr(feature = "unstable", feature(test))]

mod intcode;
use intcode::*;

use aoc_import_magic::{import_magic, PuzzleOptions};
use std::{
    collections::{HashMap, VecDeque},
    convert::TryFrom,
    io,
};

const DAY: i32 = 21;
type InputTypeSingle = IntcodeNumber;
type InputType = Vec<InputTypeSingle>;
type OutputType1 = IntcodeNumber;
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
    input[0]
        .split(",")
        .map(|xx| xx.parse::<InputTypeSingle>().unwrap())
        .collect()
}

fn droid_exec(program: &Vec<IntcodeNumber>, springscript: String) -> OutputType1 {
    let mut droid = IntcodeProcessor::new(program);
    let mut inputs = intcode::ascii_to_intcode_numbers(springscript);
    let mut outputs = VecDeque::new();
    while let None = droid.run(&mut inputs, &mut outputs, 1) {
        while let Some(output) = outputs.pop_front() {
            match u8::try_from(output) {
                Ok(ascii) if ascii.is_ascii() => print!("{}", char::from(ascii)),
                _ => return output,
            }
        }
    }

    0
}

fn part1(po: &TodaysPuzzleOptions) -> OutputType1 {
    let program = po.data.as_ref().unwrap();

    /* JUMP IF
        -) landing space (D) is ground
        -) any field between us and landing space is a hole
    */
    let springscript = "NOT A J
    NOT B T
    OR T J
    NOT C T
    OR T J
    AND D J
    WALK
    ".to_string();

    droid_exec(program, springscript)
}

fn part2(po: &TodaysPuzzleOptions, _res1: Option<OutputType1>) -> OutputType2 {
    let program = po.data.as_ref().unwrap();

    /* JUMP IF
        -) landing space (D) is ground
        -) any field between us and landing space is a hole
        -) and field after landing space (E) or next landing space (H) is ground
    */
    let springscript = "NOT A J
    NOT B T
    OR T J
    NOT C T
    OR T J
    AND D J
    NOT H T
    NOT T T
    OR E T
    AND T J
    RUN
    ".to_string();

    droid_exec(program, springscript)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_import_magic::{import_magic_with_params, PuzzleOptions};

    pub(super) fn import_helper(inputname: &str) -> PuzzleOptions<InputType> {
        let params = ["appname", "--input", inputname];
        import_magic_with_params(DAY, parse_input, &params).unwrap()
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
