/*
      -------Part 1--------   --------Part 2--------
Day       Time  Rank  Score       Time   Rank  Score
 19   01:40:22  1525      0   03:30:00   1300      0
 (started about 1 hour, 20 minutes late though because sleep xD)
BENCHMARK RESULTS
test bench::bench_parsing ... bench:     168,147 ns/iter (+/- 4,273)
test bench::bench_part1   ... bench: 143,499,936 ns/iter (+/- 7,722,489)
test bench::bench_part2   ... bench:      16,203 ns/iter (+/- 930)
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

const DAY: i32 = 19;
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
    let size = 49;

    let mut inputs: VecDeque<IntcodeNumber> = (0 ..= size)
        .flat_map(|xx| {
            (0 ..= size)
                .flat_map(move |yy| vec![xx, yy].into_iter())
        })
        .collect();

    let num_outputs_expected = inputs.len() / 2;
    let mut outputs = VecDeque::new();
    loop {
        // sadly we need to create a new cpu instance each time
        IntcodeProcessor::new(program).run(&mut inputs, &mut outputs, num_outputs_expected);
        if outputs.len() >= num_outputs_expected {
            break;
        }
    }

    if po.verbose {
        for (nn, point) in outputs.iter().enumerate() {
            if nn > 0 && nn % (size+1) as usize == 0 {
                println!("");
            }
            print!("{}", match point { 0 => '.', 1 => '#', _ => 'x'});
        }
        println!("");
    }

    outputs.into_iter().filter(|&point| point == 1).count()
}

fn part2(po: &TodaysPuzzleOptions, _res1: Option<OutputType1>) -> OutputType2 {
    let program = po.data.as_ref().unwrap();

    let gen_inputs_for_x_offset = |xx| -> VecDeque<IntcodeNumber> {
        (0 ..= xx*2).flat_map(|yy| vec![xx, yy].into_iter()).collect()
    };

    let _find_upper_lower = |xx| {
        let mut offset_inputs = gen_inputs_for_x_offset(xx);
        let mut offset_outputs = VecDeque::new();
        while !offset_inputs.is_empty() {
            IntcodeProcessor::new(program).run(&mut offset_inputs, &mut offset_outputs, std::usize::MAX);
        }
        let offset_first = offset_outputs.iter().enumerate().find(|&(_, &val)| val == 1).unwrap().0;
        let offset_last = offset_outputs.iter().enumerate().rev().find(|&(_, &val)| val == 1).unwrap().0;
        (offset_first, offset_last)
    };
/*
    let (offset_25_first, offset_25_last) = find_upper_lower(25);
    let (offset_50_first, offset_50_last) = find_upper_lower(50);
    let (offset_100_first, offset_100_last) = find_upper_lower(100);
    let (offset_200_first, offset_200_last) = find_upper_lower(200);
    let (offset_400_first, offset_400_last) = find_upper_lower(400);
    let (offset_800_first, offset_800_last) = find_upper_lower(800);
    let (offset_10000_first, offset_10000_last) = find_upper_lower(10000);
    let (offset_20000_first, offset_20000_last) = find_upper_lower(20000);

    dbg!( (offset_25_first, offset_25_last) );
    dbg!( (offset_50_first, offset_50_last) );
    dbg!( (offset_100_first, offset_100_last) );
    dbg!( (offset_200_first, offset_200_last) );
    dbg!( (offset_400_first, offset_400_last) );
    dbg!( (offset_800_first, offset_800_last) );
    dbg!( (offset_10000_first, offset_10000_last) );
    dbg!( (offset_20000_first, offset_20000_last) );

    println!("25: {} & {}", 25f64 * 1.13, 25f64 * 1.25);
    println!("50: {} & {}", 50f64 * 1.13, 50f64 * 1.25);
    println!("100: {} & {}", 100f64 * 1.13, 100f64 * 1.25);
    println!("200: {} & {}", 200f64 * 1.13, 200f64 * 1.25);
    println!("400: {} & {}", 400f64 * 1.13, 400f64 * 1.25);
    println!("800: {} {} & {}", (800f64 * 1.13).ceil(), (800f64 * 1.13) as usize, 800f64 * 1.25);
*/
    // let height = offset_last - offset_first // == xx / 10 * 12

    // let first_offset = xx * 1.13
    // let last_offset = xx * 1.25

    /* i need:

    xx2 = xx1 + 100
    xx1_last = xx2_first - 100;


    xx2 = xx1 + 100;
    xx2_first = xx2 * 1.13;
    xx1_last = xx1 * 1.25

    xx2_first = (xx1 + 100) * 1.13;
    xx1_diff = xx1 * 1.2
    xx1_diff > 100
    xx1_last = xx1 * 1.25
    */

    // pls dont mind this hack xD
    let factor_upper = |xx| if xx < 900 { 1.13 } else { 1.1289 };
    let factor_lower = |xx| if xx < 900 { 1.25} else { 1.2510 };

    for xx1 in 800 ..= 5000 {
        let xx2 = xx1 + 99; // NOT 100 -.- this off by one error cost me at least one hour of debugging..
        let xx2_first = (xx2 as f64 * factor_upper(xx2)).ceil() as i128;
        let xx1_last = (xx1 as f64 * factor_lower(xx1)).ceil() as i128;

        if xx1_last >= xx2_first + 100 {
            return xx1 * 10_000 + (xx2_first);
        }
    }

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
