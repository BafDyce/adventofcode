/*

BENCHMARK RESULTS

*/

// allow bench feature when using unstable flag
// use: $ rustup run nightly cargo bench --features unstable
#![cfg_attr(feature = "unstable", feature(test))]

use aoc_import_magic::{import_magic, PuzzleOptions};
use std::{
    collections::HashMap,
    io,
};

const DAY: u32 = 4;
type InputType = Vec<Vec<String>>;
type OutputType = usize;
type TodaysPuzzleOptions = PuzzleOptions<InputType>;


fn parse_input(input: Vec<String>, _config: &HashMap<String, String>, _verbose: bool) -> InputType {
    // What we have: Our input file as a list of lines.
    // What we want: A list of items where each passport is in a single item

    // How?
    input
        // we join everything together again
        .join("\n")
        // Data for the same passport can be split across multuple lines but we know that there is at least one empty
        // line between two passports. This means, we can simply split at two new lines following each other
        .split("\n\n")
        // So, now we have one item per passport, but we still have line breaks within passports. So we split at all
        // whitespace characters (even spaces, tabs, etc.). As a result, we'll have each "data token" in a separate
        // item (of the sublist)
        //
        // .map(ToOwned::to_owned) is required to create strings which we own from the references which split() returns
        // the .collect() at the end, collects all "data items" into a list (one list containing all data for a pass)
        .map(|line| line.split_whitespace().map(ToOwned::to_owned).collect())
        // now collect all passports into a list
        .collect()
}

fn part1(po: &TodaysPuzzleOptions) -> OutputType {
    let mut count = 0;

    for passport in po.get_data().into_iter() {
        // We keep track of the specified data items in a hashmap, initializing everything with false. if something
        // exists, we'll set the value to true.
        let mut data: HashMap<&str, bool> = [
            ("byr", false),
            ("iyr", false),
            ("eyr", false),
            ("hgt", false),
            ("hcl", false),
            ("ecl", false),
            ("pid", false),
            // cid is not required, so we can ignore it
            //("cid", false),
        // this line just converts our list of tuples into a hashmap. See how .collect() does the right thing (
        // collecting into a HashMap, while it collected into a list before? Thats some of the magic you get from the
        // rust compiler <3)
        ].iter().cloned().collect();

        for item in passport {
            // Here comes a simple showcase of Rust's amazing pattern matching capabilities
            // We're only interested in the first three characters of each data item
            match &item[0..3] {
                // and if they match a specific "word" ..
                xx @ "byr" |
                xx @ "iyr" |
                xx @ "eyr" |
                xx @ "hgt" |
                xx @ "hcl" |
                xx @ "ecl" |
                xx @ "pid" => {
                    // then we set its corresponding value to true
                    data.insert(xx, true);
                }
                // For all other strings, we do nothing. Rust's `match` REQUIRES handling of ALL possible patterns, but
                // we can also specify a wildcard (but we must do this explicitely). If we would omit the line, we would
                // get an error message similar to the following:
                //
                // error[E0004]: non-exhaustive patterns: `&_` not covered
                //  --> src/main.rs:69:19
                //  |
                //69 |             match &item[0..3] {
                //  |                   ^^^^^^^^^^^ pattern `&_` not covered
                //  |
                //  = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
                //  = note: the matched value is of type `&str`
                _ => {}
            }
        }

        // Now we can just check if all values in the hashmap are true and increase the counter if they are.
        if data.values().into_iter().all(|&x| x) {
            count += 1;
        }
    }

    count
}

fn part2(po: &TodaysPuzzleOptions) -> OutputType {
    let mut count = 0;

    for passport in po.get_data().into_iter() {
        // same approach as before
        let mut data: HashMap<&str, bool> = [
            ("byr", false),
            ("iyr", false),
            ("eyr", false),
            ("hgt", false),
            ("hcl", false),
            ("ecl", false),
            ("pid", false),
        ].iter().cloned().collect();

        for item in passport {
            // But this time, we actually have to check the values as well, so we need to handle each data point in
            // their own match arm.
            match &item[0..3] {
                xx @ "byr" => {
                    // Behold of some more rust pattern magic power!
                    // Here .parse() will try to parse the given string into a number. The result will either be an
                    // Ok(number) or an Err(error). And here we tell rust in a single line to only consider cases where
                    // the number is in a specific range
                    if let Ok(1920 ..= 2002) = &item[4..].parse() {
                        data.insert(xx, true);
                    }

                    // We *could* also write a match like this (i made this in the original implementation):
                    //
                    //match &item[4..].parse() {
                    //   Ok(2020 ..= 2030) => {
                    //        data.insert(xx, true);
                    //    },
                    //    _ => {},
                    //}
                }
                xx @ "iyr" => {
                    // Something, I would also like to show here is the dbg!() macro, as I had to use it a couple of
                    // times. Instead of writing the following line (as in the code below) ..
                    //
                    // if let Ok(2010 ..= 2020) = &item[4..].parse() {
                    //
                    // .. we can insert the dbg!() macro:
                    //
                    // if let Ok(2010 ..= 2020) = dbg!(&item[4..]).parse() {
                    //
                    // As a result, the code continues to work but the program would print the following at runtime:
                    // [src/main.rs:154] &item[4..] = "2019"
                    // [src/main.rs:154] &item[4..] = "2017"
                    // [src/main.rs:154] &item[4..] = "2026"
                    // [src/main.rs:154] &item[4..] = "2006"
                    if let Ok(2010 ..= 2020) = &item[4..].parse() {
                        data.insert(xx, true);
                    }
                }
                xx @ "eyr" => {
                    if let Ok(2020 ..= 2030) = &item[4..].parse() {
                        data.insert(xx, true);
                    }
                }
                xx @ "hgt" => {
                    if item.ends_with("cm") {
                        if let Ok(150 ..= 193) = &item[4..item.len()-2].parse() {
                            data.insert(xx, true);
                        }
                    } else if item.ends_with("in") {
                        if let Ok(59 ..= 76) = &item[4..item.len()-2].parse() {
                            data.insert(xx, true);
                        }
                    }

                }
                xx @ "hcl" => {
                    // check if the 4th character is a # (we cannot use item[4] because Rust strings do not support
                    // indexing because of utf-8 support and resulting variable character/byte boundaries. Specifying
                    // anges, e.g. item[5..] works though :D)
                    // Also, luckily Rust already provides convenience functions like is_ascii_hexdigit()
                    if item.chars().nth(4) == Some('#') && item[5..].chars().all(|cc| cc.is_ascii_hexdigit()) {
                        data.insert(xx, true);
                    }
                }
                xx @ "ecl" => {
                    match &item[4..] {
                        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {
                            data.insert(xx, true);
                        }
                        _ => {},
                    }
                }
                xx @ "pid" => {
                    // 13 == 3 ("pid") + 1 (":") + 9 (digits)
                    if item.len() == 13 && item[5..].chars().all(|cc| cc.is_ascii_digit()) {
                        data.insert(xx, true);
                    }
                }
                _ => {}
            }
        }

        if data.values().into_iter().all(|&x| x) {
            count += 1;
        }
    }

    count
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
    }

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

    fn test_case_helper(inputname: &str, sol1: OutputType1, sol2: OutputType2) {
        let po = import_helper(inputname);
        let res1 = part1(&po);
        assert_eq!(sol1, res1, "part1");
        let res2 = part2(&po, Some(res1));
        assert_eq!(sol2, res2, "part2");
    }

    #[test]
    fn example_1() {
        test_case_helper("example1", 8, 8)
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
