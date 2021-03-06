/*

BENCHMARK RESULTS

*/

// allow bench feature when using unstable flag
// use: $ rustup run nightly cargo bench --features unstable
#![cfg_attr(feature = "unstable", feature(test))]

use aoc_import_magic::{import_magic, PuzzleOptions};
use regex::Regex;
use std::{
    collections::{HashMap, VecDeque},
    io,
};

const DAY: i32 = 16;
type InputTypeSingle = isize;
type InputType = Vec<InputTypeSingle>;
type OutputType1 = String;
type OutputType2 = String;
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

fn parse_input(input: Vec<String>, config: &HashMap<String, String>, verbose: bool) -> InputType {
    // PARSE input
    input[0].chars().map(|cc| cc.to_digit(10).unwrap() as isize).collect()
}

fn part1(po: &TodaysPuzzleOptions) -> OutputType1 {
    let mut signal = po.data.as_ref().unwrap().to_owned();
    let signal = fft(&signal);

    format!("{}{}{}{}{}{}{}{}", signal[0], signal[1], signal[2], signal[3], signal[4], signal[5], signal[6], signal[7])
}

fn fft(signal: &Vec<isize>) -> Vec<isize> {
    let mut signal = signal.to_owned();

    for __ in 1 ..= 100 {
        let mut res = Vec::new();
        for nn in 1 ..= signal.len() {
            let tmp = gen_fft_sequence(nn).zip(signal.iter()).map(|(aa, bb)| aa * bb).sum::<isize>();
            let tmp = tmp % 10;
            let tmp = tmp.abs();
            res.push(tmp);
        }
        signal = res;
    }

    signal
}

fn fft2(signal: &Vec<isize>) -> Vec<isize> {
    let real_length = signal.len() * 10000;
    let mut signal: Vec<isize> = signal.iter().cycle().take(real_length).map(|xx| *xx).collect();

    for ii in 1 ..= 100 {
        let mut res = Vec::new();
        for nn in 1 ..= signal.len() {
            let tmp = gen_fft_sequence(nn).zip(signal.iter()).map(|(aa, bb)| aa * bb).sum::<isize>();
            let tmp = tmp % 10;
            let tmp = tmp.abs();
            res.push(tmp);
            print!(".");
        }
        signal = res;
        println!("| iteration {} done", ii);
    }

    signal
}

fn gen_fft_sequence(nn: usize) -> std::iter::Skip<std::iter::Cycle<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Take<std::iter::Repeat<isize>>, std::iter::Take<std::iter::Repeat<isize>>>, std::iter::Take<std::iter::Repeat<isize>>>, std::iter::Take<std::iter::Repeat<isize>>>>> {
    std::iter::repeat(0)
        .take(nn)
        .chain(
            std::iter::repeat(1)
                .take(nn)
        )
        .chain(
            std::iter::repeat(0)
                .take(nn)
        )
        .chain(
            std::iter::repeat(-1)
                .take(nn)
        )
        .cycle()
        .skip(1)
}

fn part2(po: &TodaysPuzzleOptions, _res1: Option<OutputType1>) -> OutputType2 {
    let signal = po.data.as_ref().unwrap().to_owned();
    let offset = (signal[6]
        + signal[5] * 10
        + signal[4] * 100
        + signal[3] * 1_000
        + signal[2] * 10_000
        + signal[1] * 100_000
        + signal[0] * 1_000_000)
        as usize;

    let result: Vec<isize> = fft2(&signal).into_iter().cycle().skip(offset).take(8).collect();
    format!("{}{}{}{}{}{}{}{}", result[0], result[1], result[2], result[3], result[4], result[5], result[6], result[7])
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
    fn example_2() {
        let po = import_helper("example2");
        assert_eq!(part1(&po), "24176176".to_string())
    }

    #[test]
    fn example_3() {
        let po = import_helper("example3");
        assert_eq!(part1(&po), "73745418".to_string())
    }

    #[test]
    fn example_4() {
        let po = import_helper("example4");
        assert_eq!(part1(&po), "52432133".to_string())
    }

    #[test]
    fn example_5() {
        let po = import_helper("example5");
        assert_eq!(part2(&po, None), "84462026".to_string())
    }

    #[test]
    fn example_6() {
        let po = import_helper("example6");
        assert_eq!(part2(&po, None), "78725270".to_string())
    }

    #[test]
    fn example_7() {
        let po = import_helper("example7");
        assert_eq!(part2(&po, None), "53553731".to_string())
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
