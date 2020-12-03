/*
      -------Part 1--------   -------Part 2--------
Day       Time  Rank  Score       Time  Rank  Score
  3   00:19:13  4646      0   00:24:20  3840      0


test bench::bench_parsing ... bench:      50,979 ns/iter (+/- 3,009)
test bench::bench_part1   ... bench:       1,222 ns/iter (+/- 11)
test bench::bench_part2   ... bench:       5,282 ns/iter (+/- 72)
*/

// allow bench feature when using unstable flag
// use: $ rustup run nightly cargo bench --features unstable
#![cfg_attr(feature = "unstable", feature(test))]

use aoc_import_magic::{import_magic, PuzzleOptions};
use std::{
    collections::HashMap,
    io,
    ops::Index,
};

const DAY: u32 = 3;
type InputType = Map;
type OutputType1 = usize;
type OutputType2 = OutputType1;
type TodaysPuzzleOptions = PuzzleOptions<InputType>;



#[derive(Clone, Debug, PartialEq)]
struct Map {
    map: Vec<Vec<Field>>,
    oob: Field,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Field {
    Open,
    Tree,
    OutOfBounds,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Position {
    col: usize,
    row: usize,
}

impl From<char> for Field {
    fn from(from: char) -> Field {
        match from {
            '.' => Field::Open,
            '#' => Field::Tree,
            _ => panic!("Invalid input!"),
        }
    }
}

impl Map {
    fn new(map: Vec<Vec<Field>>) -> Self {
        Map {
            map,
            oob: Field::OutOfBounds,
        }
    }
}

impl Position {
    fn new() -> Position {
        Position {
            col: 0,
            row: 0,
        }
    }

    fn step_1(&mut self) {
        self.row += 1;
        self.col += 3;
    }

    fn step_2(&mut self, step_length: &Position) {
        self.row += step_length.row;
        self.col += step_length.col;
    }
}

impl Index<Position> for Map {
    type Output = Field;

    fn index(&self, pos: Position) -> &Self::Output {
        if pos.row >= self.map.len() {
            &self.oob
        } else {
            let col = pos.col % self.map[pos.row].len();
            &self.map[pos.row][col]
        }
    }
}

fn parse_input(input: Vec<String>, _config: &HashMap<String, String>, _verbose: bool) -> InputType {
    let map = input
        .into_iter()
        .map(|line| {
            line.chars().map(Field::from).collect()
        })
        .collect();

    Map::new(map)
}

fn part1(po: &TodaysPuzzleOptions) -> OutputType1 {
    let map = po.get_data();
    let mut pos = Position::new();
    let mut count = 0;

    loop {
        let field = map[pos];
        pos.step_1();
        match field {
            Field::OutOfBounds => break,
            Field::Tree => count += 1,
            _ => {},
        }
    }

    count
}

fn part2(po: &TodaysPuzzleOptions, _res1: Option<OutputType1>) -> OutputType2 {
    let map = po.get_data();
    let steps = vec![
        Position {
            col: 1, row: 1,
        },
        Position {
            col: 3, row: 1,
        },
        Position {
            col: 5, row: 1,
        },
        Position {
            col: 7, row: 1,
        },
        Position {
            col: 1, row: 2,
        },
    ];
    let mut count_total = 1;

    for step in steps {
        let mut pos = Position::new();
        let mut count = 0;

        loop {
            let field = map[pos];
            pos.step_2(&step);
            match field {
                Field::OutOfBounds => break,
                Field::Tree => count += 1,
                _ => {},
            }
        }

        count_total *= count;
    }

    count_total
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
        test_case_helper("example1", 7, 336)
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
