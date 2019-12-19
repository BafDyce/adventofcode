/*
      -------Part 1--------   -------Part 2--------
Day       Time  Rank  Score       Time  Rank  Score
 14   01:30:09   763      0   02:52:56   953      0

BENCHMARK RESULTS

*/

// allow bench feature when using unstable flag
// use: $ cargo +nightly bench --features unstable
#![cfg_attr(feature = "unstable", feature(test))]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

use aoc_import_magic::{import_magic, PuzzleOptions};
use regex::Regex;
use std::{
    collections::{HashMap, VecDeque},
    io,
};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
struct Chemical {
    amount: usize,
    name: String,
}

const DAY: i32 = 14;
type InputType = HashMap<String, (usize, Vec<Chemical>)>;
type OutputType1 = usize;
type OutputType2 = OutputType1;
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
    input
        .into_iter()
        .map(|line| {
            lazy_static! {
                static ref RE: Regex = Regex::new(
                    r"(\d+) ([[:alpha:]]+)"
                ).unwrap();
            }

            let mut items: Vec<Chemical>  = Vec::new();
            for caps in RE.captures_iter(&line) {
                items.push( Chemical{
                    name: caps[2].to_owned(),
                    amount: caps[1].parse().unwrap(),
                });
            }

            let Chemical {name, amount } = items.pop().unwrap();
            (name, (amount, items))
        })
        .collect()
}

fn do_transformation(
    amount: usize,
    name: &str,
    transformations: &InputType,
    leftovers: &mut HashMap<String, usize>,
) -> usize {
    //println!("need {} {}", amount, name);
    if name == "ORE" {
        return amount;
    }

    let target_leftovers = leftovers.entry(name.to_owned()).or_insert(0);
    //println!("leftovers for {}: {}", name, target_leftovers);
    if *target_leftovers >= amount {
        *target_leftovers -= amount;
        return 0;
    }

    // transformation necessary
    let (out_amount, reqs) = transformations.get(name).unwrap();
    let mut count = 0;
    let mut produced = *target_leftovers;
    while produced < amount {
        for chem in reqs {
            count += do_transformation(chem.amount, &chem.name, transformations, leftovers);
        }

        produced += out_amount;
    }

    //println!("produced {} x {} ({} needed)", produced, name, amount);

    if produced >= amount {
        let target_leftovers = leftovers.entry(name.to_owned()).or_insert(0);
        *target_leftovers = produced - amount;
        //println!("new leftovers for {}: {:?}", name, leftovers.get(name));
    }

    //println!("new leftovers for {}: {:?}", name, leftovers.get(name));

    return count;
}

fn do_transformation_no_new_ore(
    amount: usize,
    name: &str,
    transformations: &InputType,
    leftovers: &mut HashMap<String, usize>,
) -> bool {
    //println!("need {} {}", amount, name);
    if name == "ORE" {
        return false;
    }

    let target_leftovers = leftovers.entry(name.to_owned()).or_insert(0);
    //println!("leftovers for {}: {}", name, target_leftovers);
    if *target_leftovers >= amount {
        *target_leftovers -= amount;
        return true;
    }

    // transformation necessary
    let (out_amount, reqs) = transformations.get(name).unwrap();
    let mut success = true;
    let mut produced = *target_leftovers;
    while produced < amount {
        for chem in reqs {
            if ! do_transformation_no_new_ore(chem.amount, &chem.name, transformations, leftovers) {
                return false;
            }
        }

        produced += out_amount;
    }

    //println!("produced {} x {} ({} needed)", produced, name, amount);

    if produced >= amount {
        let target_leftovers = leftovers.entry(name.to_owned()).or_insert(0);
        *target_leftovers = produced - amount;
        //println!("new leftovers for {}: {:?}", name, leftovers.get(name));
    }

    //println!("new leftovers for {}: {:?}", name, leftovers.get(name));
    return true;
}


fn part1(po: &TodaysPuzzleOptions) -> OutputType1 {
    let transformations = po.data.as_ref().unwrap();
    //dbg!(&transformations);

    let mut leftovers = HashMap::new();
    do_transformation(1, "FUEL", transformations, &mut leftovers)
}

fn part2(po: &TodaysPuzzleOptions, res1: Option<OutputType1>) -> OutputType1 {
    part2_attempt3(po, res1)
}

fn part2_attempt3(po: &TodaysPuzzleOptions, res1: Option<OutputType1>) -> OutputType1 {
    let transformations = po.data.as_ref().unwrap();
    //dbg!(&transformations);

    let target = 1000000000000;

    let mut fuels = target / res1.unwrap();
    let mut size = fuels * 2;

    loop {
        let mut leftovers = HashMap::new();
        let ores = do_transformation(fuels, "FUEL", transformations, &mut leftovers);
        if ores == target {
            break fuels;
        } else if ores < target {
            fuels -= ((size - fuels) / 2);
        } else if ores > target {
            fuels += ((size - fuels) / 2);
        }

        size /= 2;
        println!("fuels: {} | size {}", fuels, size);
    }
}

// DOES NOT WORK
fn part2_attempt2(po: &TodaysPuzzleOptions, res1: Option<OutputType1>) -> OutputType1 {
    let transformations = po.data.as_ref().unwrap();
    //dbg!(&transformations);

    let pure_raw_requirements: HashMap<String, usize> = transformations
        .iter()
        .map(|(chemical, (amount, __))| {
            let mut leftovers = HashMap::new();
            (chemical.to_owned(), do_transformation(*amount, chemical, transformations, &mut leftovers))
        })
        .collect();

    let mut leftovers_initial_crafting = HashMap::new();
    let ore_reqs = do_transformation(1, "FUEL", transformations, &mut leftovers_initial_crafting);

    let mut ore_available = 1000000000000 - ore_reqs;
    let mut craftings_total = 1;

    loop {
        let craftings = ore_available / ore_reqs;
        println!("can craft {} times", craftings);
        if craftings < 1 {
            break;
        }
        ore_available -= (craftings * ore_reqs);
        craftings_total += craftings;

        // compute excess resources
        let mut total_excess_ore = 0;
        for (name, val) in leftovers_initial_crafting.iter() {
            let (amount, _) = transformations.get(name).unwrap();
            if val * craftings > *amount {
                let excess = (val * craftings) / amount;
                let excess_costs = pure_raw_requirements.get(name).unwrap() * excess;

                total_excess_ore += excess_costs;
            }
        }

        ore_available += total_excess_ore;
    }

    craftings_total
}

fn part2_attempt1(po: &TodaysPuzzleOptions, res1: Option<OutputType1>) -> OutputType2 {
    let transformations = po.data.as_ref().unwrap();

    let mut leftovers = HashMap::new();
    let mut total_ore_usage = 0;
    let mut count = 0;
    loop {
        let ore_used = do_transformation(1, "FUEL", transformations, &mut leftovers);
        total_ore_usage += ore_used;

        if total_ore_usage > 1000000000000 {
            break count;
        } else if total_ore_usage == 1000000000000 {
            break count + 1;
        }

        count += 1;
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

    fn test_case_helper(inputname: &str, sol1: OutputType1, sol2: OutputType2) {
        let po = import_helper(inputname);
        let res1 = part1(&po);
        assert_eq!(sol1, res1, "part1");
        let res2 = part2(&po, Some(res1));
        assert_eq!(sol2, res2, "part2");
    }

    #[test]
    fn example_1() {
        test_case_helper("example1", 31, 0)
    }

    #[test]
    fn example_2() {
        test_case_helper("example2", 165, 0)
    }

    #[test]
    fn example_3() {
        test_case_helper("example3", 13312, 82892753)
    }

    #[test]
    fn example_4() {
        test_case_helper("example4", 180697, 5586022)
    }

    #[test]
    fn example_5() {
        test_case_helper("example5", 2210736, 460664)
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
