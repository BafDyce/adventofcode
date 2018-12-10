extern crate aoc_utils;
#[macro_use] extern crate lazy_static;
extern crate regex;

use aoc_utils::prelude::*;
use regex::Regex;
use std::{env, io};

#[derive(Debug, Clone)]
pub struct Point {
    pos: Location2D,
    velx: i64,
    vely: i64
}

impl Point {
    pub fn new(posx: i64, posy: i64, velx: i64, vely: i64) -> Point {
        Point {
            pos: Location2D::new(posx, posy),
            velx: velx,
            vely: vely,
        }
    }

    pub fn travel(&mut self) {
        *self.pos.xx_mut() += self.velx;
        *self.pos.yy_mut() += self.vely;
    }

    pub fn position(&self) -> Location2D {
        self.pos.to_owned()
    }
}

pub fn fancy_print(grid: &InfiniteGrid<char>) -> bool {
    if grid.iter().count() == 0 {
        //println!("Grid is empty!");
        false
    } else {
        let [
            _,
            loc_max,
            loc_min,
            _
        ] = grid.get_boundaries();

        if loc_max.yy() - loc_min.yy() < 250 || loc_max.xx() - loc_min.xx() < 250 {
            for yy in loc_min.yy() ..= loc_max.yy() {
                for xx in loc_min.xx() ..= loc_max.xx() {
                    match grid.get_value(&Location2D::new(xx, yy)) {
                        None => print!("."),
                        Some(cc) => print!("{}", cc)
                    }
                }
                println!()
            }

            true
        } else {
            false
        }
    }

}

const DAY: u32 = 10;
type InputTypeSingle = Point;
type InputType = Vec<InputTypeSingle>;

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
    let (mut points, _) = parse_input(input_name, verbose);

    let mut cnt = 0;
    loop {
        cnt += 1;
        let mut grid: InfiniteGrid<char> = InfiniteGrid::new();

        for point in points.iter_mut() {
            (*point).travel();
            let pos = point.position();
            grid.set_value(&pos, '#');
        }

        if fancy_print(&grid) {
            println!("counter: {}", cnt);
            let mut stdin = String::new();
            io::stdin().read_line(&mut stdin);
        }
    }
}

fn parse_input(input_name: &str, verbose: bool) -> (InputType, PuzzleConfig) {
    let config = ImportConfig::new(2018, DAY, "../../_inputs/day{day}/");
    let (input, puzzle_config) = import_with_puzzle_config(&config, input_name).unwrap();
    if verbose {
        println!("raw input: {:?}", input);
    }

    // PARSE input
    let data: InputType = input.into_iter().map(|line| {
        // Parsing logic

        // regex parsing stuff
        lazy_static! {
            // (?x)
            // (?P<name>\d+)
            static ref RE: Regex = Regex::new(
                r"position=<\s*(?P<posx>-?\d+),\s*(?P<posy>-?\d+)> velocity=<\s*(?P<velx>-?\d+),\s*(?P<vely>-?\d+)>"
            ).unwrap();
        }

        let caps = RE.captures(&line).unwrap();
        // let thingy = &caps["thingy"];
        //println!("caps {:?}", caps);
        let posx = caps["posx"].parse::<i64>().unwrap();
        let posy = caps["posy"].parse::<i64>().unwrap();
        let velx = caps["velx"].parse::<i64>().unwrap();
        let vely = caps["vely"].parse::<i64>().unwrap();
        Point::new(posx, posy, velx, vely)
    })
    .collect();

    if verbose {
        println!("input parsed: {:?}", data);
        println!("config: {:?}", puzzle_config);
    }
    (data, puzzle_config)
}
