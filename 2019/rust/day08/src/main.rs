/*
      -------Part 1--------   -------Part 2--------
Day       Time  Rank  Score       Time  Rank  Score
  8   00:26:34  1822      0   00:42:17  1529      0

BENCHMARK RESULTS
test bench::bench_parsing ... bench:     192,960 ns/iter (+/- 25,411)
test bench::bench_part1   ... bench:       6,335 ns/iter (+/- 608)
test bench::bench_part2   ... bench:      14,519 ns/iter (+/- 2,155)
*/

// allow bench feature when using unstable flag
// use: $ cargo +nightly bench --features unstable
#![cfg_attr(feature = "unstable", feature(test))]

#[macro_use]
extern crate serde_derive;

use aoc_import_magic::{import_magic, PuzzleOptions};
use std::{collections::HashMap, fmt, io};

const DAY: i32 = 8;
const PART_2_MSG: &str = "Interpret the image above :)";
type InputType = Image;
type OutputType1 = usize;
type OutputType2 = &'static str;
type TodaysPuzzleOptions = PuzzleOptions<InputType>;

type Pixel = u32;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
struct Image {
    width: usize,
    height: usize,
    layers: Vec<Vec<Vec<Pixel>>>,
}

impl Image {
    pub fn new(width: usize, height: usize, raw_data: Vec<Pixel>) -> Self {
        Image {
            width,
            height,
            layers: {
                let mut data = Vec::new();
                let mut remaining = raw_data.to_owned();

                while !remaining.is_empty() {
                    let rest = remaining.split_off(height * width);

                    let mut layer_remaining = remaining;
                    let mut layer = Vec::new();
                    while !layer_remaining.is_empty() {
                        let rest = layer_remaining.split_off(width);
                        layer.push(layer_remaining);
                        layer_remaining = rest;
                    }

                    data.push(layer);
                    remaining = rest;
                }

                data
            },
        }
    }

    fn find_layer_with_most_zeros(&self) -> usize {
        let mut min = std::usize::MAX;
        let mut retval = std::usize::MAX;

        for (idx, layer) in self.layers.iter().enumerate() {
            let cnt = layer
                .iter()
                .map(|row| row.iter().filter(|pixel| **pixel == 0).count())
                .sum();
            if cnt < min {
                min = cnt;
                retval = idx;
            }
        }

        retval
    }

    fn multiply_stuff(&self, idx: usize) -> usize {
        let ones: usize = self.layers[idx]
            .iter()
            .map(|row| row.iter().filter(|&&xx| xx == 1).count())
            .sum();
        let twos: usize = self.layers[idx]
            .iter()
            .map(|row| row.iter().filter(|&&xx| xx == 2).count())
            .sum();

        ones * twos
    }

    fn get_top_pixel(&self, row: usize, col: usize) -> Pixel {
        match self.layers.iter().find(|layer| layer[row][col] != 2) {
            Some(layer) => layer[row][col],
            None => 0,
        }
    }
}

impl fmt::Display for Image {
    fn fmt(&self, ff: &mut fmt::Formatter<'_>) -> fmt::Result {
        for xx in 0..self.height {
            for yy in 0..self.width {
                let pixel = match self.get_top_pixel(xx, yy) {
                    0 => '.',
                    1 => '#',
                    _ => ' ',
                };
                write!(ff, "{}", pixel)?;
            }
            write!(ff, "\n")?;
        }

        Ok(())
    }
}

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

fn parse_input(input: Vec<String>, config: &HashMap<String, String>, _verbose: bool) -> InputType {
    // PARSE input
    let data = input[0]
        .chars()
        .map(|cc| match cc {
            digit if digit.is_ascii_digit() => digit.to_digit(10).unwrap(),
            other => panic!("error pixel {}", other),
        })
        .collect();

    let width = config.get("width").unwrap().parse().unwrap();
    let height = config.get("height").unwrap().parse().unwrap();

    Image::new(width, height, data)
}

fn part1(po: &TodaysPuzzleOptions) -> OutputType1 {
    let img = po.data.as_ref().unwrap();

    let idx = img.find_layer_with_most_zeros();
    img.multiply_stuff(idx)
}

fn part2(po: &TodaysPuzzleOptions, _res1: Option<OutputType1>) -> OutputType2 {
    println!("{}", po.data.as_ref().unwrap());
    PART_2_MSG
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
        test_case_helper("example1", 1, PART_2_MSG)
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
