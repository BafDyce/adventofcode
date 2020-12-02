/*

test bench::bench_parsing ... bench:     579,810 ns/iter (+/- 4,629)
test bench::bench_part1   ... bench:      12,873 ns/iter (+/- 124)
test bench::bench_part2   ... bench:      20,075 ns/iter (+/- 904)

*/

// allow bench feature when using unstable flag
// use: $ rustup run nightly cargo bench --features unstable
#![cfg_attr(feature = "unstable", feature(test))]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

use aoc_import_magic::{import_magic, PuzzleOptions};
use regex::Regex;
use std::{
    collections::{HashMap},
    io,
};

const DAY: u32 = 2;
type InputTypeSingle = Password;
type InputType = Vec<InputTypeSingle>;
type OutputType1 = usize;
type OutputType2 = OutputType1;
type TodaysPuzzleOptions = PuzzleOptions<InputType>;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
struct Password {
    letter: char,
    count_min: usize,
    count_max: usize,
    pw: String,
}

impl Password {
    fn is_valid(&self) -> bool {
        let count = self.pw.chars().filter(|&cc| cc == self.letter).count();

        count >= self.count_min && count <= self.count_max
    }

    fn is_valid_2(&self) -> bool {
        let check_1 = match self.pw.chars().nth(self.count_min - 1) {
            Some(cc) => cc == self.letter,
            None => false,
        };

        match self.pw.chars().nth(self.count_max - 1) {
            Some(cc) => {
                let check_2 = cc == self.letter;

                check_1 != check_2
            }
            None => check_1
        }
    }

}

impl From<String> for Password {
    fn from(from: String) -> Password {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?P<min>\d+)-(?P<max>\d+) (?P<char>.{1}): (?P<pw>.*)"
            ).unwrap();
        }

        let caps = RE.captures(&from).unwrap();
        Password {
            letter: caps["char"].chars().nth(0).unwrap(),
            count_min: caps["min"].parse().unwrap(),
            count_max: caps["max"].parse().unwrap(),
            pw: caps["pw"].to_string(),
        }
    }
}

fn parse_input(input: Vec<String>, _config: &HashMap<String, String>, _verbose: bool) -> InputType {
    // PARSE input
    input
        .into_iter()
        .map(Password::from)
        .collect()
}

fn part1(po: &TodaysPuzzleOptions) -> OutputType1 {
    po.data.as_ref().unwrap().iter().filter(|pw| pw.is_valid()).count()
}

fn part2(po: &TodaysPuzzleOptions, _res1: Option<OutputType1>) -> OutputType2 {
    po.data.as_ref().unwrap().iter().filter(|pw| pw.is_valid_2()).count()
}

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
        println!("Part 1 result: {}", res1);
        Some(res1)
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
        assert_eq!(sol1, res1, "part1");
        let res2 = part2(&po, Some(res1));
        assert_eq!(sol2, res2, "part2");
    }

    #[test]
    fn example_1() {
        test_case_helper("example1", 2, 1)
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
