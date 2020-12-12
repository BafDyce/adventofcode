/*
      -------Part 1--------   -------Part 2--------
Day       Time  Rank  Score       Time  Rank  Score
 12   00:14:50  1797      0   01:07:18  4369      0

test bench::bench_parsing ... bench:      33,277 ns/iter (+/- 616)
test bench::bench_part1   ... bench:       4,249 ns/iter (+/- 250)
test bench::bench_part2   ... bench:       4,990 ns/iter (+/- 55)

*/

// allow bench feature when using unstable flag
// use: $ rustup run nightly cargo bench --features unstable
#![cfg_attr(feature = "unstable", feature(test))]

use aoc_import_magic::{import_magic, PuzzleOptions};
use std::{
    collections::HashMap,
    io,
};

const DAY: u32 = 12;
type InputTypeSingle = Instruction;
type InputType = Vec<InputTypeSingle>;
type OutputType1 = isize;
type OutputType2 = OutputType1;
type TodaysPuzzleOptions = PuzzleOptions<InputType>;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Instruction {
    action: Action,
    value: isize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Action {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

impl From<char> for Action {
    fn from(from: char) -> Action {
        match from {
            'N' => Action::North,
            'S' => Action::South,
            'E' => Action::East,
            'W' => Action::West,
            'L' => Action::Left,
            'R' => Action::Right,
            'F' => Action::Forward,
            _ => unreachable!(),
        }
    }
}

impl From<String> for Instruction {
    fn from(from: String) -> Instruction {
        Instruction {
            action: Action::from(from.chars().nth(0).unwrap()),
            value: from[1..].parse().unwrap(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn turn_left_90(&mut self) {
        *self = match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    fn turn_right_90(&mut self) {
        *self = match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Object {
    east: isize,
    north: isize,
    direction: Direction,
}

impl Object {
    fn rotate(&mut self, center: &Object, angle: isize) {
        // courtesy to: https://gamefromscratch.com/gamedev-math-recipes-rotating-one-point-around-another-point/
        let angle = angle as f32 * (std::f32::consts::PI / 180f32); // convert to radians
        let new_north = angle.cos() * (self.north - center.north) as f32
            - angle.sin() * (self.east - center.east) as f32
            + center.north as f32;
        let new_east = angle.sin() * (self.north - center.north) as f32
            + angle.cos() * (self.east - center.east) as f32
            + center.east as f32;

        // USE .round() BEFORE CONVERTING TO ISIZE!!! Cost me >half an hour of debugging time -.- :D
        self.north = new_north.round() as isize;
        self.east = new_east.round() as isize;
    }
}

fn parse_input(input: Vec<String>, _config: &HashMap<String, String>, _verbose: bool) -> InputType {
    input
        .into_iter()
        .map(Instruction::from)
        .collect()
}

fn part1(po: &TodaysPuzzleOptions) -> OutputType1 {
    let mut ship = Object {
        east: 0,
        north: 0,
        direction: Direction::East,
    };

    for instr in po.get_data() {
        match instr.action {
            Action::North => ship.north += instr.value,
            Action::South => ship.north -= instr.value,
            Action::East => ship.east += instr.value,
            Action::West => ship.east -= instr.value,
            Action::Left => match instr.value {
                90 => { ship.direction.turn_left_90() }
                180 => { ship.direction.turn_left_90(); ship.direction.turn_left_90() }
                270 => { ship.direction.turn_right_90() }
                _ => unreachable!(),
            }
            Action::Right => match instr.value {
                90 => { ship.direction.turn_right_90() }
                180 => { ship.direction.turn_right_90(); ship.direction.turn_right_90() }
                270 => { ship.direction.turn_left_90() }
                _ => unreachable!(),
            }
            Action::Forward => match ship.direction {
                Direction::North => ship.north += instr.value,
                Direction::South => ship.north -= instr.value,
                Direction::East => ship.east += instr.value,
                Direction::West => ship.east -= instr.value,
            }
        }
    }

    isize::abs(ship.north) + isize::abs(ship.east)
}

fn part2(po: &TodaysPuzzleOptions) -> OutputType2 {
    let mut ship = Object {
        east: 0,
        north: 0,
        direction: Direction::East,
    };

    let mut waypoint = Object {
        east: 10,
        north: 1,
        direction: Direction::North, // unused I think
    };

    for instr in po.get_data() {
        match instr.action {
            Action::North => waypoint.north += instr.value,
            Action::South => waypoint.north -= instr.value,
            Action::East => waypoint.east += instr.value,
            Action::West => waypoint.east -= instr.value,
            Action::Left => {
                waypoint.rotate(&ship, -instr.value);
            }
            Action::Right => {
                waypoint.rotate(&ship, instr.value);
            }
            Action::Forward => {
                let diff_north = waypoint.north - ship.north;
                let diff_east = waypoint.east - ship.east;

                ship.north += diff_north * instr.value;
                waypoint.north += diff_north * instr.value;

                ship.east += diff_east * instr.value;
                waypoint.east += diff_east * instr.value;
            }
        }
    }

    isize::abs(ship.north) + isize::abs(ship.east)
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

    fn test_case_helper(inputname: &str, sol1: OutputType1, sol2: OutputType2) {
        let po = import_helper(inputname);
        let res1 = part1(&po);
        assert_eq!(sol1, res1, "part1");
        let res2 = part2(&po);
        assert_eq!(sol2, res2, "part2");
    }

    #[test]
    fn example_1() {
        test_case_helper("example1", 25, 286)
    }

    #[test]
    fn rotate() {
        let center = Object { north: 0, east: 0, direction: Direction::North };
        let north = Object { north: 5, east: 0, direction: Direction::North };
        let south = Object { north: -5, east: 0, direction: Direction::North };
        let east = Object { north: 0, east: 5, direction: Direction::North };
        let west = Object { north: 0, east: -5, direction: Direction::North };

        let north_east = Object { north: 5, east: 5, direction: Direction::North };
        let south_east = Object { north: -5, east: 5, direction: Direction::North };
        let south_west = Object { north: -5, east: -5, direction: Direction::North };
        let north_west = Object { north: 5, east: -5, direction: Direction::North };

        let mut point = north;

        // rotating left by 90 degrees
        point.rotate(&center, -90);
        assert_eq!(point, west);
        point.rotate(&center, -90);
        assert_eq!(point, south);
        point.rotate(&center, -90);
        assert_eq!(point, east);
        point.rotate(&center, -90);
        assert_eq!(point, north);


        // rotating left by 180 degrees
        point.rotate(&center, -180);
        assert_eq!(point, south);
        point.rotate(&center, -180);
        assert_eq!(point, north);


        // rotating left by 270 degrees
        point.rotate(&center, -270);
        assert_eq!(point, east);
        point.rotate(&center, -270);
        assert_eq!(point, south);
        point.rotate(&center, -270);
        assert_eq!(point, west);
        point.rotate(&center, -270);
        assert_eq!(point, north);

        // =========================================================================================

        // rotating right by 90 degrees
        point.rotate(&center, 90);
        assert_eq!(point, east);
        point.rotate(&center, 90);
        assert_eq!(point, south);
        point.rotate(&center, 90);
        assert_eq!(point, west);
        point.rotate(&center, 90);
        assert_eq!(point, north);


        // rotating right by 180 degrees
        point.rotate(&center, 180);
        assert_eq!(point, south);
        point.rotate(&center, 180);
        assert_eq!(point, north);


        // rotating right by 270 degrees
        point.rotate(&center, 270);
        assert_eq!(point, west);
        point.rotate(&center, 270);
        assert_eq!(point, south);
        point.rotate(&center, 270);
        assert_eq!(point, east);
        point.rotate(&center, 270);
        assert_eq!(point, north);

        // =========================================================================================
        let mut point = north_east;

        // rotating left by 90 degrees
        point.rotate(&center, -90);
        assert_eq!(point, north_west);
        point.rotate(&center, -90);
        assert_eq!(point, south_west);
        point.rotate(&center, -90);
        assert_eq!(point, south_east);
        point.rotate(&center, -90);
        assert_eq!(point, north_east);


        // rotating left by 180 degrees
        point.rotate(&center, -180);
        assert_eq!(point, south_west);
        point.rotate(&center, -180);
        assert_eq!(point, north_east);


        // rotating left by 270 degrees
        point.rotate(&center, -270);
        assert_eq!(point, south_east);
        point.rotate(&center, -270);
        assert_eq!(point, south_west);
        point.rotate(&center, -270);
        assert_eq!(point, north_west);
        point.rotate(&center, -270);
        assert_eq!(point, north_east);
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
