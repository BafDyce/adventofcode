/*
      -------Part 1--------   --------Part 2--------
Day       Time  Rank  Score       Time   Rank  Score
 20       >24h  2934      0       >24h   2301      0
BENCHMARK RESULTS
test bench::bench_parsing ... bench:      19,798 ns/iter (+/- 1,833)
test bench::bench_part1   ... bench:   1,304,681 ns/iter (+/- 83,979)
test bench::bench_part2   ... bench: 1,355,935,900 ns/iter (+/- 132,480,755)
*/

// allow bench feature when using unstable flag
// use: $ rustup run nightly cargo bench --features unstable
#![cfg_attr(feature = "unstable", feature(test))]

use aoc_import_magic::{import_magic, PuzzleOptions};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    io,
};

const DAY: i32 = 20;
type InputTypeSingle = char;
type InputType = Vec<Vec<InputTypeSingle>>;
type OutputType1 = usize;
type OutputType2 = OutputType1;
type TodaysPuzzleOptions = PuzzleOptions<InputType>;

#[derive(Debug, PartialEq)]
enum Direction {
    Outwards,
    Inwards,
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

fn parse_input(input: Vec<String>, _config: &HashMap<String, String>, _verbose: bool) -> InputType {
    input
        .into_iter()
        .map(|line| {
            line.chars().collect()
        })
        .collect()
}

fn print_maze(maze: &InputType) {
    for line in maze {
        for field in line {
            print!("{}", field);
        }
        println!("");
    }
}

fn find_teleport_fields(maze: &InputType) -> HashMap<(usize, usize), (String, Direction)> {
    let mut teleport_fields = HashMap::new();

    for xx in 0 .. maze.len() {
        for yy in 0 .. maze[xx].len() {
            if maze[xx][yy].is_ascii_alphabetic() {
                // one part teleport label found, check adjacent fields for second one
                // assumption 1: no two unrelated labels are directly next to each other
                // assumption 2: we will always find the upper/left part first, so we only need to check the field
                //  below/right to us
                if xx + 1 < maze.len() && yy < maze[xx+1].len() && maze[xx+1][yy].is_ascii_alphabetic() {
                    // vertical label found -> need to check fields above and below
                    let name = format!("{}{}", maze[xx][yy], maze[xx+1][yy]);
                    if xx > 0 && maze[xx-1][yy] == '.' {
                        let pos = (xx - 1, yy);
                        let direction = if xx > maze.len() / 2 {
                            Direction::Outwards
                        } else {
                            Direction::Inwards
                        };
                        teleport_fields.insert(pos, (name, direction));
                    } else if xx + 2 < maze.len() && maze[xx+2][yy] == '.' {
                        let pos = (xx + 2, yy);
                        let direction = if xx > maze.len() / 2 {
                            Direction::Inwards
                        } else {
                            Direction::Outwards
                        };
                        teleport_fields.insert(pos, (name, direction));
                    }
                } else if yy + 1 < maze[xx].len() && maze[xx][yy + 1].is_ascii_alphabetic() {
                    // horizontal label found, need to check fields to the left and the right
                    let name = format!("{}{}", maze[xx][yy], maze[xx][yy + 1]);
                    if yy > 0 && maze[xx][yy - 1] == '.' {
                        let pos = (xx, yy - 1);
                        let direction = if yy > maze[xx].len() / 2 {
                            Direction::Outwards
                        } else {
                            Direction::Inwards
                        };
                        teleport_fields.insert(pos, (name, direction));
                    } else if yy + 2 < maze[xx].len() && maze[xx][yy + 2] == '.' {
                        let pos = (xx, yy + 2);
                        let direction = if yy > maze[xx].len() / 2 {
                            Direction::Inwards
                        } else {
                            Direction::Outwards
                        };
                        teleport_fields.insert(pos, (name, direction));
                    }
                }
            }
        }
    }

    teleport_fields
}

fn part1(po: &TodaysPuzzleOptions) -> OutputType1 {
    let maze = po.data.as_ref().unwrap();

    if po.verbose {
        print_maze(&maze);
    }

    let teleport_fields = find_teleport_fields(maze);
    if po.verbose {
        println!("teleport_fields: {:?}", teleport_fields);
    }

    // lets try bfs again :P
    let mut checked = HashSet::new();
    let mut queue = VecDeque::new();
    let start = teleport_fields.iter().find(|&(_, (name, _))| name == "AA").unwrap().0;
    if po.verbose {
        println!("start: {:?}", start);
    }
    queue.push_back((*start, 0, true, vec![(String::from("AA"), 0)]));

    while let Some((pos, dist, through_tele, history)) = queue.pop_front() {
        let adjacent = {
            let mut adj = Vec::new();

            if pos.0 > 0 && pos.1 < maze[pos.0 - 1].len() {
                adj.push( (pos.0 - 1, pos.1) );
            }

            if pos.0 + 1 < maze.len() && pos.1 < maze[pos.0 + 1].len() {
                adj.push( (pos.0 + 1, pos.1) );
            }

            if pos.1 > 0 {
                adj.push( (pos.0, pos.1 - 1) );
            }

            if pos.1 + 1 < maze[pos.0].len() {
                adj.push( (pos.0, pos.1 + 1) );
            }

            adj
        };

        for adj in adjacent.iter() {
            let &(xx, yy) = adj;

            if maze[xx][yy] == '.' {
                if through_tele {
                    if !queue.iter().any(|&(pos, _, _, _)| pos == *adj) && !checked.contains(adj) {
                        queue.push_back((*adj, dist + 1, false, history.clone()));
                    }
                } else {
                    // we didnt go through a tele to come here, so we'll check for one first
                    let tele = match teleport_fields.get(adj) {
                        Some((label, _)) if label == "ZZ" => {
                            if po.verbose {
                                println!("found exit (history: {:?})", history);
                            }
                            return dist + 1;
                        }
                        Some((teleporter_label, _)) if teleporter_label != "AA" => {
                            let tmp = teleport_fields.iter().find(|&(pos, (label, _))| {
                                *pos != *adj && label == teleporter_label
                            }).unwrap();
                            Some((tmp.0.to_owned(), (tmp.1).0.to_owned()))
                        }
                        _ => None,
                    };

                    match tele {
                        Some((exit, label)) => {
                            if !queue.iter().any(|&(pos, _, _, _)| pos == exit) && !checked.contains(&exit) {
                                let mut new_history = history.clone();
                                new_history.push( (label.to_string(), dist) );
                                queue.push_back((exit, dist + 2, true, new_history));
                            }
                        }
                        None => {
                            if !queue.iter().any(|&(pos, _, _, _)| pos == *adj) && !checked.contains(adj) {
                                queue.push_back((*adj, dist + 1, false, history.clone()));
                            }
                        }
                    }
                }

            }
        }

        checked.insert(pos);
    }

    0
}

fn part2(po: &TodaysPuzzleOptions, _res1: Option<OutputType1>) -> OutputType2 {
    let maze = po.data.as_ref().unwrap();

    if po.verbose {
        print_maze(&maze);
    }

    let teleport_fields = find_teleport_fields(maze);
    if po.verbose {
        println!("teleport_fields: {:?}", teleport_fields);
    }

    // lets try bfs again :P
    let mut checked = HashSet::new();
    let mut queue = VecDeque::new();
    let start = teleport_fields.iter().find(|&(_, (name, _))| name == "AA").unwrap().0;
    if po.verbose {
        println!("start: {:?}", start);
    }
    queue.push_back((*start, 0, true, 0usize, vec![(String::from("AA"), 0, 0)]));

    while let Some((pos, dist, through_tele, level, history)) = queue.pop_front() {
        //println!("queue item: pos={:?} dist={} through_tele={} level={} history={:?}", pos, dist, through_tele, level, history);
        let adjacent = {
            let mut adj = Vec::new();

            if pos.0 > 0 && pos.1 < maze[pos.0 - 1].len() {
                adj.push( (pos.0 - 1, pos.1) );
            }

            if pos.0 + 1 < maze.len() && pos.1 < maze[pos.0 + 1].len() {
                adj.push( (pos.0 + 1, pos.1) );
            }

            if pos.1 > 0 {
                adj.push( (pos.0, pos.1 - 1) );
            }

            if pos.1 + 1 < maze[pos.0].len() {
                adj.push( (pos.0, pos.1 + 1) );
            }

            adj
        };

        for adj in adjacent.iter() {
            let &(xx, yy) = adj;

            if maze[xx][yy] == '.' {
                if through_tele {
                    if !queue.iter().any(|&(pos, _, _, lvl, _)| pos == *adj && lvl == level) && !checked.contains(&(*adj, level)) {
                        queue.push_back((*adj, dist + 1, false, level, history.clone()));
                    }
                } else {
                    // we didnt go through a tele to come here, so we'll check for one first
                    let tele = match teleport_fields.get(adj) {
                        Some((label, _)) if label == "ZZ" => {
                            if level == 0 {
                                if po.verbose {
                                    println!("found exit (history: {:?})", history);
                                }
                                return dist + 1;
                            }

                            None
                        }
                        Some((teleporter_label, direction)) if teleporter_label != "AA" => {
                            if level == 0 && *direction == Direction::Outwards {
                                None
                            } else {
                                let tmp = teleport_fields.iter().find(|&(pos, (label, _))| {
                                    *pos != *adj && label == teleporter_label
                                }).unwrap();
                                Some((tmp.0.to_owned(), tmp.1))
                            }
                        }
                        _ => None,
                    };

                    match tele {
                        Some((exit, (label, direction))) => {
                            let new_level = match direction {
                                Direction::Inwards => level - 1,
                                Direction::Outwards => level + 1,
                            };
                            if !queue.iter().any(|&(pos, _, _, lvl, _)| pos == exit && lvl == new_level) && !checked.contains(&(exit, new_level)) {
                                let mut new_history = history.clone();
                                new_history.push( (label.to_string(), dist, new_level) );
                                queue.push_back((exit, dist + 2, true, new_level, new_history));
                            }
                        }
                        None => {
                            if !queue.iter().any(|&(pos, _, _, lvl, _)| pos == *adj && lvl == level) && !checked.contains(&(*adj, level)) {
                                queue.push_back((*adj, dist + 1, false, level, history.clone()));
                            }
                        }
                    }
                }

            }
        }

        checked.insert((pos, level));
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
