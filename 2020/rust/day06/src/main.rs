/*
      -------Part 1--------   -------Part 2--------
Day       Time  Rank  Score       Time  Rank  Score
  6   00:06:05  1665      0   00:09:14   910      0

test bench::bench_parsing ... bench:     239,304 ns/iter (+/- 14,666)
test bench::bench_part1   ... bench:     372,026 ns/iter (+/- 4,027)
test bench::bench_part2   ... bench:     424,406 ns/iter (+/- 4,228)
*/

// allow bench feature when using unstable flag
// use: $ rustup run nightly cargo bench --features unstable
#![cfg_attr(feature = "unstable", feature(test))]

use aoc_import_magic::{import_magic, PuzzleOptions};
use std::{
    collections::{HashMap, HashSet},
    io,
};

const DAY: u32 = 6;
type InputType = Vec<Vec<String>>;
type OutputType = usize;
type TodaysPuzzleOptions = PuzzleOptions<InputType>;


fn parse_input(input: Vec<String>, _config: &HashMap<String, String>, _verbose: bool) -> InputType {
    input
        .join("\n")
        .split("\n\n")
        .map(|line| line.split('\n').map(ToOwned::to_owned).collect())
        .collect()
}

fn part1(po: &TodaysPuzzleOptions) -> OutputType {
    let mut count = 0;
    for group in po.get_data() {
        let mut questions: HashSet<char> = HashSet::new();

        for person in group {
            for answer in person.chars() {
                questions.insert(answer);
            }
        }

        count += questions.len();
    }

    count
}

fn part2(po: &TodaysPuzzleOptions) -> OutputType {
    let mut count = 0;
    for group in po.get_data() {
        let mut questions: HashMap<char, usize> = HashMap::new();

        for person in group {
            for answer in person.chars() {
                let entry = questions.entry(answer).or_insert(0);
                *entry += 1;
            }
        }

        for (_, cnt) in questions {
            if cnt == group.len() {
                count += 1;
            }
        }
    }

    count
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

    fn test_case_helper(inputname: &str, sol1: OutputType, sol2: OutputType) {
        let po = import_helper(inputname);
        let res1 = part1(&po);
        assert_eq!(sol1, res1, "part1");
        let res2 = part2(&po);
        assert_eq!(sol2, res2, "part2");
    }

    #[test]
    fn example_1() {
        test_case_helper("example1", 11, 6)
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
