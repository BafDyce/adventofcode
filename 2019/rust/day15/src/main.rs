/*
      -------Part 1--------   -------Part 2--------
Day       Time  Rank  Score       Time  Rank  Score
 15   02:05:48   937      0   02:25:05   804      0
BENCHMARK RESULTS

*/

// allow bench feature when using unstable flag
// use: $ cargo +nightly bench --features unstable
#![cfg_attr(feature = "unstable", feature(test))]

#[macro_use]
extern crate serde_derive;

mod intcode;
use intcode::*;

use aoc_import_magic::{import_magic, PuzzleOptions};
use std::{
    collections::{HashMap, VecDeque},
    io::{self, BufRead},
};

const DAY: i32 = 15;
type InputTypeSingle = IntcodeNumber;
type InputType = Vec<InputTypeSingle>;
type OutputType1 = (usize, HashMap<(isize, isize), (Field, usize)>);
type OutputType2 = usize;
type TodaysPuzzleOptions = PuzzleOptions<InputType>;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[repr(isize)]
enum Movement {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

impl Movement {
    fn as_number(&self) -> isize {
        match self {
            Movement::North => 1,
            Movement::South => 2,
            Movement::West => 3,
            Movement::East => 4,
        }
    }

    fn turn_left(&mut self) {
        *self = match self {
            Movement::North => Movement::West,
            Movement::West => Movement::South,
            Movement::South => Movement::East,
            Movement::East => Movement::North,
        }
    }

    fn step_back(&self, (xx, yy): (isize, isize)) -> (isize, isize) {
        match self {
            Movement::North => (xx+1, yy),
            Movement::South => (xx-1, yy),
            Movement::West => (xx, yy+1),
            Movement::East => (xx, yy-1),
        }
    }

    fn get_next_pos(&self, (xx, yy): (isize, isize)) -> (isize, isize) {
        match self {
            Movement::North => (xx-1, yy),
            Movement::South => (xx+1, yy),
            Movement::West => (xx, yy-1),
            Movement::East => (xx, yy+1),
        }
    }

    fn opposit_of(&self, other: Self) -> bool {
        match self {
            Movement::North => other == Movement::South,
            Movement::South => other == Movement::North,
            Movement::West => other == Movement::East,
            Movement::East => other == Movement::West,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
enum Field {
    Empty,
    Wall,
    Oxygen,
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
        println!("Part 1 result: {}", res1.0);
        Some(res1)
    };

    let res2 = part2(&puzzle, res1);
    println!("Part 2 result: {}", res2);

    Ok(())
}

fn parse_input(input: Vec<String>, _config: &HashMap<String, String>, _verbose: bool) -> InputType {
    // IF intcode
    input[0]
        .split(",")
        .map(|xx| xx.parse::<IntcodeNumber>().unwrap())
        .collect()
}

fn part1(po: &TodaysPuzzleOptions) -> OutputType1 {
    let program = po.data.as_ref().unwrap();
    let csv = po.config.get("csv").is_some();

    let mut robot = IntcodeProcessor::new(program);
    let mut area: HashMap<(isize, isize), (Field, usize)> = HashMap::new();
    area.insert( (0, 0), (Field::Empty, 1) );

    let mut outputs = VecDeque::new();
    let mut input = Movement::North;
    let mut next_pos = (-1, 0);

    let stdin = io::stdin();
    let mut stdin = stdin.lock().lines();

    let mut oxygen_found = false;
    let mut countdown = 50_000;

    loop {
        if let Some(result) = robot.run(input.as_number(), &mut outputs, 1) {
            println!("Robot halted {}", result);
            break (0, HashMap::new());
        }

        match outputs.pop_front() {
            None => panic!("No output!"),
            Some(0) => {
                // wall
                // -> turn left
                area.insert( next_pos, (Field::Wall, 1) );
                let current_pos = input.step_back(next_pos);
                loop {
                    input.turn_left();
                    next_pos = input.get_next_pos(current_pos);

                    match area.get(&next_pos) {
                        Some((Field::Empty, _)) | None => break,
                        _ => {},
                    }
                }
            }
            Some(1) => {
                //println!("empty @ {:?}", next_pos);
                // successful move
                //area.insert( next_pos, (Field::Empty, 1) );
                let field = area.entry(next_pos).or_insert((Field::Empty, 0));
                field.1 += 1;

                // lets check the next space in our direction
                next_pos = input.get_next_pos(next_pos);
                match area.get( &next_pos ) {
                    None => {}, // Havent seen that one yet -> go for it!
                    Some(_) => {
                        // We have seen it already, so check all four neighbors, choose:
                        // 1) unknown, if available
                        // 2) empty field (which we have visited the least often so far)
                        let current_pos = input.step_back(next_pos);
                        let mut direction = input;

                        let mut options = Vec::new();
                        for __ in 0 .. 4 {
                            direction.turn_left();
                            let possible_pos = direction.get_next_pos(current_pos);
                            options.push((
                                direction,
                                possible_pos,
                                area.get(&possible_pos),
                            ));
                        }

                        if let Some((new_input, new_next_pos, _)) = options.iter().find(|&&(_, _, field)| field.is_none() ) {
                            // we have an unknown neighbor
                            input = *new_input;
                            next_pos = *new_next_pos;
                        } else if let Some((new_input, new_next_pos, _)) = options
                            .iter()
                            .filter(|&&(dir, _, field)| !input.opposit_of(dir) && field.is_some() && field.unwrap().0 == Field::Empty )
                            .min_by(|(_, _, aa), (_, _, bb)| aa.unwrap().1.cmp(&bb.unwrap().1)) {
                            // multiple adjacent empty spaces => DONT go to the one we just came from and choose the one we have visited the fewest amount of times so far
                            input = *new_input;
                            next_pos = *new_next_pos;
                        } else {
                            // we hit a dead end .. need to go back (turn around by 180 degrees)
                            input.turn_left();
                            input.turn_left();
                            next_pos = input.get_next_pos(current_pos);
                        }
                    }
                }
            }
            Some(2) => {
                // oxygen system found
                let field = area.entry( next_pos).or_insert( (Field::Oxygen, 0));
                field.1 += 1;

                // print only if we found it for the first time
                if field.1 == 1 {
                    print_field(&area, next_pos, csv);
                    println!("oxygen @ {:?}; must find shortest path!", next_pos);
                    oxygen_found = true;
                }

                // start to go in any direction
                input.turn_left();
                next_pos = input.get_next_pos(next_pos);
            }
            Some(other) => {
                panic!("Invalid output {}", other);
            }
        }

        if po.verbose {
            println!("next to check: {:?} | moving {:?}", next_pos, input);
            print_field(&area, input.step_back(next_pos), csv);
            let _ = stdin.next();
        }

        if oxygen_found {
            countdown -= 1;

            if countdown == 0 {
                println!("END{}:", if csv {""} else {"(run with --config csv true to get csv-formatted output)"});
                print_field(&area, input.step_back(next_pos), csv);
                break (0, area);
            }
        }
    }
}

fn oxygen_spread(area: &mut HashMap<(isize, isize), (Field, usize)>) -> usize {
    let mut neighbors = Vec::new();
    for ( (xx, yy), (field, _) ) in area.iter() {
        if *field == Field::Oxygen {
            neighbors.push( (*xx - 1, *yy) );
            neighbors.push( (*xx + 1, *yy) );
            neighbors.push( (*xx, *yy - 1) );
            neighbors.push( (*xx, *yy + 1) );
        }
    }

    neighbors.sort();
    neighbors.dedup();

    let mut count = 0;
    for neighbor in neighbors {
        let field = area.entry(neighbor).or_insert( (Field::Wall, 0));
        if field.0 == Field::Empty {
            field.0 = Field::Oxygen;
            count += 1;
        }
    }

    count
}

fn part2(_po: &TodaysPuzzleOptions, res1: Option<OutputType1>) -> OutputType2 {
    let mut area = res1.unwrap().1;
    let mut minutes = 0;

    loop {
        let new_fields = oxygen_spread(&mut area);
        if new_fields == 0 {
            break minutes;
        }

        minutes += 1;
    }
}

fn print_field(
    field: &HashMap<(IntcodeNumber, IntcodeNumber), (Field, usize)>,
    me: (IntcodeNumber, IntcodeNumber),
    csv: bool,
) {
    let csv = if csv {","} else {""};
    let mut xx_min = std::isize::MAX;
    let mut xx_max = std::isize::MIN;
    let mut yy_min = std::isize::MAX;
    let mut yy_max = std::isize::MIN;

    for &(xx, yy) in field.keys() {
        xx_min = isize::min(xx_min, xx);
        xx_max = isize::max(xx_max, xx);
        yy_min = isize::min(yy_min, yy);
        yy_max = isize::max(yy_max, yy);
    }

    for xx in xx_min ..= xx_max {
        for yy in yy_min ..= yy_max {
            let cc = if xx == me.0 && yy == me.1 {
                'x'
            } else if xx == 0 && yy == 0 {
                'o'
            } else {
                match field.get( &(xx, yy) ) {
                    Some((Field::Wall, _)) => '#',
                    Some((Field::Empty, _)) => '.',
                    Some((Field::Oxygen, _)) => 'O',
                    None => ' ',
                }
            };
            print!("{}{}", cc, csv);
        }
        println!("");
    }
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
