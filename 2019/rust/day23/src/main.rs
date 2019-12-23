/*
      -------Part 1--------   --------Part 2--------
Day       Time  Rank  Score       Time   Rank  Score
 23   10:17:40  2164      0   10:30:57   1963      0
(very late start though, started about 9hrs 30 minutes late (so it took me about an hour))
BENCHMARK RESULTS
test bench::bench_parsing ... bench:     996,594 ns/iter (+/- 49,391)
test bench::bench_part1   ... bench:  11,859,503 ns/iter (+/- 328,369)
test bench::bench_part2   ... bench: 692,732,895 ns/iter (+/- 27,711,520)
*/

// allow bench feature when using unstable flag
// use: $ rustup run nightly cargo bench --features unstable
#![cfg_attr(feature = "unstable", feature(test))]

mod intcode;
use intcode::*;

use aoc_import_magic::{import_magic, PuzzleOptions};
use std::{
    collections::{HashMap, VecDeque},
    io,
};

const DAY: i32 = 23;
type InputTypeSingle = IntcodeNumber;
type InputType = Vec<InputTypeSingle>;
type OutputType1 = IntcodeNumber;
type OutputType2 = OutputType1;
type TodaysPuzzleOptions = PuzzleOptions<InputType>;

#[derive(Debug, Default, PartialEq)]
struct Nic {
    cpu: IntcodeProcessor,
    inputs: InputController,
    outputs: VecDeque<IntcodeNumber>,
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

fn parse_input(input: Vec<String>, _config: &HashMap<String, String>, _verbose: bool) -> InputType {
    input[0]
        .split(",")
        .map(|xx| xx.parse::<InputTypeSingle>().unwrap())
        .collect()
}

fn part1(po: &TodaysPuzzleOptions) -> OutputType1 {
    let program = po.data.as_ref().unwrap();

    let mut nics: Vec<Nic> = (0 ..= 49).into_iter().map(|addr| {
        Nic {
            cpu: IntcodeProcessor::new(program),
            inputs: InputController::new(vec![addr]),
            outputs: VecDeque::new(),
        }
    }).collect();

    loop {
        for idx in 0 .. nics.len() {
            let nic = &mut nics[idx];

            nic.cpu.exec_single(&mut nic.inputs, &mut nic.outputs);
            if nic.outputs.len() == 3 {
                let target = nic.outputs.pop_front().unwrap();
                let val_1 = nic.outputs.pop_front().unwrap();
                let val_2 = nic.outputs.pop_front().unwrap();

                if target == 255 {
                    return val_2;
                }

                nics[target as usize].inputs.add(val_1);
                nics[target as usize].inputs.add(val_2);
            }
        }
    }
}

fn are_nics_idling(nics: &Vec<Nic>) -> bool {
    nics.iter().all(|nic| {
        nic.inputs.is_empty() && nic.inputs.has_failed_reads()
    })
}

fn part2(po: &TodaysPuzzleOptions, _res1: Option<OutputType1>) -> OutputType2 {
    let program = po.data.as_ref().unwrap();

    let mut nics: Vec<Nic> = (0 ..= 49).into_iter().map(|addr| {
        Nic {
            cpu: IntcodeProcessor::new(program),
            inputs: InputController::new(vec![addr]),
            outputs: VecDeque::new(),
        }
    }).collect();

    let mut nat = (0, 0);
    let mut delivered_ys = Vec::new();

    loop {
        for idx in 0 .. nics.len() {
            let nic = &mut nics[idx];

            nic.cpu.exec_single(&mut nic.inputs, &mut nic.outputs);
            if nic.outputs.len() == 3 {
                let target = nic.outputs.pop_front().unwrap();
                let val_1 = nic.outputs.pop_front().unwrap();
                let val_2 = nic.outputs.pop_front().unwrap();

                if target == 255 {
                    nat = (val_1, val_2);
                } else {
                    nics[target as usize].inputs.add(val_1);
                    nics[target as usize].inputs.add(val_2);
                }
            }
        }

        if are_nics_idling(&nics) {
            let (xx, yy) = nat;
            nics[0].inputs.add(xx);
            nics[0].inputs.add(yy);

            if delivered_ys.contains(&yy) {
                return yy;
            }
            delivered_ys.push(yy);
        }
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
