/*

BENCHMARK RESULTS

*/

// allow bench feature when using unstable flag
// use: $ cargo +nightly bench --features unstable
#![cfg_attr(feature = "unstable", feature(test))]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

use aoc_import_magic::{import_magic, PuzzleOptions};
use regex::Regex;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    io,
};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const DAY: i32 = 12;
type InputTypeSingle = Point;
type InputType = Vec<InputTypeSingle>;
type OutputType1 = isize;
type OutputType2 = OutputType1;
type TodaysPuzzleOptions = PuzzleOptions<InputType>;

#[derive(Clone, Copy, Debug, Default, Deserialize, Hash, Eq, PartialEq, Serialize)]
struct Point {
    xx: isize,
    yy: isize,
    zz: isize,
    vel_xx: isize,
    vel_yy: isize,
    vel_zz: isize,
}

impl Point {
    fn potential_energy(&self) -> isize {
        self.xx.abs() + self.yy.abs() + self.zz.abs()
    }

    fn kinetic_energy(&self) -> isize {
        self.vel_xx.abs() + self.vel_yy.abs() + self.vel_zz.abs()
    }

    fn total_energy(&self) -> isize {
        self.potential_energy() * self.kinetic_energy()
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

            // regex parsing stuff
            lazy_static! {
                // (?x)
                // (?P<name>xxx)
                static ref RE: Regex = Regex::new(
                    r"<x=(?P<xx>-?\d+), y=(?P<yy>-?\d+), z=(?P<zz>-?\d+)>"
                ).unwrap();
            }

            let caps = RE.captures(&line).unwrap();
            //let thingy = &caps["thingy"];
            let xx = caps["xx"].parse().unwrap();
            let yy = caps["yy"].parse().unwrap();
            let zz = caps["zz"].parse().unwrap();

            println!("{} {} {}", xx, yy, zz);
            Point {
                xx,
                yy,
                zz,
                vel_xx: 0,
                vel_yy: 0,
                vel_zz: 0,
            }

        })
        .collect()
}

fn apply_gravity(moons: &mut Vec<Point>) {
    //let mut new_moons = moons.to_owned();

    for xx in 0 .. moons.len() {
        for yy in 0 .. xx {
            if xx == yy {
                continue;
            }

            let orig_aa = &mut moons[xx];
            let orig_bb = &mut moons[yy];
            //let new_aa = &mut new_moons[xx];
            //let new_bb = &mut new_moons[yy];

            if moons[xx].xx < moons[yy].xx {
                moons[xx].vel_xx += 1;
                moons[yy].vel_xx -= 1;
            } else if moons[xx].xx > moons[yy].xx {
                moons[xx].vel_xx -= 1;
                moons[yy].vel_xx += 1;
            }

            if moons[xx].yy < moons[yy].yy {
                moons[xx].vel_yy += 1;
                moons[yy].vel_yy -= 1;
            } else if moons[xx].yy > moons[yy].yy {
                moons[xx].vel_yy -= 1;
                moons[yy].vel_yy += 1;
            }

            if moons[xx].zz < moons[yy].zz {
                moons[xx].vel_zz += 1;
                moons[yy].vel_zz -= 1;;
            } else if moons[xx].zz > moons[yy].zz {
                moons[xx].vel_zz -= 1;
                moons[yy].vel_zz += 1;
            }
        }
    }

    //new_moons
}

fn apply_velocity(moons: &mut Vec<Point>) {
    for moon in moons.iter_mut() {
        moon.xx += moon.vel_xx;
        moon.yy += moon.vel_yy;
        moon.zz += moon.vel_zz;
    }
}

fn calc_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn part1(po: &TodaysPuzzleOptions) -> OutputType1 {
    let mut moons = po.data.as_ref().unwrap().to_owned();

    for ii in 0 .. 1000 {
        apply_gravity(&mut moons);
        apply_velocity(&mut moons);

        //println!("step {}", ii);
        //dbg!(&moons);
    }

    moons.into_iter().map(|moon| moon.total_energy()).sum()
}

fn part2(po: &TodaysPuzzleOptions, res1: Option<OutputType1>) -> OutputType2 {
    let mut moons = po.data.as_ref().unwrap().to_owned();
    let mut history = HashSet::new();

    let hash = calc_hash(&moons);
    history.insert(hash);
    let mut count = 0;
    loop {
        count += 1;

        apply_gravity(&mut moons);
        apply_velocity(&mut moons);

        let hash = calc_hash(&moons);
        if history.contains(&hash) {
            break count;
        }

        history.insert(hash);

        if count % 1_000_000 == 0 {
            println!("iteration {}", count)
        }
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
