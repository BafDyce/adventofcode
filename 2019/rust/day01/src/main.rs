use aoc_import_magic::{import_magic, PuzzleOptions};
use std::{
    collections::HashMap,
    io,
};

const DAY: i32 = 1;
type InputTypeSingle = i32;
type InputType = Vec<InputTypeSingle>;
type OutputType1 = i32;
type OutputType2 = OutputType1;
type TodaysPuzzleOptions = PuzzleOptions<InputType>;

//       -------Part 1--------   -------Part 2--------
// Day       Time  Rank  Score       Time  Rank  Score
//   1   00:01:52   205      0   00:19:51  1241      0

// runtime in release mode with real input:
// real    0.003070799s

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
    // PARSE input
    input
        .into_iter()
        .map(|line| {
            line.parse::<InputTypeSingle>().unwrap_or_default()
        })
        .collect()
}

fn part1(po: &TodaysPuzzleOptions) -> OutputType1 {
    po.data.as_ref().unwrap().into_iter().map(|mass| mass / 3 - 2).sum()
}

fn part2(po: &TodaysPuzzleOptions, _res1: Option<OutputType1>) -> OutputType2 {
    po.data.as_ref().unwrap().into_iter().map(|mass| {
        let mut fuel_total = 0;

        let mut fuel = mass / 3 - 2;
        while fuel > 0 {
            fuel_total += fuel;
            fuel = fuel / 3 - 2;
        }

        fuel_total
    }).sum()
}
