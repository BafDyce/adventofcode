/*

BENCHMARK RESULTS

*/

// allow bench feature when using unstable flag
// use: $ cargo +nightly bench --features unstable
#![cfg_attr(feature = "unstable", feature(test))]



use nalgebra::{ComplexField, RealField};

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
type OutputType1 = (usize, (usize, usize));
type OutputType2 = (usize, usize);
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
        println!("Part 1 result: {:?}", res1);
        Some(res1)
    };

    let res2 = part2(&puzzle, res1);
    println!("Part 2 result: {:?}", res2);
    println!("{}", 100 * res2.0 + res2.1);

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
    get_hitable_asteroids(space, xx, yy).len()
}

fn get_hitable_asteroids(space: &Vec<Vec<Position>>, xx: usize, yy: usize) -> HashMap<(usize, usize), f64> {
    let max_x = space.len();
    let max_y = space[0].len();

    let mut detected = HashMap::<(usize, usize), f64>::new();

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
                    detected.insert((check_x, check_y), calc_winkel((check_x, check_y)));
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
                        detected.insert((check_x, check_y), calc_winkel((check_x, check_y)));
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
                        detected.insert((check_x, check_y), calc_winkel((check_x, check_y)));
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
                        detected.insert((check_x, check_y), calc_winkel((check_x, check_y)));
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

    detected
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

    (max_count, pos)
}

#[derive(Debug)]
struct Laser {
    xx: usize,
    yy: usize,
    rotation: f64,
}

fn part2(po: &TodaysPuzzleOptions, res1: Option<OutputType1>) -> OutputType2 {
    let mut space = po.data.as_ref().unwrap().to_owned();

    let xx = (res1.unwrap().1).0;
    let yy = (res1.unwrap().1).1;

    let mut laser = Laser {
        xx: xx,
        yy: yy,
        rotation: 0f64,
    };

    let mut count = 0;
    loop {
        match shoot_next(&mut space, &mut laser) {
            Some(pos) => {
                count += 1;
                if count == 200 {
                    return pos;
                }
            }
            _ => panic!("oO count = {}", count),
        }
    }
/*
    let bb = (3f64, 2f64);

    let winkel = calc_winkel((3, 2));
    println!("{}", winkel);
*/
    (0, 0)
}

fn shoot_next(space: &mut Vec<Vec<Position>>, laser: &mut Laser) -> Option<(usize, usize)> {
    let asteroids = get_hitable_asteroids(space, laser.xx, laser.yy);
    println!("number of asteroids: {}", asteroids.len());
    dbg!(&asteroids);

    let mut candidates: Vec<((usize, usize), f64)> = asteroids.into_iter().filter(|&(kk, vv)| vv >= laser.rotation).collect();
    //candidates.sort_by_key(|(kk, vv)| vv);
    candidates.sort_by(| (_, aa), (_, bb)| aa.partial_cmp(bb).unwrap());
    return Some(candidates[199].0);

    if candidates.is_empty() {
        let mut candidates: Vec<((usize, usize), f64)> = get_hitable_asteroids(space, laser.xx, laser.yy).into_iter().filter(|&(kk, vv)| !vv.is_nan()).collect();
        //candidates.sort_by_key(|(kk, vv)| vv);
        //dbg!(&candidates);
        candidates.sort_by(| (_, aa), (_, bb)| aa.partial_cmp(bb).unwrap());

        let pos = candidates[0].0;
        space[pos.0][pos.1] = Position::Empty;

        laser.rotation = dbg!(candidates[0].1);
        return Some(pos);
    }

    let (pos, rotation) = candidates[0];
    println!("{:?}", (pos, rotation));

    space[pos.0][pos.1] = Position::Empty;
    laser.rotation = dbg!(rotation);

    Some(pos)
}

fn calc_winkel(pos: (usize, usize)) -> f64 {
    let pos = (pos.0 as f64, pos.1 as f64);

    let tmp: f64 = ComplexField::from_real( (pos.0 * pos.0 + pos.1 * pos.1) as f64 );
    let radius = tmp.sqrt();

    let tmp: f64 = (pos.0 * pos.0 - pos.1 * pos.1 + radius * radius) / (2f64 * pos.0 * radius);
    let winkel = tmp.acos();

    //let pi: f64 = RealField::pi();
    //let grad: f64 = (360f64 / (2f64 * pi) ) * winkel;

    winkel
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
