extern crate aoc_utils;

mod part1;
mod part2;

use aoc_utils::prelude::*;
use std::{collections::{HashMap, VecDeque}, env};
use std::str::FromStr;
use std::fmt;

const DAY: u32 = 15;
type InputTypeSingle = Field;
type InputType = Vec<Vec<InputTypeSingle>>;

#[derive(Debug, Clone)]
pub enum ParseError {
    EmptyString,
    InvalidChar(char),
    InvalidUnit(char),
}

#[derive(Debug, Clone)]
pub enum Field {
    Empty,
    Wall,
    Unit(Unit),
}

impl FromStr for Field {
    type Err = ParseError;

    fn from_str(ss: &str) -> Result<Self, Self::Err> {
        match ss.chars().next() {
            None => return Err(ParseError::EmptyString),
            Some('#') => Ok(Field::Wall),
            Some('.') => Ok(Field::Empty),
            Some('E') | Some('G') => match ss.parse::<Unit>() {
                Ok(unit) => Ok(Field::Unit(unit)),
                Err(err) => Err(err),
            },
            Some(cc) => Err(ParseError::InvalidChar(cc))
        }
    }
}

impl fmt::Display for Field {
    fn fmt(&self, ff: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Field::Empty => write!(ff, "."),
            Field::Wall => write!(ff, "#"),
            Field::Unit(unit) => unit.fmt(ff),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Unit {
    utype: UnitType,
    hp: isize,
    attack_power: isize,
    rounds_processed: isize,
    id: usize,
}

impl Unit {
    pub fn is_enemy(&self, other: &Unit) -> bool {
        self.utype != other.utype
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnitType {
    Elf,
    Goblin,
}

impl FromStr for Unit {
    type Err = ParseError;

    fn from_str(ss: &str) -> Result<Self, Self::Err> {
        let utype = match ss.chars().next() {
            None => return Err(ParseError::EmptyString),
            Some('E') => UnitType::Elf,
            Some('G') => UnitType::Goblin,
            Some(cc) => return Err(ParseError::InvalidUnit(cc)),
        };

        Ok(Unit {
            utype,
            hp: 200,
            attack_power: 3,
            rounds_processed: -1,
            id: 0,
        })
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, ff: &mut fmt::Formatter) -> fmt::Result {
        match self.utype {
            UnitType::Elf => write!(ff, "E"),
            UnitType::Goblin => write!(ff, "G"),
        }
    }
}

fn get_adjacents(loc: &Location2D) -> [Location2D; 4] {
    [
        Location2D::new( loc.xx() - 1, loc.yy() ),
        Location2D::new( loc.xx(), loc.yy() - 1 ),
        Location2D::new( loc.xx(), loc.yy() + 1),
        Location2D::new( loc.xx() + 1, loc.yy() ),
    ]
}

pub fn process_cell(field: &mut Vec<Vec<Field>>, cell: Location2D, round: isize) -> Option<usize> {
    //println!("Started processing unit @ {:?}", cell);
    //print_field(field);

    if let Field::Unit(unit) = field[cell.xx_as_usize()][cell.yy_as_usize()].to_owned() {
        // prevent processing of unit which moved into the place of cell after killing the original
        // unit in cell.
        if unit.rounds_processed >= round {
            return None;
        }
        // 1. SEARCH (if not adjacent to enemy)
        let mut move_required = true;
        for adjacent in get_adjacents(&cell).into_iter() {
            if let Field::Unit(ref other) = field[adjacent.xx_as_usize()][adjacent.yy_as_usize()] {
                if unit.is_enemy(other) {
                    move_required = false;
                    break;
                }
            }
        }

        let mypos = if move_required {
            // SEARCH
            let mut queue = VecDeque::new();

            let mut meta = HashMap::new();
            queue.push_back( cell.to_owned() );
            meta.insert( cell.to_owned(), Vec::new());

            let mut enemies = Vec::new();

            while ! queue.is_empty() {
                let here = queue.pop_front().unwrap();
                let path_to_here = meta.get(&here).unwrap().clone();

                for adjacent in get_adjacents(&here).into_iter() {
                    if queue.contains(&adjacent) || meta.get(&adjacent).is_some() {
                        continue;
                    }

                    let add_to_queue = match field[adjacent.xx() as usize][adjacent.yy() as usize] {
                        Field::Wall => false,
                        Field::Empty => {
                            meta.insert(
                                adjacent.to_owned(),
                                {
                                    let mut path = path_to_here.clone();
                                    path.push(adjacent.to_owned());
                                    path
                                }
                            );
                            true
                        }
                        Field::Unit(ref other) => {
                            if unit.is_enemy(other) {
                                meta.insert(
                                    adjacent.to_owned(),
                                    Vec::new(),
                                );

                                enemies.push({
                                    let mut path = path_to_here.clone();
                                    path.push(adjacent.to_owned());
                                    path
                                })
                            }
                            false
                        },
                    };

                    if add_to_queue {
                        queue.push_back( adjacent.to_owned() );
                    }
                }
            }


            enemies.sort_by(|aa, bb| aa.len().cmp(&bb.len()));
            //println!("Found paths to enemies: {:?}", enemies);
            let mut targets = Vec::new();
            for path in enemies.into_iter() {
                for adjacent in get_adjacents(&path[path.len()-1]).into_iter() {
                    if let Some(path) = meta.get(adjacent) {
                        if !path.is_empty() {
                            targets.push( path );
                        }
                    }
                }
            }

            targets.sort_by(|aa, bb| aa.len().cmp(&bb.len()));
            //println!("Targets:");
            //for (ii, target) in targets.iter().enumerate() {
            //    println!(" [{:2}] {:?}", ii, target);
            //}

            // MOVE
            if !targets.is_empty() {
                let shortest = targets[0].len();
                let mut target_options: Vec<_> = targets.into_iter()
                    .filter(|path| path.len() == shortest)
                    .map(|path| (path.last().unwrap().to_owned(), path))
                    .collect();

                target_options.sort_unstable_by(|(aa_target, _aa_path), (bb_target, _bb_path)| {
                    aa_target.cmp(bb_target)
                });

                let step_to = &target_options[0].1[0];
                field[step_to.xx_as_usize()][step_to.yy_as_usize()] = field[cell.xx_as_usize()][cell.yy_as_usize()].to_owned();
                field[cell.xx_as_usize()][cell.yy_as_usize()] = Field::Empty;

                //println!("Field after move:");
                //print_field(field);

                step_to.to_owned()
            } else {
                cell
            }
        } else {
            cell
        };

        #[derive(Debug)]
        struct TargetData {
            loc: Location2D,
            hp: isize,
        };
        let mut enemies_to_attack = Vec::new();
        for adjacent in get_adjacents(&mypos).into_iter() {
            if let Field::Unit(ref other) = field[adjacent.xx_as_usize()][adjacent.yy_as_usize()] {
                if unit.is_enemy(other) {
                    enemies_to_attack.push( TargetData {
                        loc: adjacent.to_owned(),
                        hp: other.hp,
                    });
                }
            }
        }
        //println!("Enemies to attack: {:?}", enemies_to_attack);

        let result = if !enemies_to_attack.is_empty() {
            // stable sort
            enemies_to_attack.sort_by(|aa, bb| aa.hp.cmp(&bb.hp));
            let attacked_pos = &enemies_to_attack[0].loc;
            let attacked_cell = &mut field[attacked_pos.xx_as_usize()][attacked_pos.yy_as_usize()];
            //println!("Attacking enemy {:?} @ pos {:?}", attacked_cell, attacked_pos);
            let (it_died, unit_killed) = if let Field::Unit(ref mut attacked_unit) = attacked_cell {
                attacked_unit.hp -= unit.attack_power;
                // did it die?
                (attacked_unit.hp <= 0, Some(attacked_unit.id))
            } else {
                println!("ERROR: We tried to attack a non-unit");
                (false, None)
            };

            if it_died {
                *attacked_cell = Field::Empty;
                unit_killed
            } else {
                None
            }
        } else {
            None
        };

        if let Field::Unit(ref mut me_in_field) = field[mypos.xx_as_usize()][mypos.yy_as_usize()] {
            me_in_field.rounds_processed += 1;
        } else {
            println!("ERROR: I disappeared unexpectedly!");
        }

        return result;
    }

    //enter_to_continue();
    None
}

pub fn print_field(field: &Vec<Vec<Field>>) {
    for row in field {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
    println!();
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
    let (res1, _) = part1::solve(&input, &puzzle_config);
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
    let data: InputType = input.into_iter()
        .map(|line| {
            (0 .. line.len()).map(|idx| line[idx..].parse::<Field>().unwrap()).collect()
        })
        .collect();

    if verbose {
        println!("input parsed: {:?}", data);
        println!("config: {:?}", puzzle_config);
    }
    (data, puzzle_config)
}
