use aoc_import_magic::{import_magic, PuzzleOptions};
use std::{
    collections::HashMap,
    convert::TryFrom,
    iter::successors,
    io,
};

const DAY: i32 = 1;
type InputTypeSingle = i32;
type InputType = Vec<InputTypeSingle>;
type OutputType1 = i32;
type OutputType2 = u32;
type TodaysPuzzleOptions = PuzzleOptions<InputType>;

//       -------Part 1--------   -------Part 2--------
// Day       Time  Rank  Score       Time  Rank  Score
//   1   00:01:52   205      0   00:19:51  1241      0

// runtime in release mode with real input:
// real    0.002673526s
// original version:
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
        // Thanks to /u/bsullio
        successors(u32::try_from(*mass).ok(), |mass| (mass / 3).checked_sub(2))
            .skip(1) // don't include the initial mass
            .sum::<u32>()

        /* // original code:
        let mut fuel_total = 0;

        let mut fuel = mass / 3 - 2;
        while fuel > 0 {
            fuel_total += fuel;
            fuel = fuel / 3 - 2;
        }

        fuel_total
        */
    }).sum()
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
    fn example_12() {
        test_case_helper("example_12", 2, 2)
    }

    #[test]
    fn example_14() {
        test_case_helper("example_14", 2, 2)
    }

    #[test]
    fn example_1969() {
        test_case_helper("example_1969", 654, 966)
    }

    #[test]
    fn example_100756() {
        test_case_helper("example_100756", 33583, 50346)
    }
}