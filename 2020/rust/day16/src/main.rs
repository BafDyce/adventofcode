/*
      -------Part 1--------   -------Part 2--------
Day       Time  Rank  Score       Time  Rank  Score
 16   00:43:47  4769      0   01:18:00  2714      0

test bench::bench_parsing ... bench:     137,156 ns/iter (+/- 3,327)
test bench::bench_part1   ... bench:       8,030 ns/iter (+/- 222)
test bench::bench_part2   ... bench:     560,917 ns/iter (+/- 7,447)

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
    fn ticket_scanning_rate(&self) -> usize {
        let valid_ranges: Vec<(&(usize, usize), &(usize, usize))> = self.classes.iter().map(|class| {
            (
                (&class.ranges[0]),
                (&class.ranges[1]),
            )
        }).collect();

        let mut error_rate = 0;
        for ticket in &self.others {
            for &number in &ticket.numbers {
                let numbercheck = valid_ranges.iter().any(|((low_1, high_1), (low_2, high_2))| {
                    (number >= *low_1 && number <= *high_1) || (number >= *low_2 && number <= *high_2)
                });

                if ! numbercheck {
                    error_rate += number;
                }
            }
        }

        error_rate
    }

    fn get_valid_tickets(&self) -> Vec<&Ticket> {
        let valid_ranges: Vec<(&(usize, usize), &(usize, usize))> = self.classes.iter().map(|class| {
            (
                (&class.ranges[0]),
                (&class.ranges[1]),
            )
        }).collect();

        self.others.iter().filter(|ticket| {
            let mut ticket_valid = true;
            for &number in &ticket.numbers {
                let numbercheck = valid_ranges.iter().any(|((low_1, high_1), (low_2, high_2))| {
                    (number >= *low_1 && number <= *high_1) || (number >= *low_2 && number <= *high_2)
                });

                ticket_valid = ticket_valid && numbercheck;
            }

            ticket_valid
        }).collect()
    }
}

impl From<Vec<String>> for TicketData {
    fn from(from: Vec<String>) -> TicketData {
        let mut classes = Vec::new();
        let mut data = from.into_iter();
        // ranges
        //for line in data {
        while let Some(line) = data.next() {
            if line.is_empty() {
                break;
            }

            classes.push(TicketClass::from(line));
        }

        // "your ticket line"
        let _ = data.next();
        let mine = Ticket::from(data.next().unwrap());

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
    fn valid_number(&self, number: usize) -> bool {
        self.ranges.iter().any(|&(low, high)| number >= low && number <= high)
    }

    fn all_valid_numbers(&self, numbers: &HashSet<usize>) -> bool {
        numbers.iter().all(|number| self.valid_number(*number))
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

    data.ticket_scanning_rate()
}

fn part2(po: &TodaysPuzzleOptions) -> OutputType2 {
    let data = po.get_data();

    let valid_tickets = data.get_valid_tickets();
    let mut field_numbers: HashMap<usize, HashSet<usize>> = HashMap::new();

    for ticket in valid_tickets {
        for (field, number) in ticket.numbers.iter().enumerate() {
            let entry = field_numbers.entry(field).or_default();
            entry.insert(*number);
        }
    }

    //dbg!(&field_numbers);
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

    //dbg!(&possible_classes);
    let target = data.classes.len();
    let mut fields = HashMap::new();

    loop {
        let mut next_hit = None;
        for (field, candidates) in &possible_classes {
            if candidates.len() == 1 {
                next_hit = Some((*field, candidates[0].to_owned()));
                break;
            }
        }

        match next_hit {
            Some((field, field_name)) => {
                for candidates in possible_classes.values_mut() {
                    //candidates.remove_item(&next_hit.1)
                    if let Some(position) = candidates.iter().position(|item| **item == field_name) {
                        candidates.remove(position);
                    }
                }

                fields.insert(field, field_name);
            }
            None => {
                println!("[ERROR] No candidate found!");
                dbg!(&fields);
                dbg!(&possible_classes);
            }
        }

        if fields.len() == target {
            break;
        }
    }

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
        test_case_helper("example1", Some(17), None)
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
