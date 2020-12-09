/*
      -------Part 1--------   -------Part 2--------
Day       Time  Rank  Score       Time  Rank  Score
  9   00:10:55  2477      0   00:17:35  1804      0

test bench::bench_parsing ... bench:      35,397 ns/iter (+/- 1,269)
test bench::bench_part1   ... bench:     718,034 ns/iter (+/- 24,933)
test bench::bench_part2   ... bench:     180,179 ns/iter (+/- 4,292)

*/

// allow bench feature when using unstable flag
// use: $ rustup run nightly cargo bench --features unstable
#![cfg_attr(feature = "unstable", feature(test))]

use aoc_import_magic::{import_magic, PuzzleOptions};
use itertools::Itertools;
use std::{
    collections::HashMap,
    io,
};

const DAY: u32 = 9;
type InputTypeSingle = usize;
type InputType = Vec<InputTypeSingle>;
type OutputType1 = usize;
type OutputType2 = OutputType1;
type TodaysPuzzleOptions = PuzzleOptions<InputType>;


fn parse_input(input: Vec<String>, _config: &HashMap<String, String>, _verbose: bool) -> InputType {
    input
        .into_iter()
        .map(|line| {
            line.parse::<InputTypeSingle>().unwrap()
        })
        .collect()
}

fn part1(po: &TodaysPuzzleOptions) -> Option<OutputType1> {
    let numbers = po.get_data();
    // here my template magic comes in handy, as I can simply create a config file with a different
    // preamble for the example input and can easily switch between example and real input without
    // having to worry about adapting the preamble size ;)
    let window_size = match po.config.get("preamble") {
        Some(number) => number.parse().unwrap(),
        None => 25usize,
    };

    for ii in window_size .. numbers.len() {
        let check = numbers[ii];
        let valid = numbers[ii-window_size .. ii].iter().combinations(2).any(|pair| {
            let aa = pair[0];
            let bb = pair[1];

            check == aa + bb
        });

        if !valid {
            return Some(check);
        }
    }

    None
}

fn part2(po: &TodaysPuzzleOptions, sum_to_find: Option<OutputType1>) -> OutputType2 {
    let numbers = po.get_data();
    let sum_to_find = sum_to_find.unwrap();

    for ii in 0 .. numbers.len() {
        let mut sum = 0;
        let mut numberset = Vec::new();

        for jj in ii .. numbers.len() {
            let number_to_add = numbers[jj];
            sum += number_to_add;
            if sum == sum_to_find {
                // Since we need both, smallest and highest number, it's faster to sort first, then
                // get first and last item
                numberset.sort();
                return numberset.first().unwrap() + numberset.last().unwrap();
            } else if sum > sum_to_find {
                // shortcut if we're already adding up too much
                break;
            } else {
                numberset.push(number_to_add);
            }
        }
    }

    0
}


// =================================================================================================
// End of actual logic
// What follows is the main function glue as well as tests + benchmarking code
// =================================================================================================
fn main() -> Result<(), io::Error> {
    println!("AoC 2020 | Day {}", DAY);

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
        println!("Part 1 result: {:?}", res1);
        res1
    };

    let res2 = part2(&puzzle, res1);
    println!("Part 2 result: {}", res2);

    Ok(())
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
        assert_eq!(Some(sol1), res1, "part1");
        let res2 = part2(&po, res1);
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
        let res1 = part1(&puzzle_options);
        bb.iter(|| test::black_box(part2(&puzzle_options, res1)));
    }
}
