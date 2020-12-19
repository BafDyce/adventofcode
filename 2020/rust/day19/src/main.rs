/*

test bench::bench_parsing ... bench:     172,231 ns/iter (+/- 2,498)
test bench::bench_part1   ... bench:   5,224,529 ns/iter (+/- 39,053)
test bench::bench_part2   ... bench:     369,039 ns/iter (+/- 4,438)

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
    collections::{HashMap, HashSet},
    io,
};

const DAY: u32 = 19;
type RuleSet = HashMap<usize, MatchingRule>;
type InputType = (RuleSet, Vec<String>);
type OutputType1 = usize;
type OutputType2 = OutputType1;
type TodaysPuzzleOptions = PuzzleOptions<InputType>;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
enum MatchingRule {
    Letter(char),
    Subgroups(Vec<Vec<usize>>),
    Special(HashSet<String>),
}

const MAGIC_NUMBER: usize = 8;

impl MatchingRule {
    fn full_match(&self, string: &str, ruleset: &RuleSet) -> bool {
        match self.matches(string, ruleset) {
            Some(len) => len == string.len(),
            None => false,
        }
    }

    fn matches(&self, string: &str, ruleset: &RuleSet) -> Option<usize> {
        match self {
            MatchingRule::Letter(cc) => {
                match string.chars().nth(0) {
                    Some(first_cc) if *cc == first_cc => Some(1),
                    _ => None,
                }
            }
            MatchingRule::Subgroups(groups) => {
                let mut match_sizes = Vec::new();
                for group in groups {
                    let mut offset = 0;
                    for subrule_num in group {
                        let subrule = ruleset.get(subrule_num).unwrap();
                        match subrule.matches(&string[offset..], ruleset) {
                            None => {
                                offset = 0;
                                break
                            },
                            Some(matching_chars) => offset += matching_chars,
                        }
                    }

                    match_sizes.push(offset);
                }

                match_sizes.into_iter().filter(|size| *size != 0).next()
            }
            MatchingRule::Special(matching_set) => {
                if string.len() >= MAGIC_NUMBER && matching_set.contains(&string[0..MAGIC_NUMBER]) {
                    Some(MAGIC_NUMBER)
                } else {
                    None
                }
            },
        }
    }


    fn build_strings(&self, ruleset: &RuleSet) -> Vec<String> {
        match self {
            MatchingRule::Letter(cc) => vec![format!("{}", cc)],
            MatchingRule::Subgroups(groups) => {
                let mut strings = Vec::new();
                for group in groups {
                    let mut group_strings = Vec::new();
                    for subrule_num in group {
                        let subrule = ruleset.get(subrule_num).unwrap();
                        let subrule_strings = subrule.build_strings(ruleset);

                        if group_strings.is_empty() {
                            group_strings = subrule_strings;
                        } else {
                            group_strings = group_strings.into_iter().map(|existing_group_string| {
                                subrule_strings.iter().map(|new_subrule_string| {
                                    format!("{}{}", existing_group_string, new_subrule_string)
                                }).collect::<Vec<String>>()
                            }).flatten().collect();
                        }

                    }

                    strings.extend(group_strings);
                }

                strings
            }
            MatchingRule::Special(list) => list.iter().map(ToOwned::to_owned).collect(),
        }
    }
}

impl From<&str> for MatchingRule {
    fn from(from: &str) -> MatchingRule {
        if from.contains('"') {
            MatchingRule::Letter(from.chars().nth(1).unwrap())
        } else {
            let groups = from.split("|").map(|group| {
                group.split_ascii_whitespace().map(|item| item.parse().unwrap()).collect()
            }).collect();

            MatchingRule::Subgroups(groups)
        }
    }
}

fn parse_input(input: Vec<String>, _config: &HashMap<String, String>, _verbose: bool) -> InputType {
    let mut rules = HashMap::new();
    let mut strings = Vec::new();

    for line in input {
        lazy_static! {
            static ref RE_RULE: Regex = Regex::new(
                r"(?P<num>\d+): (?P<rule>.*)"
            ).unwrap();
        }

        match RE_RULE.captures(&line) {
            Some(caps) => {
                let rule = MatchingRule::from(&caps["rule"]);
                let number = caps["num"].parse().unwrap();

                rules.insert(number, rule);
            }
            None => {
                if !line.is_empty() {
                    strings.push(line);
                }
            }
        }
    }

    (rules, strings)
}

fn part1(po: &TodaysPuzzleOptions) -> OutputType1 {
    let (rules, strings) = po.get_data();

    let rule_0 = rules.get(&0).unwrap();
    let mut count = 0;
    for string in strings {
        if rule_0.full_match(string, &rules) {
            count += 1;
        }
    }

    count
}

fn part2(po: &TodaysPuzzleOptions) -> OutputType2 {
    let (rules, strings) = po.get_data();
    let rules = rules.to_owned();
/*
    rules.insert(8, MatchingRule::from("42 | 42 8"));
    rules.insert(11, MatchingRule::from("42 31 | 42 11 31"));
    rules.insert(31, MatchingRule::Special(
        rules.get(&31).unwrap().build_strings(&rules).into_iter().collect()
    ));
    rules.insert(42, MatchingRule::Special(
        rules.get(&42).unwrap().build_strings(&rules).into_iter().collect()
    ));
*/

    let strings_31: HashSet<_> = rules.get(&31).unwrap().build_strings(&rules).into_iter().collect();
    let strings_42: HashSet<_> = rules.get(&42).unwrap().build_strings(&rules).into_iter().collect();

    let mut count = 0;

    for mut string in strings.clone() {
        // MUST start with 42 TWICE
        if string.len() < 2* MAGIC_NUMBER {
            continue;
        }
        if !strings_42.contains(&string[0..MAGIC_NUMBER]) {
            continue;
        }
        if !strings_42.contains(&string[MAGIC_NUMBER..MAGIC_NUMBER*2]) {
            continue;
        }


        let mut counted_42s = 0;
        let mut counted_31s = 0;

        // now remove "42" as often as possible
        while string.len() >= MAGIC_NUMBER && strings_42.contains(&string[0..MAGIC_NUMBER]) {
            string = string.split_off(MAGIC_NUMBER);
            counted_42s += 1;
        }

        // we need at least MAGIC_NUMBER more characters for at least one "31"
        if string.len() < MAGIC_NUMBER {
            continue;
        }

        // rmeove "31" as often as possible
        while string.len() >= MAGIC_NUMBER && strings_31.contains(&string[0..MAGIC_NUMBER]) {
            string = string.split_off(MAGIC_NUMBER);
            counted_31s += 1;
        }


        // now the string must be empty AND we must have had more "42"s than "31"s (because each "31" requires EXACTLY
        // one "42")
        if string.len() == 0 && counted_42s > counted_31s {
            count += 1;
        }
    }

    println!("MAGIC NUMBER IS {}", MAGIC_NUMBER);
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

    fn test_case_helper(inputname: &str, sol1: Option<OutputType1>, sol2: Option<OutputType2>) {
        let po = import_helper(inputname);

        if let Some(sol1) = sol1 {
            let res1 = part1(&po);
            assert_eq!(sol1, res1, "part1");
        }

        if let Some(sol2) = sol2 {
            let res2 = part2(&po);
            assert_eq!(sol2, res2, "part2");
        }
    }

    #[test]
    fn example_1() {
        test_case_helper("example1", Some(2), None)
    }

    #[test]
    fn example_2() {
        test_case_helper("example2", Some(3), None)
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
