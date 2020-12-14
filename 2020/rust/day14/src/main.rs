/*
      -------Part 1--------   -------Part 2--------
Day       Time  Rank  Score       Time  Rank  Score
 14   00:21:26  1726      0   00:32:01   726      0

test bench::bench_parsing ... bench:   1,354,265 ns/iter (+/- 39,668)
test bench::bench_part1   ... bench:      32,967 ns/iter (+/- 519)
test bench::bench_part2   ... bench:   6,543,627 ns/iter (+/- 813,649)

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
    collections::HashMap,
    io,
};

const DAY: u32 = 14;
type InputTypeSingle = Instruction;
type InputType = Vec<InputTypeSingle>;
type OutputType1 = usize;
type OutputType2 = OutputType1;
type TodaysPuzzleOptions = PuzzleOptions<InputType>;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
enum Instruction {
    Mask(Vec<char>),
    SetMem(usize, usize),
}

impl From<String> for Instruction {
    fn from(from: String) -> Instruction {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(mask = (?P<mask>[X01]+))|(mem\[(?P<address>\d+)\] = (?P<value>\d+))"
            ).unwrap();
        }

        let caps = RE.captures(&from).unwrap();
        println!("line: {}\ncaps: {:?}\n\n", from, caps);
        if let Some(mask) = caps.name("mask") {
            Instruction::Mask(mask.as_str().chars().collect())
        } else {
            Instruction::SetMem(
                caps["address"].parse().unwrap(),
                caps["value"].parse().unwrap(),
            )
        }
    }
}

fn parse_input(input: Vec<String>, _config: &HashMap<String, String>, _verbose: bool) -> InputType {
    input
        .into_iter()
        .map(Instruction::from)
        .collect()
}

fn part1(po: &TodaysPuzzleOptions) -> OutputType1 {
    let instructions = po.get_data();//.into_iter().count()

    let mut memory = HashMap::<usize, usize>::new();
    let mut active_mask = &vec![];

    for instr in instructions {
        match instr {
            Instruction::Mask(mask) => active_mask = mask,
            Instruction::SetMem(address, value) => {
                let mut masked_value = 0;
                let mut bit_value = 1;

                for bit in active_mask.iter().rev() {
                    match bit {
                        '1' => masked_value |= bit_value,
                        'X' => masked_value += bit_value & value,
                        '0' | _ => {},
                    }

                    bit_value *= 2;
                }

                memory.insert(*address, masked_value);
            }
        }
    }

    memory.values().sum()
}

fn part2(po: &TodaysPuzzleOptions) -> OutputType2 {
    let instructions = po.get_data();//.into_iter().count()

    let mut memory = HashMap::<usize, usize>::new();
    let mut active_mask = &vec![];

    for instr in instructions {
        match instr {
            Instruction::Mask(mask) => active_mask = mask,
            Instruction::SetMem(address, value) => {
                // store list of addresses. whenever we hit an X, we simply double the number of
                // addresses. For each existing address we create two new ones: One with a 1-bit,
                // and one with a 0-bit.
                // If there are many X's in a mask (e.g. first example), this would take a LOT of
                // time (and memory) but I checked my input and I didnt see any masks with an
                // excessive amount of X's (maybe 8-10 at most), so I decided this was a good enough
                // approach ;)
                let mut addresses = vec![0usize];
                let mut bit_value = 1;

                for bit in active_mask.iter().rev() {
                    match bit {
                        '0' => {
                            for addr in &mut addresses {
                                *addr |= bit_value & address;
                            }
                        }
                        '1' => {
                            for addr in &mut addresses {
                                *addr |= bit_value;
                            }
                        }
                        'X' => {
                            addresses = addresses.into_iter().map(|addr| {
                                vec![
                                    addr,
                                    addr | bit_value
                                ]
                            }).flatten().collect();
                        }
                        _ => {},
                    }

                    bit_value *= 2;
                }

                for address in addresses {
                    memory.insert(address, *value);
                }
            }
        }
    }
    memory.values().sum()
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
        test_case_helper("example1", Some(165), None)
    }

    #[test]
    fn example_2() {
        test_case_helper("example2", None, Some(208))
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
