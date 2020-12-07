/*
      -------Part 1--------   -------Part 2--------
Day       Time  Rank  Score       Time  Rank  Score
  7   00:50:06  4551      0   01:02:18  3435      0

test bench::bench_parsing ... bench:   1,677,676 ns/iter (+/- 94,926)
test bench::bench_part1   ... bench:     976,453 ns/iter (+/- 15,253)
test bench::bench_part2   ... bench:     124,806 ns/iter (+/- 2,492)

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

const DAY: u32 = 7;
type InputTypeSingle = Bag;
type InputType = Vec<InputTypeSingle>;
type OutputType = usize;
type TodaysPuzzleOptions = PuzzleOptions<InputType>;

// prefer structs with named members over tuples :P
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
struct Bag {
    color: String,
    inner: Vec<InnerBag>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
struct InnerBag {
    num: usize,
    color: String,
}


fn parse_input(input: Vec<String>, _config: &HashMap<String, String>, _verbose: bool) -> InputType {
    input
        .into_iter()
        .map(|line| {
            lazy_static! {
                // First I tried to create a regex which matches everything in "rest" in repeated
                // groups, but I couldnt find out how.. So I went this split approach instead.
                static ref RE: Regex = Regex::new(
                    r"(?P<outer_color>.*?) bags contain (?P<rest>.*)\."
                ).unwrap();
            }

            let caps = RE.captures(&line).unwrap();
            let outer_color = &caps["outer_color"];

            let rest = &caps["rest"];
                // "no other bags" and specifications with only one contained bag do not have a ",".
                // In that case. split() will simply return a list with len 1, so we can still use
                // an iterator over the list ;)
                let items = rest.split(", ");

                lazy_static! {
                    static ref RE_INNER: Regex = Regex::new(
                        r"(?P<num>\d) (?P<color>.*?) bag(s)?"
                    ).unwrap();
                }

                // iterate over all items and filter + map them
                let inner = items.into_iter().filter_map(|item| {
                    // if we have a capture (= `Some(inner_cap)`), then process it.
                    // if we dont have a capture (=`None`) because "no other bags." then we directly
                    // return that `None` (because of how `map` works). `filter_map` then
                    // discards this completely, resulting in an empty inner.
                    RE_INNER.captures(&item).map(|inner_caps| {
                        InnerBag {
                            num: inner_caps["num"].parse().unwrap(),
                            color: inner_caps["color"].to_string(),
                        }
                    })
                }).collect();

            Bag {
                color: outer_color.to_string(),
                inner,
            }
        })
        .collect()
}

// WARNING! Code may cause serious headache, wo needs graphs anyways? :D
fn part1(po: &TodaysPuzzleOptions) -> OutputType {
    // all bags
    let bags = po.get_data();
    // all bags that can, eventually, contain a shiny gold bag
    let mut valid_bags = Vec::new();

    // let's start by collecting a list of bags that can directly contain a shiny gold bag
    for bag in bags {
        for inner in &bag.inner {
            if inner.color == "shiny gold" {
                valid_bags.push(bag.color.to_owned());
            }
        }
    }

    // search for new bags if we found at least one new bag last round
    let mut last_len = 0;
    while valid_bags.len() != last_len {
        last_len = valid_bags.len();

        // whoop whoop O(n^2) party time ahead :D
        for bag in bags {
            // skip the bag if we already know that its valid
            if valid_bags.contains(&bag.color) {
                continue;
            }

            // if any inner bag is already known as a valid bag, add it to the list
            for inner in &bag.inner {
                // we need the '&& !valid_bags.contains(&bag.color)` in case our bag contains more
                // than one new valid bag
                if valid_bags.contains(&inner.color) && !valid_bags.contains(&bag.color) {
                    valid_bags.push(bag.color.to_owned());
                }
            }
        }
    }

    valid_bags.len()
}

fn part2(po: &TodaysPuzzleOptions) -> OutputType {
    let bags = po.get_data();
    // for each bag color, store the number of bags it contains
    let mut bag_counts = HashMap::new();

    // compute until we have what we need
    while bag_counts.get("shiny gold").is_none() {
        // this will (hopefully) only be computed a few times, it should terminate at some point
        for bag in bags {
            // if we already have the count for this bag, skip it
            if bag_counts.get(&bag.color).is_some() {
                continue;
            // if the bag cannot contain any other bag, store this information
            } else if bag.inner.is_empty() {
                bag_counts.insert(bag.color.to_owned(), 0);
            // otherwise, check if we already know all values for all inner bags
            } else if bag.inner.iter().all(|inner| bag_counts.get(&inner.color).is_some()) {
                // if yes, count!
                // Dont forget to: multiply the value by the number of times this bag is in our bag
                // and also dont forget to add these bags as well :D
                let count: usize = bag.inner.iter().map(|inner| {
                    inner.num + inner.num * bag_counts.get(&inner.color).unwrap()
                }).sum();
                bag_counts.insert(bag.color.to_string(), count);
            }
        }
    }

    *bag_counts.get("shiny gold").unwrap()
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
        test_case_helper("example1", 4, 32)
    }

    #[test]
    fn example_2() {
        test_case_helper("example2", 0, 126)
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
