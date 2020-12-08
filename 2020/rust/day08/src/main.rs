/*
      -------Part 1--------   -------Part 2--------
Day       Time  Rank  Score       Time  Rank  Score
  8   00:15:46  4222      0   00:54:11  5832      0

test bench::bench_parsing        ... bench:      64,744 ns/iter (+/- 2,801)
test bench::bench_part1          ... bench:      10,092 ns/iter (+/- 161)
test bench::bench_part2          ... bench:   1,962,547 ns/iter (+/- 19,544)
test bench::bench_part2_original ... bench:   8,511,750 ns/iter (+/- 1,373,254)

*/

// allow bench feature when using unstable flag
// use: $ rustup run nightly cargo bench --features unstable
#![cfg_attr(feature = "unstable", feature(test))]

use aoc_import_magic::{import_magic, PuzzleOptions};
use std::{
    collections::{HashMap, HashSet},
    io,
};

const DAY: u32 = 8;
type InputTypeSingle = Instruction;
type InputType = Vec<InputTypeSingle>;
type OutputType = i32;
type TodaysPuzzleOptions = PuzzleOptions<InputType>;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Instruction {
    op: Opcode,
    val: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Opcode {
    Acc,
    Jmp,
    Nop,
}

impl From<String> for Instruction {
    fn from(from: String) -> Instruction {
        let parts = from.split_whitespace().collect::<Vec<_>>();
        let opcode = match parts[0] {
            "acc" => Opcode::Acc,
            "jmp" => Opcode::Jmp,
            "nop" => Opcode::Nop,
            _ => panic!("Invalid instruction: {}", from),
        };

        Instruction {
            op: opcode,
            val: parts[1].parse().unwrap(),
        }
    }
}

fn parse_input(input: Vec<String>, _config: &HashMap<String, String>, _verbose: bool) -> InputType {
    input
        .into_iter()
        .map(Instruction::from)
        .collect()
}

// return value:
// .1: did it terminate?
// .2: value of acc after program has terminated or value of acc before first instruction was
//  executed twice
fn execute(program: &Vec<Instruction>) -> (bool, i32) {
    let mut acc = 0;
    let mut ip = 0;
    let mut ips_executed = HashSet::new();
    ips_executed.insert(0);

    while ip < program.len() {
        let instr = program[ip];
        let acc_before = acc;
        let ip_change = match instr.op {
            Opcode::Acc => {
                acc += instr.val;
                1
            }
            Opcode::Jmp => instr.val,
            Opcode::Nop => 1,
        };

        let ip_next = (ip as i32 + ip_change) as usize;
        if ips_executed.get(&ip_next).is_some() {
            return (false, acc_before);
        } else {
            ips_executed.insert(ip_next);
            ip = ip_next;
        }
    }

    (true, acc)
}

fn flip_instruction(instr: &mut Instruction) {
    instr.op = match instr.op {
        unchanged @ Opcode::Acc => unchanged,
        Opcode::Jmp => Opcode::Nop,
        Opcode::Nop => Opcode::Jmp,
    }
}

fn part1(po: &TodaysPuzzleOptions) -> OutputType {
    let program = po.get_data();

    execute(&program).1
}

fn part2(po: &TodaysPuzzleOptions) -> OutputType {
    let mut program = po.get_data().clone();

    for ii in 0..program.len() {
        let instr = &mut program[ii];
        if instr.op == Opcode::Acc {
            continue;
        }

        flip_instruction(instr);

        if let (true, result) = execute(&program) {
            return result;
        }

        // flip back before going to next one
        flip_instruction(&mut program[ii]);
    }

    0
}

fn part2_original(po: &TodaysPuzzleOptions) -> OutputType {
    let mut program = po.get_data().clone();
    let verbose = po.config.get("verbose").is_some();

    // calculate target locations
    // key = instruction index
    // value = (target instruction, target instruction if modified)
    let mut targets = HashMap::<usize, (usize, usize)>::new();
    for (idx, instr) in program.iter().enumerate() {
        let target = match &instr.op {
            Opcode::Acc | Opcode::Nop => idx + 1,
            Opcode::Jmp => (idx as i32 + instr.val) as usize,
        };

        let target_modified = match &instr.op {
            Opcode::Acc => target,
            Opcode::Jmp => idx + 1,
            Opcode::Nop => (idx as i32 + instr.val) as usize,
        };

        targets.insert(idx, (target, target_modified));
    }
    if verbose {
        println!("targets: {:?}", targets);
    }

    // run program once (so that we know which instructions are reachable from the start, without
    // modifications)
    let mut ip = 0;
    let mut ips_executed = HashSet::new();
    ips_executed.insert(0);

    loop {
        let instr = program[ip];
        let ip_change = match instr.op {
            Opcode::Acc | Opcode::Nop => 1,
            Opcode::Jmp => instr.val,
        };

        let ip_next = (ip as i32 + ip_change) as usize;
        if ips_executed.get(&ip_next).is_some() {
            break;
        } else {
            ips_executed.insert(ip_next);
            ip = ip_next;
        }
    }
    if verbose {
        println!("executed instructions (in loop): {:?}", ips_executed);
    }

    // which instructions can lead to goal?
    let mut can_reach = Vec::new();
    let end = program.len();
    for ii in 0..end {
        let target = targets.get(&ii).unwrap();

        if target.0 >= end {
            //println!("{:3} (unmod) ", ii);
            if !can_reach.contains(&ii) {
                can_reach.push(ii)
            }
        }

        if target.1 >= end {
            //println!("{:3} (if modified) ", ii);
            if !can_reach.contains(&ii) {
                can_reach.push(ii)
            }
        }
    }
    if verbose {
        println!("Instructions that can terminate: {:?}", can_reach);
    }

    // now, lets find the corrupted instruction
    let end = program.len();
    // we won't find the correct instruction on the first attempt, so we need to keep track of
    // previous suspects.
    let mut corruption_attempts = Vec::new();
    loop {
        let mut round = 2;
        // this endless loop has the label "find_corrupted". This allows us to easily jump out of it
        // even if we're in nested loops :)
        let corrupted = 'find_corrupted: loop {
            for ii in 0..end {
                let target = targets.get(&ii).unwrap();

                if can_reach.contains(&target.0) {
                    //println!("{:3} (unmod) ", ii);
                    if !can_reach.contains(&ii) {
                        can_reach.push(ii)
                    }
                }

                if can_reach.contains(&target.1) {
                    if ips_executed.contains(&ii) && !corruption_attempts.contains(&ii) {
                        if verbose {
                            println!("Corruption suspect: {}", ii);
                        }

                        // break out of loop and return ii (return value of the loop)
                        break 'find_corrupted ii;
                    }
                    //println!("{:3} (if modified) ", ii);
                    if !can_reach.contains(&ii) {
                        can_reach.push(ii)
                    }
                }
            }

            if verbose {
                println!("instructions that, eventually, terminate (after {} rounds): {:?}", round, can_reach);
            }
            round += 1;
        };

        // modify suspect
        program[corrupted].op = match program[corrupted].op {
            Opcode::Acc => Opcode::Acc,
            Opcode::Nop => Opcode::Jmp,
            Opcode::Jmp => Opcode::Nop,
        };
        // keep track of suspect
        corruption_attempts.push(corrupted);


        // execute program
        let mut acc = 0;
        let mut ip = 0;
        let mut ips_executed = HashSet::new();
        ips_executed.insert(0);

        'execute: while ip < program.len() {
            let instr = program[ip];
            let ip_change = match instr.op {
                Opcode::Acc => {
                    acc += instr.val;
                    1
                }
                Opcode::Jmp => instr.val,
                Opcode::Nop => 1,
            };

            let ip_next = (ip as i32 + ip_change) as usize;
            if ips_executed.get(&ip_next).is_some() {
                if verbose {
                    println!("Loop detected (for suspect {})", corrupted);
                }
                // change back
                program[corrupted].op = match program[corrupted].op {
                    Opcode::Acc => Opcode::Acc,
                    Opcode::Nop => Opcode::Jmp,
                    Opcode::Jmp => Opcode::Nop,
                };
                break 'execute
            } else {
                ips_executed.insert(ip_next);
                ip = ip_next;
            }
        }

        if ip >= program.len() {
            if verbose {
                println!("FINISHED");
            }
            break acc;
        }
    }
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
        test_case_helper("example1", 5, 8)
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
    fn bench_part2_original(bb: &mut Bencher) {
        let puzzle_options = tests::import_helper("real1");
        bb.iter(|| test::black_box(part2_original(&puzzle_options)));
    }
}
