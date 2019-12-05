/*
      -------Part 1--------   -------Part 2--------
Day       Time  Rank  Score       Time  Rank  Score
  5   00:24:38   384      0   00:36:18   390      0

(tests added after the fact)
*/

use aoc_import_magic::{import_magic, PuzzleOptions};
use std::{collections::HashMap, io};

const DAY: i32 = 5;
type InputTypeSingle = i32;
type InputType = Vec<InputTypeSingle>;
type OutputType1 = i32;
type OutputType2 = OutputType1;
type TodaysPuzzleOptions = PuzzleOptions<InputType>;

fn run_intcode_program(program: &Vec<i32>, start_input: i32) -> OutputType1 {
    let mut memory = program.to_owned();
    let input = start_input;
    let mut output = 0;

    // TODO: create macro/lambda to avoid copy & pasting of `match mode{}..` structure
    let mut next_opcode_idx = 0;
    loop {
        let modes = [
            memory[next_opcode_idx] / 10_000,
            (memory[next_opcode_idx] % 10_000) / 1_000,
            (memory[next_opcode_idx] % 1_000) / 100,
        ];
        next_opcode_idx += match memory[next_opcode_idx] % 10 {
            1 => {
                // add
                let nr_1 = match modes[2] {
                    0 => {
                        let src_1 = memory[next_opcode_idx + 1] as usize;
                        memory[src_1]
                    }
                    1 => memory[next_opcode_idx + 1],
                    other => panic!("Invalid mode ({})", other),
                };

                let nr_2 = match modes[1] {
                    0 => {
                        let src_2 = memory[next_opcode_idx + 2] as usize;
                        memory[src_2]
                    }
                    1 => memory[next_opcode_idx + 2],
                    other => panic!("Invalid mode ({})", other),
                };

                match modes[0] {
                    0 => {
                        let dst = memory[next_opcode_idx + 3] as usize;
                        memory[dst] = nr_1 + nr_2;
                    }
                    other => panic!("Invalid mode ({})", other),
                }

                4
            }
            2 => {
                // multiply
                let nr_1 = match modes[2] {
                    0 => {
                        let src_1 = memory[next_opcode_idx + 1] as usize;
                        memory[src_1]
                    }
                    1 => memory[next_opcode_idx + 1],
                    other => panic!("Invalid mode ({})", other),
                };

                let nr_2 = match modes[1] {
                    0 => {
                        let src_2 = memory[next_opcode_idx + 2] as usize;
                        memory[src_2]
                    }
                    1 => memory[next_opcode_idx + 2],
                    other => panic!("Invalid mode ({})", other),
                };

                match modes[0] {
                    0 => {
                        let dst = memory[next_opcode_idx + 3] as usize;
                        memory[dst] = nr_1 * nr_2;
                    }
                    other => panic!("Invalid mode ({})", other),
                }

                4
            }
            3 => {
                // store input
                match modes[2] {
                    0 => {
                        let addr = memory[next_opcode_idx + 1] as usize;
                        memory[addr] = input;
                    }
                    other => panic!("Invalid mode ({})", other),
                }

                2
            }
            4 => {
                // get output
                output = match modes[2] {
                    0 => {
                        let addr = memory[next_opcode_idx + 1] as usize;
                        memory[addr]
                    }
                    1 => memory[next_opcode_idx + 1],
                    other => panic!("Invalid mode ({} / {})", other, memory[next_opcode_idx]),
                };

                2
            }
            5 => {
                // jump if true
                let param_1 = match modes[2] {
                    0 => {
                        let addr = memory[next_opcode_idx + 1] as usize;
                        memory[addr]
                    }
                    1 => memory[next_opcode_idx + 1],
                    other => panic!("Invalid mode ({})", other),
                };

                let param_2 = match modes[1] {
                    0 => {
                        let addr = memory[next_opcode_idx + 2] as usize;
                        memory[addr]
                    }
                    1 => memory[next_opcode_idx + 2],
                    other => panic!("Invalid mode ({})", other),
                };

                if param_1 != 0 {
                    next_opcode_idx = param_2 as usize;
                    0
                } else {
                    3
                }
            }
            6 => {
                // jump if false
                let param_1 = match modes[2] {
                    0 => {
                        let addr = memory[next_opcode_idx + 1] as usize;
                        memory[addr]
                    }
                    1 => memory[next_opcode_idx + 1],
                    other => panic!("Invalid mode ({})", other),
                };

                let param_2 = match modes[1] {
                    0 => {
                        let addr = memory[next_opcode_idx + 2] as usize;
                        memory[addr]
                    }
                    1 => memory[next_opcode_idx + 2],
                    other => panic!("Invalid mode ({})", other),
                };

                if param_1 == 0 {
                    next_opcode_idx = param_2 as usize;
                    0
                } else {
                    3
                }
            }
            7 => {
                // less than
                let param_1 = match modes[2] {
                    0 => {
                        let addr = memory[next_opcode_idx + 1] as usize;
                        memory[addr]
                    }
                    1 => memory[next_opcode_idx + 1],
                    other => panic!("Invalid mode ({})", other),
                };

                let param_2 = match modes[1] {
                    0 => {
                        let addr = memory[next_opcode_idx + 2] as usize;
                        memory[addr]
                    }
                    1 => memory[next_opcode_idx + 2],
                    other => panic!("Invalid mode ({})", other),
                };

                let param_3 = match modes[0] {
                    0 => memory[next_opcode_idx + 3] as usize,
                    other => panic!("Invalid mode ({})", other),
                };

                memory[param_3] = if param_1 < param_2 { 1 } else { 0 };

                4
            }
            8 => {
                // less than
                let param_1 = match modes[2] {
                    0 => {
                        let addr = memory[next_opcode_idx + 1] as usize;
                        memory[addr]
                    }
                    1 => memory[next_opcode_idx + 1],
                    other => panic!("Invalid mode ({})", other),
                };

                let param_2 = match modes[1] {
                    0 => {
                        let addr = memory[next_opcode_idx + 2] as usize;
                        memory[addr]
                    }
                    1 => memory[next_opcode_idx + 2],
                    other => panic!("Invalid mode ({})", other),
                };

                let param_3 = match modes[0] {
                    0 => memory[next_opcode_idx + 3] as usize,
                    other => panic!("Invalid mode ({})", other),
                };

                memory[param_3] = if param_1 == param_2 { 1 } else { 0 };

                4
            }
            9 if memory[next_opcode_idx] == 99 => {
                break output;
            }
            other => {
                panic!(
                    "Invalid opcode {} @ {} ({})",
                    other, next_opcode_idx, memory[next_opcode_idx]
                );
            }
        }
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
    run_intcode_program(po.data.as_ref().unwrap(), 1)
}

fn part2(po: &TodaysPuzzleOptions, _res1: Option<OutputType1>) -> OutputType2 {
    run_intcode_program(po.data.as_ref().unwrap(), 5)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_import_magic::{import_magic_with_params, PuzzleOptions};

    fn import_helper(inputname: &str) -> PuzzleOptions<InputType> {
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
        let program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];

        assert_eq!(run_intcode_program(&program, 7), 0, "7");
        assert_eq!(run_intcode_program(&program, 8), 1, "8");
        assert_eq!(run_intcode_program(&program, 9), 0, "9");
    }

    #[test]
    fn example_2() {
        let program = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];

        assert_eq!(run_intcode_program(&program, 7), 1, "7");
        assert_eq!(run_intcode_program(&program, 8), 0, "8");
        assert_eq!(run_intcode_program(&program, 9), 0, "9");
    }

    #[test]
    fn example_3() {
        let program = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];

        assert_eq!(run_intcode_program(&program, 7), 0, "7");
        assert_eq!(run_intcode_program(&program, 8), 1, "8");
        assert_eq!(run_intcode_program(&program, 9), 0, "9");
    }

    #[test]
    fn example_4() {
        let program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];

        assert_eq!(run_intcode_program(&program, 7), 1, "7");
        assert_eq!(run_intcode_program(&program, 8), 0, "8");
        assert_eq!(run_intcode_program(&program, 9), 0, "9");
    }

    #[test]
    fn example_5() {
        let program = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];

        assert_eq!(run_intcode_program(&program, 0), 0, "0");
        assert_eq!(run_intcode_program(&program, 1), 1, "1");
    }

    #[test]
    fn example_6() {
        let program = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];

        assert_eq!(run_intcode_program(&program, 0), 0, "0");
        assert_eq!(run_intcode_program(&program, 1), 1, "1");
    }

    #[test]
    fn example_7() {
        let program = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];

        assert_eq!(run_intcode_program(&program, 7), 999, "7");
        assert_eq!(run_intcode_program(&program, 8), 1000, "8");
        assert_eq!(run_intcode_program(&program, 9), 1001, "9");
    }
}
