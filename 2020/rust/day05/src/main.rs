/*
test bench::bench_parsing       ... bench:     261,028 ns/iter (+/- 18,042)
test bench::bench_part1         ... bench:         322 ns/iter (+/- 21)
test bench::bench_part2         ... bench:      10,722 ns/iter (+/- 287)
test bench::bench_part2_initial ... bench:      26,563 ns/iter (+/- 984)
*/

// allow bench feature when using unstable flag
// use: $ rustup run nightly cargo bench --features unstable
#![cfg_attr(feature = "unstable", feature(test))]

use aoc_import_magic::{import_magic, PuzzleOptions};
use std::{collections::HashMap, io};

const DAY: u32 = 5;
type InputTypeSingle = Seat;
type InputType = Vec<InputTypeSingle>;
type OutputType1 = usize;
type OutputType2 = OutputType1;
type TodaysPuzzleOptions = PuzzleOptions<InputType>;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Seat {
    row: usize,
    col: usize,
}

impl Seat {
    fn id(&self) -> usize {
        self.row * 8 + self.col
    }

    pub fn row_from_spec(spec: &str) -> usize {
        // exctract relevant sequence and translate into binary
        let rowspec = spec[0..7].to_owned().replace("F", "0").replace("B", "1");
        // convert into number by using standard library functions
        usize::from_str_radix(&rowspec, 2).unwrap()
    }

    pub fn col_from_spec(spec: &str) -> usize {
        // same approach as in row_from_spec
        let rowspec = spec[7..=9].to_owned().replace("L", "0").replace("R", "1");

        usize::from_str_radix(&rowspec, 2).unwrap()
    }
}

impl From<String> for Seat {
    fn from(from: String) -> Seat {
        assert!(from.len() >= 10);

        Seat {
            row: Self::row_from_spec(&from),
            col: Self::col_from_spec(&from),
        }
    }
}

fn parse_input(input: Vec<String>, _config: &HashMap<String, String>, _verbose: bool) -> InputType {
    // iterate through list and convert Seat from each line
    input.into_iter().map(|line| Seat::from(line)).collect()
}

fn part1(po: &TodaysPuzzleOptions) -> OutputType1 {
    // iterate over list of seats, then map ("transform") each Seat into its id and get the max
    po.get_data().into_iter().map(Seat::id).max().unwrap()
}

fn part2(po: &TodaysPuzzleOptions, _res1: Option<OutputType1>) -> OutputType2 {
    // key = row number, value = list of seats numbers (columns)
    let mut plane = Vec::new();

    // populate plane
    for seat in po.get_data() {
        plane.push(seat.id());
    }

    plane.sort();
    for (aa, bb) in plane.iter().zip(plane.iter().skip(1)) {
        if bb - aa > 1 {
            return aa + 1;
        }
    }

    0
}

// Initial solution (slightly cleaned up) which brought me the star
fn part2_initial(po: &TodaysPuzzleOptions, _res1: Option<OutputType1>) -> OutputType2 {
    // key = row number, value = list of seats numbers (columns)
    let mut plane: HashMap<usize, Vec<_>> = HashMap::new();

    // populate plane
    for seat in po.get_data() {
        let row = seat.row;
        let entry = plane.entry(row).or_default();
        entry.push(seat.col);
    }

    // skip first and last row
    for row in 1..=126 {
        // only consider available rows
        if let Some(seats) = plane.get(&row) {
            // if only one seat is missing (maybe us) check if the rows before and after also exist
            let row_prev = row - 1;
            let row_next = row + 1;
            if seats.len() == 7 && plane.get(&row_prev).is_some() && plane.get(&row_next).is_some()
            {
                // calculate our missing seat number
                let missing_seat: usize =
                    (1usize + 2 + 3 + 4 + 5 + 6 + 7) - seats.iter().sum::<usize>();
                // calculate the ID
                //return row * 8 + missing_seat;
                return Seat {
                    row,
                    col: missing_seat,
                }
                .id();
            }
        }
    }

    0
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rowspec() {
        assert_eq!(44, Seat::row_from_spec("FBFBBFFRLR"));
        assert_eq!(70, Seat::row_from_spec("BFFFBBFRRR"));
        assert_eq!(14, Seat::row_from_spec("FFFBBBFRRR"));
        assert_eq!(102, Seat::row_from_spec("BBFFBBFRLL"));
    }

    #[test]
    fn colspec() {
        assert_eq!(5, Seat::col_from_spec("FBFBBFFRLR"));
        assert_eq!(7, Seat::col_from_spec("BFFFBBFRRR"));
        assert_eq!(7, Seat::col_from_spec("FFFBBBFRRR"));
        assert_eq!(4, Seat::col_from_spec("BBFFBBFRLL"));
    }
}

#[cfg(all(feature = "unstable", test))]
mod bench {
    extern crate test;

    use super::*;
    use aoc_import_magic::{import_magic_with_params, PuzzleOptions, test_helper_import_config};
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };
    use test::Bencher;

    fn import_helper(inputname: &str) -> PuzzleOptions<InputType> {
        let params = ["appname", "--input", inputname];
        import_magic_with_params(DAY, parse_input, &params).unwrap()
    }

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
        let puzzle_options = import_helper("real1");
        bb.iter(|| test::black_box(part1(&puzzle_options)));
    }

    #[bench]
    fn bench_part2_initial(bb: &mut Bencher) {
        let puzzle_options = import_helper("real1");
        bb.iter(|| test::black_box(part2_initial(&puzzle_options, None)));
    }

    #[bench]
    fn bench_part2(bb: &mut Bencher) {
        let puzzle_options = import_helper("real1");
        bb.iter(|| test::black_box(part2(&puzzle_options, None)));
    }
}
