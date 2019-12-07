/*
      -------Part 1--------   -------Part 2--------
Day       Time  Rank  Score       Time  Rank  Score
  7   00:14:16   301      0   01:08:37   622      0

 (Restructured to use custom structs and same run_intcode function during cleanup after I solved
 both parts)

BENCHMARK RESULTS
test bench::bench_parsing ... bench:       6,056 ns/iter (+/- 776)
test bench::bench_part1   ... bench:     106,606 ns/iter (+/- 4,215)
test bench::bench_part2   ... bench:     302,674 ns/iter (+/- 25,617)
*/

// allow bench feature when using unstable flag
// use: $ cargo +nightly bench --features unstable
#![cfg_attr(feature = "unstable", feature(test))]

use aoc_import_magic::{import_magic, PuzzleOptions};
use permutohedron::Heap;
use std::{
    collections::{HashMap, VecDeque},
    convert::TryInto,
    io,
};

const DAY: i32 = 7;
type InputTypeSingle = i32;
type InputType = Vec<InputTypeSingle>;
type OutputType1 = i32;
type OutputType2 = OutputType1;
type TodaysPuzzleOptions = PuzzleOptions<InputType>;

#[derive(Debug)]
struct Controller {
    id: ControllerId,
    ip: usize,
    memory: Vec<i32>,
}

#[derive(Clone, Copy, Debug)]
struct ControllerId {
    id: usize,
}

#[derive(Debug)]
struct IOManager {
    ios: [VecDeque<i32>; 5]
}

impl IOManager {
    fn new(permutations: &Vec<i32>) -> IOManager {
        let mut iom = IOManager {
            ios: [
                VecDeque::new(),
                VecDeque::new(),
                VecDeque::new(),
                VecDeque::new(),
                VecDeque::new(),
            ],
        };

        iom.ios[0].push_back(permutations[0]);
        iom.ios[0].push_back(0);
        iom.ios[1].push_back(permutations[1]);
        iom.ios[2].push_back(permutations[2]);
        iom.ios[3].push_back(permutations[3]);
        iom.ios[4].push_back(permutations[4]);

        iom
    }

    fn load_next_input(&mut self, cid: ControllerId) -> Option<i32> {
        let idx_input = cid.id;
        self.ios[idx_input].pop_front()
    }

    fn save_output(&mut self, cid: ControllerId, out: i32) {
        let idx_output = match cid.id {
            id @ 0..=3 => id + 1,
            4 => 0,
            other => panic!("Invalid Controller id ({})", other),
        };

        self.ios[idx_output].push_back(out)
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

fn parse_input(input: Vec<String>, _config: &HashMap<String, String>, _verbose: bool) -> InputType {
    input[0]
        .split(",")
        .map(|xx| xx.parse::<InputTypeSingle>().unwrap())
        .collect()
}

fn part1(po: &TodaysPuzzleOptions) -> OutputType1 {
    let mut phases = vec![0, 1, 2, 3, 4];
    let mut permutations = Heap::new(&mut phases);

    let mut highest = 0;
    while let Some(permutations) = permutations.next_permutation() {
        let mut controllers = [
            Controller {
                id: ControllerId { id: 0 },
                ip: 0,
                memory: po.data.as_ref().unwrap().to_owned(),
            },
            Controller {
                id: ControllerId { id: 1 },
                ip: 0,
                memory: po.data.as_ref().unwrap().to_owned(),
            },
            Controller {
                id: ControllerId { id: 2},
                ip: 0,
                memory: po.data.as_ref().unwrap().to_owned(),
            },
            Controller {
                id: ControllerId { id: 3 },
                ip: 0,
                memory: po.data.as_ref().unwrap().to_owned(),
            },
            Controller {
                id: ControllerId { id: 4 },
                ip: 0,
                memory: po.data.as_ref().unwrap().to_owned(),
            },
        ];

        let mut io_manager = IOManager::new(permutations);

        let _aa = run_intcode_program(&mut controllers[0], &mut io_manager);
        let _bb = run_intcode_program(&mut controllers[1], &mut io_manager);
        let _cc = run_intcode_program(&mut controllers[2], &mut io_manager);
        let _dd = run_intcode_program(&mut controllers[3], &mut io_manager);
        let ee = run_intcode_program(&mut controllers[4], &mut io_manager).unwrap();

        highest = i32::max(highest, ee);
    }

    highest
}

fn part2(po: &TodaysPuzzleOptions, _res1: Option<OutputType1>) -> OutputType2 {
    let mut phases = vec![5, 6, 7, 8, 9];
    let mut permutations = Heap::new(&mut phases);
    let mut highest = 0;
    while let Some(permutations) = permutations.next_permutation() {
        let mut controllers = [
            Controller {
                id: ControllerId { id: 0 },
                ip: 0,
                memory: po.data.as_ref().unwrap().to_owned(),
            },
            Controller {
                id: ControllerId { id: 1 },
                ip: 0,
                memory: po.data.as_ref().unwrap().to_owned(),
            },
            Controller {
                id: ControllerId { id: 2},
                ip: 0,
                memory: po.data.as_ref().unwrap().to_owned(),
            },
            Controller {
                id: ControllerId { id: 3 },
                ip: 0,
                memory: po.data.as_ref().unwrap().to_owned(),
            },
            Controller {
                id: ControllerId { id: 4 },
                ip: 0,
                memory: po.data.as_ref().unwrap().to_owned(),
            },
        ];

        let mut io_manager = IOManager::new(permutations);

        loop {
            let _aa = run_intcode_program(&mut controllers[0], &mut io_manager);
            let _bb = run_intcode_program(&mut controllers[1], &mut io_manager);
            let _cc = run_intcode_program(&mut controllers[2], &mut io_manager);
            let _dd = run_intcode_program(&mut controllers[3], &mut io_manager);
            if let Some(ee) = run_intcode_program(&mut controllers[4], &mut io_manager) {
                highest = i32::max(highest, ee);
                break;
            }
        }
    }

    highest
}

fn run_intcode_program(controller: &mut Controller, io_manager: &mut IOManager) -> Option<OutputType1> {
    let mut output = 0;
    let memory = &mut controller.memory;
    let ip = &mut controller.ip;
    let cid = controller.id;

    loop {
        let modes = [
            memory[*ip] / 10_000,
            (memory[*ip] % 10_000) / 1_000,
            (memory[*ip] % 1_000) / 100,
        ];

        // Tbh, I created these three closures when cleaning up the code in the evening.
        // In my original solution I had copy & pasted these `match modes[ip] {}` blocks all over
        // the place (and fortunately changed all offsets correctly on first try :D)
        let get_mode_idx = |param_idx| match param_idx {
            1 => 2,
            2 => 1,
            3 => 0,
            _ => panic!("Invalid mode idx"),
        };

        let get_value_of_parameter = |param_idx| {
            let mode_idx = get_mode_idx(param_idx);

            match modes[mode_idx] {
                0 => {
                    let addr: usize = memory[*ip + param_idx].try_into().unwrap();
                    memory[addr]
                }
                1 => memory[*ip + param_idx],
                other => panic!("get_value_of_parameter: Invalid mode ({})", other),
            }
        };

        let get_addr_from_param = |param_idx| {
            let mode_idx = get_mode_idx(param_idx);

            match modes[mode_idx] {
                0 => {
                    memory[*ip + param_idx] as usize
                }
                other => panic!("get_addr_from_param: Invalid mode ({})", other),
            }
        };

        //println!("memory[{}] = {}", ip, memory[ip]);
        *ip += match memory[*ip] % 100 {
            1 => {
                // add
                let param_1 = get_value_of_parameter(1);
                let param_2 = get_value_of_parameter(2);

                let dst = get_addr_from_param(3);
                memory[dst] = param_1 + param_2;

                4
            }
            2 => {
                // multiply
                let param_1 = get_value_of_parameter(1);
                let param_2 = get_value_of_parameter(2);

                let dst = get_addr_from_param(3);
                memory[dst] = param_1 * param_2;

                4
            }
            3 => {
                // store input
                let addr = get_addr_from_param(1);
                match io_manager.load_next_input(cid) {
                    Some(input) => memory[addr] = input,
                    None => break None,
                }

                2
            }
            4 => {
                // send output
                output = get_value_of_parameter(1);
                io_manager.save_output(cid, output);

                2
            }
            5 => {
                // jump if true
                let param_1 = get_value_of_parameter(1);
                let param_2 = get_value_of_parameter(2);

                if param_1 != 0 {
                    *ip = param_2.try_into().unwrap();
                    0
                } else {
                    3
                }
            }
            6 => {
                // jump if false
                let param_1 = get_value_of_parameter(1);
                let param_2 = get_value_of_parameter(2);

                if param_1 == 0 {
                    *ip = param_2.try_into().unwrap();
                    0
                } else {
                    3
                }
            }
            7 => {
                // less than
                let param_1 = get_value_of_parameter(1);
                let param_2 = get_value_of_parameter(2);

                let addr = get_addr_from_param(3);
                memory[addr] = if param_1 < param_2 { 1 } else { 0 };

                4
            }
            8 => {
                // less than
                let param_1 = get_value_of_parameter(1);
                let param_2 = get_value_of_parameter(2);

                let addr = get_addr_from_param(3);
                memory[addr] = if param_1 == param_2 { 1 } else { 0 };

                4
            }
            99 => {
                break Some(output);
            }
            other => {
                panic!(
                    "Invalid opcode {} @ {} ({})",
                    other, ip, memory[*ip]
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_import_magic::{import_magic_with_params, PuzzleOptions};

    #[allow(dead_code)]
    pub(in super) fn import_helper(inputname: &str) -> PuzzleOptions<InputType> {
        let params = ["appname", "--input", inputname];
        import_magic_with_params(DAY, parse_input, &params).unwrap()
    }

    #[test]
    fn example_1() {
        let puzzle_options = PuzzleOptions {
            day: DAY,
            data: Some(vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0]),
            ..Default::default()
        };

        assert_eq!(part1(&puzzle_options), 43210);
    }

    #[test]
    fn example_2() {
        let puzzle_options = PuzzleOptions {
            day: DAY,
            data: Some(vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,
101,5,23,23,1,24,23,23,4,23,99,0,0]),
            ..Default::default()
        };

        assert_eq!(part1(&puzzle_options), 54321);
    }

    #[test]
    fn example_3() {
        let puzzle_options = PuzzleOptions {
            day: DAY,
            data: Some(vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0]),
            ..Default::default()
        };

        assert_eq!(part1(&puzzle_options), 65210);
    }

    #[test]
    fn example_4() {
        let puzzle_options = PuzzleOptions {
            day: DAY,
            data: Some(vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5]),
            ..Default::default()
        };

        assert_eq!(part2(&puzzle_options, None), 139629729);
    }

    #[test]
    fn example_5() {
        let puzzle_options = PuzzleOptions {
            day: DAY,
            data: Some(vec![3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10]),
            ..Default::default()
        };

        assert_eq!(part2(&puzzle_options, None), 18216);
    }
}

#[cfg(all(feature = "unstable", test))]
mod bench {
    extern crate test;

    use super::*;
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };
    use test::Bencher;

    fn helper_read_file(fname: &str) -> Vec<String> {
        BufReader::new(File::open(fname).unwrap()).lines().map(|line| line.unwrap()).collect()
    }

    #[bench]
    fn bench_parsing(bb: &mut Bencher) {
        let input = helper_read_file(&format!("../../_inputs/day{:02}/real1.input", DAY));
        bb.iter(|| parse_input(input.to_owned(), &HashMap::new(), false));
    }

    #[bench]
    fn bench_part1(bb: &mut Bencher) {
        let puzzle_options = tests::import_helper("real1");
        bb.iter(|| part1(&puzzle_options));
    }

    #[bench]
    fn bench_part2(bb: &mut Bencher) {
        let puzzle_options = tests::import_helper("real1");
        bb.iter(|| part2(&puzzle_options, None));
    }
}
