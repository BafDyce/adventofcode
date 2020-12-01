/*

running 6 tests
test tests::example_1 ... ignored
test bench::bench_parsing       ... bench:       6,976 ns/iter (+/- 655)
test bench::bench_part1         ... bench:       8,473 ns/iter (+/- 373)
test bench::bench_part1_initial ... bench:       2,473 ns/iter (+/- 166)
test bench::bench_part2         ... bench:      27,134 ns/iter (+/- 1,552)
test bench::bench_part2_initial ... bench:     942,981 ns/iter (+/- 7,275)

*/

// allow bench feature when using unstable flag
// use: $ rustup run nightly cargo bench --features unstable
#![cfg_attr(feature = "unstable", feature(test))]


use aoc_import_magic::{import_magic, PuzzleOptions};
use std::{
    collections::{HashMap, HashSet},
    io,
};

const DAY: u32 = 1;
type InputTypeSingle = usize;
type InputType = Vec<InputTypeSingle>;
type OutputType = usize;
type TodaysPuzzleOptions = PuzzleOptions<InputType>;


fn main() -> Result<(), io::Error> {
    println!("AoC 2020 | Day {}", DAY);

    // This function is pure magic (see ../../aoc_import_magic/lib.rs) because it
    // 1. parses command line arguments
    // 2. reads the input file for the correct day
    // 3. uses `parse_input` as a parsing function
    // 4. returns a nice usable struct which contains everything which we need for the actual puzzle
    let puzzle = import_magic(DAY, parse_input)?;
    let _res1 = if puzzle.skip_p1 {
        None
    } else {
        let res1 = part1(&puzzle);
        println!("Part 1 result: {}", res1);
        Some(res1)
    };

    let res2 = part2(&puzzle);
    println!("Part 2 result: {}", res2);

    Ok(())
}

fn parse_input(input: Vec<String>, _config: &HashMap<String, String>, _verbose: bool) -> InputType {
    // PARSE input
    input
        .into_iter()
        .map(|line| {
            line.parse::<InputTypeSingle>().unwrap_or_default()
        })
        .collect()
}

/// "Improved" version. On paper, this algorithm is O(n), compared to O(n^2) for the initial
/// solution, however for my actual puzzle input it is almost 4 times as slow!?
fn part1(po: &TodaysPuzzleOptions) -> OutputType {
    let mut seen = HashSet::new();

    for number in po.data.as_ref().unwrap().iter() {
        if *number >= 2020 {
            continue;
        }

        let diff = 2020 - number;
        if seen.contains(&diff) {
            return *number * diff;
        } else {
            seen.insert(*number);
        }
    }

    0
}


/// More efficient solution
fn part2(po: &TodaysPuzzleOptions) -> OutputType {
    let numbers: HashSet<usize> = po.data.as_ref().unwrap().iter().map(ToOwned::to_owned).collect();

    for (idx, aa) in numbers.iter().enumerate() {
        for bb in numbers.iter().skip(idx + 1) {
            if aa + bb >= 2020 {
                continue;
            }

            let diff = 2020 - aa - bb;
            if numbers.contains(&diff) {
                return aa * bb * diff;
            }
        }
    }

    0
}

// Initial implementations (those that got me the stars at first)
mod initial {
    use super::{OutputType, TodaysPuzzleOptions};

    pub(super) fn part1(po: &TodaysPuzzleOptions) -> OutputType {
        for aa in po.data.as_ref().unwrap().iter() {
            for bb in po.data.as_ref().unwrap().iter() {
                if aa == bb {
                    continue;
                }

                if aa + bb == 2020 {
                    return aa * bb;
                }
            }
        }

        0
    }


    pub(super) fn part2(po: &TodaysPuzzleOptions) -> OutputType {
        for aa in po.data.as_ref().unwrap().iter() {
            for bb in po.data.as_ref().unwrap().iter() {
                for cc in po.data.as_ref().unwrap().iter() {
                    if aa == bb || aa == cc || bb == cc {
                        continue;
                    }

                    if aa + bb + cc == 2020 {
                        return aa * bb * cc;
                    }
                }

            }
        }

        0
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

    fn test_case_helper(inputname: &str, sol1: OutputType, sol2: OutputType) {
        let po = import_helper(inputname);
        let res1 = part1(&po);
        assert_eq!(sol1, res1, "part1");
        let res2 = part2(&po);
        assert_eq!(sol2, res2, "part2");
    }

    fn test_case_helper_initial(inputname: &str, sol1: OutputType, sol2: OutputType) {
        let po = import_helper(inputname);
        let res1 = initial::part1(&po);
        assert_eq!(sol1, res1, "part1");
        let res2 = initial::part2(&po);
        assert_eq!(sol2, res2, "part2");
    }

    #[test]
    fn example_1() {
        test_case_helper_initial("example1", 514579, 241861950);
        test_case_helper("example1", 514579, 241861950);
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

    #[bench]
    fn bench_part1_initial(bb: &mut Bencher) {
        let puzzle_options = tests::import_helper("real1");
        bb.iter(|| test::black_box(initial::part1(&puzzle_options)));
    }

    #[bench]
    fn bench_part2_initial(bb: &mut Bencher) {
        let puzzle_options = tests::import_helper("real1");
        bb.iter(|| test::black_box(initial::part2(&puzzle_options)));
    }
}
