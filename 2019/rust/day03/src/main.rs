/*
      -------Part 1--------   -------Part 2--------
Day       Time  Rank  Score       Time  Rank  Score
  3   00:29:06   928      0   00:37:23   754      0
*/

use aoc_import_magic::{import_magic, PuzzleOptions};
use std::{
    collections::HashMap,
    io,
};

const DAY: i32 = 3;
type InputTypeSingle = Vec<Dir>;
type InputType = Vec<InputTypeSingle>;
type OutputType1 = i32;
type OutputType2 = OutputType1;
type TodaysPuzzleOptions = PuzzleOptions<InputType>;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Dir {
    Up(usize),
    Right(usize),
    Down(usize),
    Left(usize),
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
    input
        .into_iter()
        .map(|line| {
            line
                .split(",")
                .into_iter()
                .map(|dir| {
                    match dir.chars().nth(0) {
                        Some('R') => Dir::Right(dir[1..].parse().unwrap()),
                        Some('L') => Dir::Left(dir[1..].parse().unwrap()),
                        Some('U') => Dir::Up(dir[1..].parse().unwrap()),
                        Some('D') => Dir::Down(dir[1..].parse().unwrap()),
                        _ => panic!("oO")
                    }
                })
                .collect()
        })
        .collect()
}

fn part1(po: &TodaysPuzzleOptions) -> OutputType1 {
    // HashMap< position, Vec(lines_present) >
    let mut grid = HashMap::<(i32, i32), Vec<usize>>::new();

    for (ln, line) in po.data.as_ref().unwrap().into_iter().enumerate() {
        let mut xx = 0i32;
        let mut yy = 0i32;

        for dir in line {
            match dir {
                Dir::Up(len) => {
                    for _ in 1 ..= *len {
                        xx += 1;
                        let entry = grid.entry((xx, yy)).or_insert(Vec::new());
                        if !entry.contains(&ln) {
                            entry.push(ln);
                        }
                    }
                },
                Dir::Right(len) => {
                    for _ in 1 ..= *len {
                        yy += 1;
                        let entry = grid.entry((xx, yy)).or_insert(Vec::new());
                        if !entry.contains(&ln) {
                            entry.push(ln);
                        }
                    }
                }
                Dir::Down(len) => {
                    for _ in 1 ..= *len {
                        xx -= 1;
                        let entry = grid.entry((xx, yy)).or_insert(Vec::new());
                        if !entry.contains(&ln) {
                            entry.push(ln);
                        }
                    }
                }
                Dir::Left(len) => {
                    for _ in 1 ..= *len {
                        yy -= 1;
                        let entry = grid.entry((xx, yy)).or_insert(Vec::new());
                        if !entry.contains(&ln) {
                            entry.push(ln);
                        }
                    }
                }
            }
        }
    }

    let mut manhatten = std::i32::MAX;
    for ((xx, yy), vv) in grid.into_iter() {
        if vv.len() >= 2 {
            let new_manhatten = xx.abs() + yy.abs();
            manhatten = i32::min(manhatten, new_manhatten);
        }
    }

    manhatten
}

fn part2(po: &TodaysPuzzleOptions, _res1: Option<OutputType1>) -> OutputType2 {
    // HashMap< position, HashMap< line, distance_to_that_point>  >
    let mut grid = HashMap::<(i32, i32), HashMap<usize, i32>>::new();

    for (ln, line) in po.data.as_ref().unwrap().into_iter().enumerate() {
        let mut xx = 0i32;
        let mut yy = 0i32;
        let mut distance = 0;

        for dir in line {
            match dir {
                Dir::Up(len) => {
                    for _ in 1 ..= *len {
                        xx += 1;
                        distance += 1;
                        let entry = grid.entry((xx, yy)).or_insert(HashMap::new());
                        let _inner = entry.entry(ln).or_insert(distance);
                    }
                }
                Dir::Right(len) => {
                    for _ in 1 ..= *len {
                        yy += 1;
                        distance += 1;
                        let entry = grid.entry((xx, yy)).or_insert(HashMap::new());
                        let _inner = entry.entry(ln).or_insert(distance);
                    }
                }
                Dir::Down(len) => {
                    for _ in 1 ..= *len {
                        xx -= 1;
                        distance += 1;
                        let entry = grid.entry((xx, yy)).or_insert(HashMap::new());
                        let _inner = entry.entry(ln).or_insert(distance);
                    }
                }
                Dir::Left(len) => {
                    for _ in 1 ..= *len {
                        yy -= 1;
                        distance += 1;
                        let entry = grid.entry((xx, yy)).or_insert(HashMap::new());
                        let _inner = entry.entry(ln).or_insert(distance);
                    }
                }
            }
        }
    }

    let mut min_steps = std::i32::MAX;
    for steps in grid.values() {
        if steps.len() < 2 {
            continue;
        }
        let steps = steps.values().sum();
        min_steps = i32::min(min_steps, steps);
    }

    min_steps
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_import_magic::{import_magic_with_params, PuzzleOptions};

    fn import_helper(inputname: &str) -> PuzzleOptions<InputType> {
        let params = [
            "appname",
            "--input",
            inputname,
        ];
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
        test_case_helper("example1", 6, 30)
    }

    #[test]
    fn example_2() {
        test_case_helper("example2", 159, 610)
    }

    #[test]
    fn example_3() {
        test_case_helper("example3", 135, 410)
    }
}
