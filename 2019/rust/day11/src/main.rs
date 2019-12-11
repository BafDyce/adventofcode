/*
      -------Part 1--------   -------Part 2--------
Day       Time  Rank  Score       Time  Rank  Score
 11   00:50:32  1393      0   00:57:12  1215      0
BENCHMARK RESULTS

*/

// allow bench feature when using unstable flag
// use: $ cargo +nightly bench --features unstable
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
    io,
};

const DAY: i32 = 11;
type InputTypeSingle = IntcodeNumber;
type InputType = Vec<InputTypeSingle>;
type OutputType1 = usize;
type OutputType2 = OutputType1;
type TodaysPuzzleOptions = PuzzleOptions<InputType>;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
struct Robot {
    xx: i128,
    yy: i128,
    dir: Dir,
}

impl Robot {
    fn pos(&self) -> (i128, i128) {
        (self.xx, self.yy)
    }

    fn turn(&mut self, dir: i128) {
        self.dir.turn(dir)
    }

    fn step(&mut self) {
        match &self.dir {
            Dir::Up => self.xx += 1,
            Dir::Down => self.xx -= 1,
            Dir::Left => self.yy -= 1,
            Dir::Right => self.yy += 1,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn turn(&mut self, dir: i128) {
        *self = match *self {
            Dir::Up if dir == 0 => Dir::Left,
            Dir::Up if dir == 1 => Dir::Right,
            Dir::Right if dir == 0 => Dir::Up,
            Dir::Right if dir == 1 => Dir::Down,
            Dir::Down if dir == 0 => Dir::Right,
            Dir::Down if dir == 1 => Dir::Left,
            Dir::Left if dir == 0 => Dir::Down,
            Dir::Left if dir == 1 => Dir::Up,
            _ => panic!("Invalid turn spec! {:?}, {}", self, dir),
        }
    }
}

enum Color {
    White,
    Black,
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
    input[0]
        .split(",")
        .map(|xx| xx.parse::<InputTypeSingle>().unwrap())
        .collect()
}

fn part1(po: &TodaysPuzzleOptions) -> OutputType1 {
    let program = po.data.as_ref().unwrap();
    let mut panel: HashMap<(i128, i128), i128> = HashMap::new();

    let mut robot_cpu = IntcodeProcessor::new(program);
    let mut robot = Robot {
        xx: 0,
        yy: 0,
        dir: Dir::Up,
    };

    let mut outputs = VecDeque::new();

    loop {
        //let current_color = panel.get(&robot.pos()).unwrap();
        //let current_color = *panel.entry(robot.pos()).or_insert(0);
        let current_color = match panel.get(&robot.pos()) {
            Some(color) => *color,
            None => 0,
        };
        assert!(outputs.len() == 0);
        if let Some(_finished) = robot_cpu.run(current_color, &mut outputs) {
            break;
        };

        let color = outputs.pop_front().unwrap();
        let dir = outputs.pop_front().unwrap();

        panel.insert( robot.pos(), color );
        robot.turn(dir);
        robot.step();
    }

    panel.len()
}

fn part2(po: &TodaysPuzzleOptions, res1: Option<OutputType1>) -> OutputType2 {
    let program = po.data.as_ref().unwrap();
    let mut panel: HashMap<(i128, i128), i128> = HashMap::new();

    let mut robot_cpu = IntcodeProcessor::new(program);
    let mut robot = Robot {
        xx: 0,
        yy: 0,
        dir: Dir::Up,
    };

    panel.insert(robot.pos(), 1);
    let mut outputs = VecDeque::new();

    loop {
        //let current_color = panel.get(&robot.pos()).unwrap();
        //let current_color = *panel.entry(robot.pos()).or_insert(0);
        let current_color = match panel.get(&robot.pos()) {
            Some(color) => *color,
            None => 0,
        };
        assert!(outputs.len() == 0);
        if let Some(_finished) = robot_cpu.run(current_color, &mut outputs) {
            break;
        };

        let color = outputs.pop_front().unwrap();
        let dir = outputs.pop_front().unwrap();

        panel.insert( robot.pos(), color );
        robot.turn(dir);
        robot.step();
    }

    let mut xx_min = std::i128::MAX;
    let mut xx_max = std::i128::MIN;
    let mut yy_min = std::i128::MAX;
    let mut yy_max = std::i128::MIN;

    for (xx, yy) in panel.keys() {
        xx_min = i128::min(xx_min, *xx);
        xx_max = i128::max(xx_max, *xx);
        yy_min = i128::min(yy_min, *yy);
        yy_max = i128::max(yy_max, *yy);
    }

    for xx in xx_min ..= xx_max {
        for yy in yy_min ..= yy_max {
            let cc = match panel.get( &(xx, yy) ) {
                Some(0) => '.',
                Some(1) => '#',
                _ => '.',
            };
            print!("{}", cc);
        }
        println!("");
    }

    panel.len()
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
