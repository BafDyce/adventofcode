/*

BENCHMARK RESULTS

*/

// allow bench feature when using unstable flag
// use: $ cargo +nightly bench --features unstable
#![cfg_attr(feature = "unstable", feature(test))]

mod intcode;
use intcode::*;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

use aoc_import_magic::{import_magic, PuzzleOptions};
use regex::Regex;
use std::{
    collections::{HashMap, VecDeque, HashSet},
    io,
};

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
enum Position {
    Empty,
    Asteroid,
}

impl Position {
    fn print(&self) {
        match self {
            Position::Empty => print!("."),
            Position::Asteroid => print!("#"),
        }
    }
}

const DAY: i32 = 10;
type InputTypeSingle = Position;
type InputType = Vec<Vec<InputTypeSingle>>;
type OutputType1 = usize;
type OutputType2 = OutputType1;
type TodaysPuzzleOptions = PuzzleOptions<InputType>;


impl From<char> for Position {
    fn from(cc: char) -> Position {
        match cc {
            '.' => Position::Empty,
            '#' => Position::Asteroid,
            _ => panic!("Invalid char! {}", cc),
        }
    }
}

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
            // Parsing logic
            // single numeric types

            line.chars().map(|cc| Position::from(cc)).collect()
        })
        .collect()
}

fn count_asteroids_in_direct_sight(space: &Vec<Vec<Position>>, xx: usize, yy: usize) -> usize {
    let max_x = space.len();
    let max_y = space[0].len();

    let mut detected = HashSet::new();

    let mut count = 0;
    for step_x in (0 .. max_x).rev() {
        for step_y in (0 .. max_y).rev() {
            if step_x == 0 && step_y == 0 {
                continue;
            }
            // left down
            let mut check_x = xx + step_x;
            let mut check_y = yy + step_y;

            while check_x < max_x && check_y < max_y {
                if space[check_x][check_y] == Position::Asteroid {
                    detected.insert((check_x, check_y));
                    check_x += step_x;
                    check_y += step_y;
                    while check_x < max_x && check_y < max_y {
                        detected.remove(&(check_x, check_y));
                        check_x += step_x;
                        check_y += step_y;
                    }

                    break;
                }

                check_x += step_x;
                check_y += step_y;
            }

            // left up
            if xx >= step_x {
                let mut check_x = xx - step_x;
                let mut check_y = yy + step_y;

                while check_x >= 0 && check_y < max_y {
                    if space[check_x][check_y] == Position::Asteroid {
                        detected.insert((check_x, check_y));
                        if step_x > check_x {
                            break;
                        }
                        check_x -= step_x;
                        check_y += step_y;
                        while check_y < max_y {
                            detected.remove(&(check_x, check_y));
                            if step_x > check_x {
                                break;
                            }
                            check_x -= step_x;
                            check_y += step_y;
                        }

                        break;
                    }

                    if step_x > check_x {
                        break;
                    }
                    check_x -= step_x;
                    check_y += step_y;
                }
            }


            // right up
            if xx >= step_x && yy >= step_y {
                let mut check_x = xx - step_x;
                let mut check_y = yy - step_y;

                while check_x >= 0 && check_y >= 0 {
                    if space[check_x][check_y] == Position::Asteroid {
                        detected.insert((check_x, check_y));
                        if step_x > check_x || step_y > check_y {
                            break;
                        }
                        check_x -= step_x;
                        check_y -= step_y;
                        while check_x >= 0 && check_y >= 0 {
                            detected.remove(&(check_x, check_y));
                            if step_x > check_x || step_y > check_y {
                                break;
                            }
                            check_x -= step_x;
                    check_y -= step_y;
                        }

                        break;
                    }

                    if step_x > check_x || step_y > check_y {
                        break;
                    }
                    check_x -= step_x;
                    check_y -= step_y;
                }
            }




            // right down
            if yy >= step_y {
                let mut check_x = xx + step_x;
                let mut check_y = yy - step_y;

                while check_x < max_x && check_y >= 0 {
                    if space[check_x][check_y] == Position::Asteroid {
                        detected.insert((check_x, check_y));
                        if step_y > check_y {
                            break;
                        }
                        check_x += step_x;
                        check_y -= step_y;
                        while check_x < max_x && check_y >= 0 {
                            detected.remove(&(check_x, check_y));
                            if step_y > check_y {
                                break;
                            }
                            check_x += step_x;
                    check_y -= step_y;
                        }

                        break;
                    }

                    if step_y > check_y {
                        break;
                    }
                    check_x += step_x;
                    check_y -= step_y;
                }
            }
        }
    }

    detected.len()
}

fn part1(po: &TodaysPuzzleOptions) -> OutputType1 {
    let space = po.data.as_ref().unwrap();

    let mut max_count = std::usize::MIN;
    let mut pos = (0, 0);

    for (xx, row) in space.iter().enumerate() {
        for (yy, field) in row.iter().enumerate() {
            if space[xx][yy] == Position::Asteroid {
                let count = count_asteroids_in_direct_sight(space, xx, yy);
                //max_count = usize::max(max_count, count);
                if count > max_count {
                    max_count = count;
                    pos = (xx, yy);
                }
            }
        }
    }

    println!("{:?}", pos);
    max_count
}

fn part2(po: &TodaysPuzzleOptions, res1: Option<OutputType1>) -> OutputType2 {
    //dbg!(po.data.as_ref().unwrap());
    for ii in (3..7).rev() {
        println!("{}", ii);
    }

    0
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
