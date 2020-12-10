/*
      -------Part 1--------   -------Part 2--------
Day       Time  Rank  Score       Time  Rank  Score
 10   00:08:55  1774      0   00:27:26  1237      0

test bench::bench_parsing ... bench:       3,658 ns/iter (+/- 116)
test bench::bench_part1   ... bench:         997 ns/iter (+/- 51)
test bench::bench_part2   ... bench:      28,402 ns/iter (+/- 229)

*/

// allow bench feature when using unstable flag
// use: $ rustup run nightly cargo bench --features unstable
#![cfg_attr(feature = "unstable", feature(test))]

use aoc_import_magic::{import_magic, PuzzleOptions};
use std::{
    collections::{HashMap, HashSet},
    io,
};

const DAY: u32 = 10;
type InputTypeSingle = usize;
type InputType = Vec<InputTypeSingle>;
type OutputType1 = usize;
type OutputType2 = OutputType1;
type TodaysPuzzleOptions = PuzzleOptions<InputType>;


fn parse_input(input: Vec<String>, _config: &HashMap<String, String>, _verbose: bool) -> InputType {
    // PARSE input
    input
        .into_iter()
        .map(|line| {
            line.parse::<InputTypeSingle>().unwrap_or_default()
        })
        .collect()
}

fn part1(po: &TodaysPuzzleOptions) -> OutputType1 {
    // just sort the list, then we can go through it and check the distances to the next pair
    let mut adapters = po.get_data().clone();
    // ensure that we also have the beginning and end
    adapters.push(0);
    adapters.push(adapters.iter().max().unwrap() + 3);
    adapters.sort();

    let mut count_1 = 0;
    let mut count_3 = 0;

    for (prev, next) in adapters.iter().zip(adapters.iter().skip(1)) {
        match next - prev {
            // ignore (a diff of 0 should also be an error, i guess)
            0 | 2 => {},
            1 => count_1 += 1,
            3 => count_3 += 1,
            // abort if we encounter a major error. either we did something massively wrong or the
            // input is invalid
            wrong => panic!("wrong diff: {}", wrong),
        }
    }

    count_1 * count_3
}

fn part2(po: &TodaysPuzzleOptions, _res1: Option<OutputType1>) -> OutputType2 {
    let verbose = po.verbose;

    // create a hash set for the list of adapters
    let mut adapters = po.get_data().clone();
    adapters.push(0);
    adapters.push(adapters.iter().max().unwrap() + 3);
    let adapters: HashSet<usize> = adapters.into_iter().collect();

    // now we can use the hash set to check for each adapter how many valid adapter targets are
    // available. we'll store that info in connections
    let mut connections = HashMap::<usize, Vec<usize>>::new();
    for adapter in &adapters {
        let possible = (1..=3).filter_map(|increase| {
            adapters.get(&(adapter+increase))
        }).map(ToOwned::to_owned).collect();

        connections.insert(*adapter, possible);
    }

    if verbose {
        dbg!(&connections);
    }

    // now we know which adapter can connect to which other adapters
    // Therefore, we can start at the end and can count how many distinct ways there are for the
    // given adapter to reach the end. Since adapters follow a strict order, we can simply compute
    // this back from the end to the front.
    let mut count_connections = HashMap::<usize, usize>::new();

    let mut adapters_list = po.get_data().clone();
    adapters_list.push(0);
    adapters_list.sort();
    adapters_list.reverse();

    // our device has only one "connection" to the end. store this directly, so that we can directly
    // use unwrap() in the loop below
    count_connections.insert( adapters_list.first().unwrap() + 3, 1 );

    for adapter in adapters_list {
        let num_connections = match connections.get(&adapter) {
            Some(list) => {
                list.iter().map(|conn| {
                    count_connections.get(conn).unwrap()
                }).sum()
            }
            None => panic!("no connections for {}!", adapter),
        };
        count_connections.insert(adapter, num_connections);
    }

    if verbose {
        dbg!(&count_connections);
    }

    *count_connections.get(&0).unwrap()
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
        test_case_helper("example1", 35, 8)
    }

    #[test]
    fn example_2() {
        test_case_helper("example2", 220, 19208)
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
