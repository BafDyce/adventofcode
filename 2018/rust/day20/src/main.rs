extern crate aoc_utils;

mod part1;
mod part2;

use aoc_utils::prelude::*;
use std::{cmp, collections::{HashMap, VecDeque}, env, str::Chars};

const DAY: u32 = 20;
type InputType = String;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Door {
    Yes,
    No,
    Unknown,
}

impl Door {
    pub fn as_char(&self) -> char {
        match self {
            Door::Yes => '-',
            Door::No => '#',
            Door::Unknown => '?',
        }
    }
}

impl Default for Door {
    fn default() -> Door {
        Door::Unknown
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Room {
    pub distance: usize,

    pub north: Door,
    pub east: Door,
    pub south: Door,
    pub west: Door,
}

impl Room {
    pub fn new() -> Self {
        Room::default()
    }
}

pub fn fill(
    iter: &mut Chars,
    start_pos: &mut Location2D,
    map: &mut HashMap<Location2D, Room>
) {
    let mut pos = start_pos.to_owned();
    while let Some(cc) = iter.next() {
        match cc {
            'N' => {
                {
                    let room = map.entry(pos.to_owned()).or_insert(Room::default());
                    room.north = Door::Yes;
                }

                *pos.xx_mut() -= 1;
                {
                    let room = map.entry(pos.to_owned()).or_insert(Room::default());
                    room.south = Door::Yes;
                }
            }
            'S' => {
                {
                    let room = map.entry(pos.to_owned()).or_insert(Room::default());
                    room.south = Door::Yes;
                }

                *pos.xx_mut() += 1;
                {
                    let room = map.entry(pos.to_owned()).or_insert(Room::default());
                    room.north = Door::Yes;
                }
            }
            'W' => {
                {
                    let room = map.entry(pos.to_owned()).or_insert(Room::default());
                    room.west = Door::Yes;
                }

                *pos.yy_mut() -= 1;
                {
                    let room = map.entry(pos.to_owned()).or_insert(Room::default());
                    room.east = Door::Yes;
                }
            }
            'E' => {
                {
                    let room = map.entry(pos.to_owned()).or_insert(Room::default());
                    room.east = Door::Yes;
                }

                *pos.yy_mut() += 1;
                {
                    let room = map.entry(pos.to_owned()).or_insert(Room::default());
                    room.west = Door::Yes;
                }
            }
            '(' => {
                // We entered a new sub-path-branch
                // recursion!
                fill(iter, &mut pos, map);
            }
            ')' => return,
            '|' => {
                // We finished processing of a branch, therefore, the next starts.
                // so lets go back to the start position and continue with this branch
                pos = start_pos.to_owned();
            }
            _ => {}
        }
    }
}

pub fn calc_distances(map: &mut HashMap<Location2D, Room>) -> (Location2D, Room) {
    let mut queue = VecDeque::new();
    queue.push_back(Location2D::new(0, 0));

    let mut meta = HashMap::new();
    meta.insert(Location2D::new(0, 0), 0);

    while let Some(loc) = queue.pop_front() {
        match map.get_mut(&loc) {
            Some(room) => {
                let dist = *meta.get(&loc).unwrap();
                room.distance = dist;

                if let Door::Yes = room.north {
                    let next_loc = Location2D::new(loc.xx() - 1, loc.yy());
                    if !queue.contains(&next_loc) && meta.get(&next_loc).is_none() {
                        meta.insert(next_loc.to_owned(), dist + 1);
                        queue.push_back(next_loc.to_owned());
                    }
                }

                if let Door::Yes = room.south {
                    let next_loc = Location2D::new(loc.xx() + 1, loc.yy());
                    if !queue.contains(&next_loc) && meta.get(&next_loc).is_none() {
                        meta.insert(next_loc.to_owned(), dist + 1);
                        queue.push_back(next_loc.to_owned());
                    }
                }

                if let Door::Yes = room.west {
                    let next_loc = Location2D::new(loc.xx(), loc.yy() - 1);
                    if !queue.contains(&next_loc) && meta.get(&next_loc).is_none() {
                        meta.insert(next_loc.to_owned(), dist + 1);
                        queue.push_back(next_loc.to_owned());
                    }
                }

                if let Door::Yes = room.east {
                    let next_loc = Location2D::new(loc.xx(), loc.yy() + 1);
                    if !queue.contains(&next_loc) && meta.get(&next_loc).is_none() {
                        meta.insert(next_loc.to_owned(), dist + 1);
                        queue.push_back(next_loc.to_owned());
                    }
                }
            },
            _ => panic!("Error! tried to enter non existing room!"),
        };
    }

    let furthest = map.iter().max_by(|(_, aa_room), (_, bb_room)| {
        usize::cmp(&aa_room.distance, &bb_room.distance)
    }).unwrap();

    (furthest.0.to_owned(), furthest.1.to_owned())
}

pub fn fancy_print(map: &HashMap<Location2D, Room>, fin: bool) {
    let mut xx_max = isize::min_value();
    let mut xx_min = isize::max_value();
    let mut yy_max = isize::min_value();
    let mut yy_min = isize::max_value();

    for (pos, _) in map.iter() {
        xx_max = cmp::max(xx_max, pos.xx());
        xx_min = cmp::min(xx_min, pos.xx());
        yy_max = cmp::max(yy_max, pos.yy());
        yy_min = cmp::min(yy_min, pos.yy());
    }

    let mut only_middle = true;
    for xx in xx_min ..= xx_max {
        let mut rows = [Vec::new(), Vec::new(), Vec::new()];
        let mut first = true;

        only_middle = !only_middle;

        rows[0].push('#');
        rows[2].push('#');

        for yy in yy_min ..= yy_max {
            let loc = Location2D::new(xx, yy);
            match map.get(&loc) {
                Some(room) => {
                    if first {
                        rows[1].push(room.west.as_char());
                    }
                    if only_middle {
                        rows[1].extend(&['.', room.east.as_char()]);
                    } else {
                        rows[0].extend(&[room.north.as_char(), '#']);
                        rows[1].extend(&['.', room.east.as_char()]);
                        rows[2].extend(&[room.south.as_char(), '#']);
                    }
                },
                _ => {
                    if first {
                        rows[1].push('#');
                    }
                    if only_middle {
                        rows[1].extend(&['.', '?']);
                    } else {
                        rows[0].extend(&['?', '#']);
                        rows[1].extend(&['.', '?']);
                        rows[2].extend(&['?', '#']);
                    }
                }
            }
            first = false;
        }

        if only_middle {
            let mut output: String = rows[1].iter().collect();
            output = output.replace('-', "|");
            if fin {
                output = output.replace('?', "#");
            }
            println!("{}", output);
        } else {
            let mut output: String = rows[0].iter().collect();
            if fin {
                output = output.replace('?', "#");
            }
            println!("{}", output);

            let mut output: String = rows[1].iter().collect();
            output = output.replace('-', "|");
            if fin {
                output = output.replace('?', "#");
            }
            println!("{}", output);

            let mut output: String = rows[2].iter().collect();
            if fin {
                output = output.replace('?', "#");
            }
            println!("{}", output);
        }
    }
}

fn main() {
    // READ input
    let args: Vec<String> = env::args().collect();

    // Parse command line arguments
    let input_name = if args.len() > 1 {
        &args[1]
    } else {
        "puzzle1"
    };
    let verbose = args.contains(&String::from("-v")) || args.contains(&String::from("--verbose"));

    if verbose {
        println!("Loading data from input file {}", input_name);
    }
    // READ & PARSE input
    let (input, puzzle_config) = parse_input(input_name, verbose);

    // SOLVE puzzles
    let res1 = part1::solve(&input, &puzzle_config);
    let res2 = part2::solve(&input, &puzzle_config);

    println!("results: {} and {}", res1, res2);
}

fn parse_input(input_name: &str, verbose: bool) -> (InputType, PuzzleConfig) {
    let config = ImportConfig::new(2018, DAY, "../../_inputs/day{day}/");
    let (input, puzzle_config) = import_with_puzzle_config(&config, input_name).unwrap();
    if verbose {
        println!("raw input: {:?}", input);
    }

    // PARSE input
    let data: InputType = input.into_iter().take(1).next().unwrap();

    if verbose {
        println!("input parsed: {:?}", data);
        println!("config: {:?}", puzzle_config);
    }
    (data, puzzle_config)
}
