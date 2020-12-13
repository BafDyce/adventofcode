/*

BENCHMARK RESULTS

*/

// allow bench feature when using unstable flag
// use: $ rustup run nightly cargo bench --features unstable
#![cfg_attr(feature = "unstable", feature(test))]

use aoc_import_magic::{import_magic, PuzzleOptions};
use ring_algorithm::chinese_remainder_theorem;
use std::{
    collections::{HashMap},
    io,
};

const DAY: u32 = 13;
type InputType = Data;
type OutputType1 = usize;
type OutputType2 = isize;
type TodaysPuzzleOptions = PuzzleOptions<InputType>;

#[derive(Clone, Debug, Default, PartialEq)]
struct Data {
    timestamp: usize,
    buses: Vec<Option<usize>>,
}

fn parse_input(input: Vec<String>, _config: &HashMap<String, String>, _verbose: bool) -> InputType {
    let mut iter = input.into_iter();

    let timestamp = iter.next().unwrap().parse().unwrap();
    let buses = iter.next().unwrap().split(",").map(|bus| {
        bus.parse::<usize>().ok()
    }).collect();

    Data {
        timestamp,
        buses,
    }
}

fn part1(po: &TodaysPuzzleOptions) -> OutputType1 {
    let data = po.get_data();
    let mut waiting_times = HashMap::new();

    for bus in data.buses.iter().filter_map(|&bus| bus) {
        let waiting = bus - (data.timestamp % bus);
        waiting_times.insert(bus, waiting);
    }

    let bus = waiting_times.into_iter().min_by_key(|(_bus, waiting)| *waiting).unwrap();
    bus.0 * bus.1
}

fn part2(po: &TodaysPuzzleOptions) -> OutputType2 {
    let data = po.get_data();

    let (buses, offsets): (Vec<_>, Vec<_>) = data.buses.iter().enumerate().filter_map(|(idx, bus)| {
        match bus {
            Some(bus) => {
                Some((*bus as isize, -(idx as isize)))
            },
            None => None,
        }
    }).unzip();

    let buses_product: isize = buses.iter().product();

    let mut result: isize = chinese_remainder_theorem(
        &offsets,
        &buses,
    ).unwrap();

    while result < 0 {
        result += buses_product;
    }

    result
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

    fn test_case_helper(inputname: &str, sol1: Option<OutputType1>, sol2: Option<OutputType2>) {
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
        test_case_helper("example1", Some(295), Some(1068781))
    }

    #[test]
    fn example_2() {
        test_case_helper("example2", None, Some(3417))
    }

    #[test]
    fn example_3() {
        test_case_helper("example3", None, Some(754018))
    }

    #[test]
    fn example_4() {
        test_case_helper("example4", None, Some(779210))
    }

    #[test]
    fn example_5() {
        test_case_helper("example5", None, Some(1261476))
    }

    #[test]
    fn example_6() {
        test_case_helper("example6", None, Some(1202161486))
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
