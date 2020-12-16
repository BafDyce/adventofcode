/*
      -------Part 1--------   -------Part 2--------
Day       Time  Rank  Score       Time  Rank  Score
 16   00:43:47  4769      0   01:18:00  2714      0

test bench::bench_parsing ... bench:     137,253 ns/iter (+/- 1,176)
test bench::bench_part1   ... bench:      14,456 ns/iter (+/- 831)
test bench::bench_part2   ... bench:     559,044 ns/iter (+/- 9,259)

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

const DAY: u32 = 16;
type InputType = TicketData;
type OutputType1 = usize;
type OutputType2 = OutputType1;
type TodaysPuzzleOptions = PuzzleOptions<InputType>;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
struct TicketData {
    classes: Vec<TicketClass>,
    mine: Ticket,
    others: Vec<Ticket>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
struct Ticket {
    numbers: Vec<usize>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
struct TicketClass {
    name: String,
    ranges: Vec<(usize, usize)>,
}

impl TicketData {
    fn ticket_scanning_error_rate(&self) -> usize {
        let mut error_rate = 0;
        for ticket in &self.others {
            for &number in &ticket.numbers {
                let number_invalid_at_least_once = self.classes.iter().all(|class| !class.is_valid_number(number));

                if number_invalid_at_least_once {
                    error_rate += number;
                }
            }
        }

        error_rate
    }

    fn get_valid_tickets(&self) -> Vec<&Ticket> {
        self.others.iter().filter(|ticket| {
            let mut ticket_valid = true;
            for &number in &ticket.numbers {
                let number_invalid_at_least_once = self.classes.iter().all(|class| !class.is_valid_number(number));

                ticket_valid = ticket_valid && !number_invalid_at_least_once;
            }

            ticket_valid
        }).collect()
    }
}

impl From<Vec<String>> for TicketData {
    fn from(from: Vec<String>) -> TicketData {
        let mut classes = Vec::new();
        let mut data = from.into_iter();

        while let Some(line) = data.next() {
            if line.is_empty() {
                break;
            }

            classes.push(TicketClass::from(line));
        }

        // "your ticket line" (empty line was already consumed above)
        let _ = data.next();
        let mine = Ticket::from(data.next().unwrap());

        // empty line, followed by "nearby tickets:"
        let _ = data.next();
        let _ = data.next();

        TicketData {
            classes,
            mine: mine,
            others: data.map(Ticket::from).collect(),
        }
    }
}

impl TicketClass {
    fn is_valid_number(&self, number: usize) -> bool {
        self.ranges.iter().any(|&(low, high)| number >= low && number <= high)
    }

    fn all_valid_numbers(&self, numbers: &HashSet<usize>) -> bool {
        numbers.iter().all(|number| self.is_valid_number(*number))
    }
}

impl From<String> for TicketClass {
    fn from(from: String) -> TicketClass {
        lazy_static! {
            static ref RE_CLASSES: Regex = Regex::new(
                r"(?P<name>.*?): (?P<range1_low>\d+)-(?P<range1_high>\d+) or (?P<range2_low>\d+)-(?P<range2_high>\d+)"
            ).unwrap();
        }

        let caps = RE_CLASSES.captures(&from).unwrap();

        TicketClass {
            name: caps["name"].to_string(),
            ranges: vec![
                (caps["range1_low"].parse().unwrap(), caps["range1_high"].parse().unwrap()),
                (caps["range2_low"].parse().unwrap(), caps["range2_high"].parse().unwrap()),
            ]
        }
    }
}

impl From<String> for Ticket {
    fn from(from: String) -> Ticket {
        Ticket {
            numbers: from.split(",").map(|number| number.parse().unwrap()).collect()
        }
    }
}

fn parse_input(input: Vec<String>, _config: &HashMap<String, String>, _verbose: bool) -> InputType {
    TicketData::from(input)
}


fn part1(po: &TodaysPuzzleOptions) -> OutputType1 {
    let data = po.get_data();

    data.ticket_scanning_error_rate()
}

fn part2(po: &TodaysPuzzleOptions) -> OutputType2 {
    let data = po.get_data();

    let valid_tickets = data.get_valid_tickets();

    // For each field, collect all observed values
    let mut field_numbers: HashMap<usize, HashSet<usize>> = HashMap::new();
    for ticket in valid_tickets {
        for (field, number) in ticket.numbers.iter().enumerate() {
            let entry = field_numbers.entry(field).or_default();
            entry.insert(*number);
        }
    }

    // For each field, collect the list of classes which would be possible (i.e. allow all observed numbers for that
    // field)
    let mut possible_classes: HashMap<_, _> = field_numbers.into_iter().map(|(field, numbers)| {
        let possible_classes: Vec<_> = data.classes.iter().filter_map(|class| {
            if class.all_valid_numbers(&numbers) {
                Some(&class.name)
            } else {
                None
            }
        }).collect();

        (field, possible_classes)
    }).collect();

    // How many fields do we have? That's the number of fields we need to compute.
    let target = data.classes.len();
    // For each field, save the class (once we know it)
    let mut fields = HashMap::new();

    // Loop is guaranteed to terminate at some point:
    // Either we find 1 or more next_hits and increase the length of fields, OR we dont find a next_hit and return from
    // the function altogether.
    while fields.len() != target {
        // scan possible_classes for those with only one item remaining (if only one candidate is remaining, then this
        // must be it)
        // We must create owned copies of the data so that we dont create references to `possible_classes` (as this
        // would prevent us from removing entries further down)
        let next_hits: Vec<_> = possible_classes.iter().filter_map(|(field, candidates)| {
            if candidates.len() == 1 {
                Some((*field, candidates[0].to_owned()))
            } else {
                None
            }
        }).collect();

        // just in case
        if next_hits.is_empty() {
            println!("[ERROR] No candidate found!");
            dbg!(&fields);
            dbg!(&possible_classes);
            return 0;
        }

        for (field, field_name) in next_hits {
            // remove this name from the candidates list of all fields (if present)
            for candidates in possible_classes.values_mut() {
                if let Some(position) = candidates.iter().position(|item| **item == field_name) {
                    candidates.remove(position);
                }
            }

            // and store the information that we found it
            fields.insert(field, field_name);
        }

    }

    // now just filter for the relevant fields and multiply the values from our ticket
    fields.into_iter().filter_map(|(position, name)| {
        if name.starts_with("departure") {
            Some(data.mine.numbers[position])
        } else {
            None
        }
    }).product()
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
        test_case_helper("example1", Some(71), None)
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
