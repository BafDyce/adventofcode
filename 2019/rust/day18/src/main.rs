/*

BENCHMARK RESULTS

*/

// allow bench feature when using unstable flag
// use: $ rustup run nightly cargo bench --features unstable
#![cfg_attr(feature = "unstable", feature(test))]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

mod intcode;
use intcode::*;

use aoc_import_magic::{import_magic, PuzzleOptions};
use regex::Regex;
use std::{
    collections::{HashMap, VecDeque},
    fmt,
    io,
};

const DAY: i32 = 18;
type InputTypeSingle = Field;
type InputType = Maze;
type OutputType1 = usize;
type OutputType2 = OutputType1;
type TodaysPuzzleOptions = PuzzleOptions<InputType>;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
enum Field {
    Entrance,
    Open,
    Wall,
    Key(char),
    Door(char),
}

impl From<char> for Field {
    fn from(from: char) -> Field {
        match from {
            '@' => Field::Entrance,
            '.' => Field::Open,
            '#' => Field::Wall,
            key @ 'a' ..= 'z' => Field::Key(key),
            key @ 'A' ..= 'Z' => Field::Door(key.to_ascii_lowercase()),
            other => panic!("Invalid map specification {}", other),
        }
    }
}

impl fmt::Display for Field {
    fn fmt(&self, ff: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cc = match self {
            Field::Entrance => '@',
            Field::Open => '.',
            Field::Wall => '#',
            Field::Key(key) => *key,
            Field::Door(key) => key.to_ascii_uppercase(),
        };

        write!(ff, "{}", cc)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
struct Maze {
    maze: Vec<Vec<Field>>,
}

impl Maze {
    fn reachable_keys(&self, person: &Person) -> Vec<(char, usize, (usize, usize))> {
        let mut keys = Vec::new();

        let mut checked = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back( ((person.xx, person.yy), 0, vec![]));

        while let Some((pos, distance, tmp_keys)) = queue.pop_front() {
            match self.maze[pos.0][pos.1] {
                Field::Key(key) => {
                    keys.push( (key, distance, pos) );
                }
                _ => {}
            }

            let adjacent = {
                let mut adj = Vec::new();

                if pos.0 > 0 {
                    adj.push( (pos.0 - 1, pos.1) );
                }

                if pos.0 < self.maze.len() - 1 {
                    adj.push( (pos.0 + 1, pos.1) );
                }

                if pos.1 > 0 {
                    adj.push( (pos.0, pos.1 - 1) );
                }

                if pos.1 < self.maze[pos.0].len() - 1 {
                    adj.push( (pos.0, pos.1 + 1) );
                }

                adj
            };

            for adj in adjacent.iter() {
                if self.is_walkable(*adj, person, &tmp_keys) {
                    if !queue.iter().any(|&(pos, _, _)| pos == *adj) && !checked.contains(adj) {
                        let tmp_keys = match self.maze[pos.0][pos.1] {
                            Field::Key(key) => {
                                let mut tmp_keys = tmp_keys.to_owned();
                                tmp_keys.push(key);

                                tmp_keys
                            }
                            _ => tmp_keys.to_owned(),
                        };
                        queue.push_back((*adj, distance + 1, tmp_keys));
                    }
                }
            }

            checked.push(pos);
        }

        keys
    }

    fn is_walkable(&self, (xx, yy): (usize, usize), person: &Person, additional_keys: &Vec<char>) -> bool {
        match self.maze[xx][yy] {
            Field::Entrance => true,
            Field::Open => true,
            Field::Key(_) => true,
            Field::Door(key) => person.keys.contains(&key) || additional_keys.contains(&key),
            _ => false,
        }
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, ff: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.maze {
            for field in row {
                if let Err(ee) = field.fmt(ff) {
                    return Err(ee);
                }
            }

            if let Err(ee) = writeln!(ff, "") {
                return Err(ee);
            }
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
struct Person {
    xx: usize,
    yy: usize,
    keys: Vec<char>,
    steps: usize,
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
    Maze {
        maze: input
            .into_iter()
            .map(|line| {
                // Parsing logic
                // single numeric types

                line.chars().map(|cc| Field::from(cc)).collect()
            })
            .collect()
    }
}

fn part1(po: &TodaysPuzzleOptions) -> OutputType1 {
    let maze = po.data.as_ref().unwrap();

    if po.verbose {
        println!("{}", maze);
    }

    let (mut entrance_xx, mut entrance_yy) = (0, 0);
    let mut number_of_keys = 0;
    let mut keys = Vec::new();
    let mut doors = Vec::new();
    for (xx, row) in maze.maze.iter().enumerate() {
        for (yy, field) in row.iter().enumerate() {
            match field {
                Field::Entrance => {
                    entrance_xx = xx;
                    entrance_yy = yy;
                },
                Field::Key(key) => keys.push(key),
                Field::Door(door) => doors.push(door),
                _ => {}
            }
        }
    }

    if po.verbose {
        println!("entrance @ {} / {}", entrance_xx, entrance_yy);
        println!("keys ({}): {:?}", keys.len(), keys);
        println!("doors ({}): {:?}", doors.len(), doors);
    }

    let mut target_key = '0';
    for key in keys {
        if ! doors.contains(&key) {
            target_key = *key;
            break;
        }
    }

    if po.verbose {
        println!("target key: {}", target_key);
    }

    let mut person = Person {
        xx: entrance_xx,
        yy: entrance_yy,
        keys: Vec::new(),
        steps: 0,
    };

    while person.keys.len() < number_of_keys {
        let keys = maze.reachable_keys(&person);
        println!("reachable keys: {:?}", keys);
        break;
    }

    let mut checked = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(('0', 0, (entrance_xx, entrance_yy), vec![]));

    while let Some((key, my_distance, keypos, tmp_keys)) = queue.pop_front() {
        if po.verbose {
            println!("checking key = {} | dist = {} | pos = {:?} | tmp_keys = {:?}", key, my_distance, keypos, tmp_keys);
        }
        if key == target_key {
            return my_distance;
        }

        let person = Person {
            xx: keypos.0,
            yy: keypos.1,
            keys: tmp_keys.to_owned(),
            steps: my_distance,
        };
        for (key, distance, keypos) in maze.reachable_keys(&person) {
            /*if key == target_key {
                return my_distance + distance;
            }*/

            if !queue.iter().any(|&(check_key, _, _, _)| check_key == key) && !checked.contains(&key) {
                let new_tmp_keys = {
                    let mut keys = tmp_keys.to_owned();
                    keys.push(key);
                    keys
                };
                queue.push_back((key, my_distance + distance, keypos, new_tmp_keys));
            }
        }

        checked.push(key);
        if checked.len() == number_of_keys {
            return my_distance;
        }
    }

    0
}

fn part2(po: &TodaysPuzzleOptions, res1: Option<OutputType1>) -> OutputType2 {
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
        test_case_helper("example1", 8, 0)
    }

    #[test]
    fn example_2() {
        test_case_helper("example2", 86, 0)
    }

    #[test]
    fn example_3() {
        test_case_helper("example3", 132, 0)
    }

    #[test]
    fn example_4() {
        test_case_helper("example4", 136, 0)
    }

    #[test]
    fn example_5() {
        test_case_helper("example5", 81, 0)
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
