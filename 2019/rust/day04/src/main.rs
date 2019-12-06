/*
      -------Part 1--------   -------Part 2--------
Day       Time  Rank  Score       Time  Rank  Score
  4   00:14:08  1964      0   00:38:56  2425      0

BENCHMARK RESULTS
test bench::bench_parsing ... bench:         116 ns/iter (+/- 10)
test bench::bench_part1   ... bench:   2,384,565 ns/iter (+/- 192,334)
test bench::bench_part2   ... bench:   8,036,241 ns/iter (+/- 705,061)
*/

// allow bench feature when using unstable flag
// use: $ cargo +nightly bench --features unstable
#![cfg_attr(feature = "unstable", feature(test))]

use aoc_import_magic::{import_magic, PuzzleOptions};
use std::{
    collections::HashMap,
    io,
};

const DAY: i32 = 4;
type InputType = (usize, usize);
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

fn is_correct_pw_1(pw: &usize) -> bool {
    let mut doubles = false;
    // avoiding strings 101
    let digits = [
        pw / 100_000,
        (pw % 100_000) / 10_000,
        (pw % 10_000) / 1_000,
        (pw % 1000) / 100,
        (pw % 100) / 10,
        (pw % 10) / 1,
    ];

    let mut iter = digits.windows(2);
    while let Some([aa, bb]) = iter.next() {
        if aa == bb {
            doubles = true;
        }

        if bb < aa {
            // early abort, we dont care for timing attacks :P
            return false;
        }
    }

    doubles
}

fn is_correct_pw_2(pw: &usize) -> bool {
    let mut multiples = HashMap::new();
    let digits = [
        pw / 100_000,
        (pw % 100_000) / 10_000,
        (pw % 10_000) / 1_000,
        (pw % 1000) / 100,
        (pw % 100) / 10,
        (pw % 10) / 1,
    ];

    let mut iter = digits.windows(2);
    while let Some([aa, bb]) = iter.next() {
        if aa == bb {
            let entry = multiples.entry(aa).or_insert(1);
            *entry += 1;
        }

        if bb < aa {
            // early abort, we dont care for timing attacks :P
            return false;
        }
    }

    for xx in multiples.values() {
        // we need just any pair
        if *xx == 2 {
            return true;
        }
    }

    false
}

fn parse_input(input: Vec<String>, _config: &HashMap<String, String>, _verbose: bool) -> InputType {
    let numbers: Vec<usize> = input[0]
        .split("-")
        .into_iter()
        .map(|xx| xx.parse::<usize>().unwrap())
        .collect();
    assert!(numbers.len() == 2);
    (numbers[0], numbers[1])
}

fn part1(po: &TodaysPuzzleOptions) -> OutputType1 {
    let (start, end) = po.data.as_ref().unwrap();
    (*start ..= *end).into_iter().filter(is_correct_pw_1).count()
}

fn part2(po: &TodaysPuzzleOptions, _res1: Option<OutputType1>) -> OutputType2 {
    let (start, end) = po.data.as_ref().unwrap();
    (*start ..= *end).into_iter().filter(is_correct_pw_2).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_import_magic::{import_magic_with_params, PuzzleOptions};

    pub(in super) fn import_helper(inputname: &str) -> PuzzleOptions<InputType> {
        let params = [
            "appname",
            "--input",
            inputname,
        ];
        import_magic_with_params(DAY, parse_input, &params).unwrap()
    }

    #[test]
    fn example_1() {
        assert!( is_correct_pw_1(&111111) )
    }

    #[test]
    fn example_2() {
        assert!( !is_correct_pw_1(&223450) )
    }

    #[test]
    fn example_3() {
        assert!( !is_correct_pw_1(&123789) )
    }

    #[test]
    fn example_4() {
        assert!( is_correct_pw_2(&112233) )
    }

    #[test]
    fn example_5() {
        assert!( !is_correct_pw_2(&123444) )
    }

    #[test]
    fn example_6() {
        assert!( is_correct_pw_2(&111122) )
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
