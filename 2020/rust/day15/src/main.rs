/*
      -------Part 1--------   -------Part 2--------
Day       Time  Rank  Score       Time  Rank  Score
 15   00:32:48  3743      0   00:34:00  2246      0

test bench::bench_parsing ... bench:         187 ns/iter (+/- 4)
test bench::bench_part1   ... bench:     111,132 ns/iter (+/- 581)
test bench::bench_part2   ... bench: 2,348,068,690 ns/iter (+/- 111,414,202)

*/

// allow bench feature when using unstable flag
// use: $ rustup run nightly cargo bench --features unstable
#![cfg_attr(feature = "unstable", feature(test))]

use aoc_import_magic::{import_magic, PuzzleOptions};
use std::{
    collections::HashMap,
    io,
};

const DAY: u32 = 15;
type InputTypeSingle = usize;
type InputType = Vec<InputTypeSingle>;
type OutputType = usize;
type TodaysPuzzleOptions = PuzzleOptions<InputType>;

fn parse_input(input: Vec<String>, _config: &HashMap<String, String>, _verbose: bool) -> InputType {
    input[0].split(",").into_iter().map(|item| item.parse().unwrap()).collect()
}

fn play_game(start_numbers: &InputType, end_turn: usize) -> OutputType {
    let mut spoken_numbers: HashMap<usize, (usize, Option<usize>)> = start_numbers
        .iter()
        .enumerate()
        .map(|(idx, number)| (*number, (idx+1, None)))
        .collect();

    let mut last_number = *start_numbers.last().unwrap();
    for turn in spoken_numbers.len()+1 ..= end_turn {
        let speak_number = match spoken_numbers.get(&last_number) {
            Some((_, None)) => {
                // spoken for the first time last round
                0
            }
            Some((last_turn, Some(turn_before))) => {
                // was already spoken before
                last_turn - turn_before
            }
            None => unreachable!(),
        };

        // modify the entry for the number we're about to speak
        spoken_numbers.entry(speak_number)
            // if if already exists, "shift" the turn numbers: The current last spoken turn for this number will become
            // the second last turn, and the current turn will become the last turn for this number.
            .and_modify(|(last_turn, second_last_turn)| {
                *second_last_turn = Some(*last_turn);
                *last_turn = turn;
            })
            // if no entry exists yet, insert the turn number
            .or_insert_with(|| (turn, None));


        //println!("turn {} | {}", turn, speak_number);
        //dbg!(&spoken_numbers);
        last_number = speak_number;
    }

    last_number
}

fn part1(po: &TodaysPuzzleOptions) -> OutputType {
    play_game(po.get_data(), 2020)
}

fn part2(po: &TodaysPuzzleOptions) -> OutputType {
    play_game(po.get_data(), 30000000)
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
    if !puzzle.skip_p1 {
        let res1 = part1(&puzzle);
        println!("Part 1 result: {}", res1);
    };

    let res2 = part2(&puzzle);
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

    fn test_case_helper(inputname: &str, sol1: Option<OutputType>, sol2: Option<OutputType>) {
        let po = import_helper(inputname);

        if let Some(sol1) = sol1 {
            let res1 = part1(&po);
            assert_eq!(sol1, res1, "part1");
        }

        if let Some(sol2) = sol2 {
            let res2 = part2(&po);
            assert_eq!(sol2, res2, "part2");
        }
    }

    #[test]
    fn example_1() {
        test_case_helper("example1", Some(436), Some(175594))
    }

    #[test]
    fn example_2() {
        test_case_helper("example2", Some(1), None)
    }

    #[test]
    fn example_3() {
        test_case_helper("example3", Some(10), None)
    }

    #[test]
    fn example_4() {
        test_case_helper("example4", Some(27), None)
    }

    #[test]
    fn example_5() {
        test_case_helper("example5", Some(78), None)
    }

    #[test]
    fn example_6() {
        test_case_helper("example6", Some(438), None)
    }

    #[test]
    fn example_7() {
        test_case_helper("example7", Some(1836), None)
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
        bb.iter(|| test::black_box(part2(&puzzle_options)));
    }
}
