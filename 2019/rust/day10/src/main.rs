/*
      -------Part 1--------   --------Part 2--------
Day       Time  Rank  Score       Time   Rank  Score
 10   00:49:44   955      0       >24h  11771      0
BENCHMARK RESULTS
test bench::bench_parsing ... bench:       5,305 ns/iter (+/- 944)
test bench::bench_part1   ... bench:  24,233,709 ns/iter (+/- 892,249)
test bench::bench_part2   ... bench:      76,488 ns/iter (+/- 3,422)
*/

// allow bench feature when using unstable flag
// use: $ rustup run nightly cargo bench --features unstable
#![cfg_attr(feature = "unstable", feature(test))]

#[macro_use]
extern crate serde_derive;

use aoc_import_magic::{import_magic, PuzzleOptions};
use std::{
    collections::HashMap,
    io,
};

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
enum Position {
    Empty,
    Asteroid,
}

const DAY: i32 = 10;
type InputTypeSingle = Position;
type InputType = Vec<Vec<InputTypeSingle>>;
type OutputType1 = (usize, (usize, usize));
type OutputType2 = (usize, (usize, usize));
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

    Ok(())
}

fn parse_input(input: Vec<String>, _config: &HashMap<String, String>, _verbose: bool) -> InputType {
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

    for step_x in (0 .. max_x).rev() {
        for step_y in (0 .. max_y).rev() {
            if step_x == 0 && step_y == 0 {
                continue;
            }
            // right down
            let mut check_x = xx + step_x;
            let mut check_y = yy + step_y;

            while check_x < max_x && check_y < max_y {
                if space[check_x][check_y] == Position::Asteroid {
                    detected.insert((check_x, check_y), calc_winkel((check_x as isize - xx as isize, check_y as isize - yy as isize)));
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

            // right up
            if xx >= step_x {
                let mut check_x = xx - step_x;
                let mut check_y = yy + step_y;

                while check_y < max_y {
                    if space[check_x][check_y] == Position::Asteroid {
                        detected.insert((check_x, check_y), calc_winkel((check_x as isize - xx as isize, check_y as isize - yy as isize)));
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

            // left up
            if xx >= step_x && yy >= step_y {
                let mut check_x = xx - step_x;
                let mut check_y = yy - step_y;

                loop {
                    if space[check_x][check_y] == Position::Asteroid {
                        detected.insert((check_x, check_y), calc_winkel((check_x as isize - xx as isize, check_y as isize - yy as isize)));
                        if step_x > check_x || step_y > check_y {
                            break;
                        }
                        check_x -= step_x;
                        check_y -= step_y;
                        loop {
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

            // left down
            if yy >= step_y {
                let mut check_x = xx + step_x;
                let mut check_y = yy - step_y;

                while check_x < max_x {
                    if space[check_x][check_y] == Position::Asteroid {
                        detected.insert((check_x, check_y), calc_winkel((check_x as isize - xx as isize, check_y as isize - yy as isize)));
                        if step_y > check_y {
                            break;
                        }
                        check_x += step_x;
                        check_y -= step_y;
                        while check_x < max_x {
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
            if *field == Position::Asteroid {
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

fn part2(po: &TodaysPuzzleOptions, res1: Option<OutputType1>) -> OutputType2 {
    let space = po.data.as_ref().unwrap().to_owned();

    let xx = (res1.unwrap().1).0;
    let yy = (res1.unwrap().1).1;

    let asteroids = get_hitable_asteroids(&space, xx, yy);
    let mut candidates: Vec<((usize, usize), f64)> = asteroids.into_iter().collect();
    candidates.sort_by(| (_, aa), (_, bb)| aa.partial_cmp(bb).unwrap());

    let result = candidates[199].0;
    (result.1 * 100 + result.0, result)
}


fn calc_winkel((xx, yy): (isize, isize)) -> f64 {
    if xx == 0 {
        return if yy > 0 {
            90f64
        } else if yy < 0 {
            270f64
        } else {
            0f64
        };
    } else if yy == 0 {
        return if xx > 0 {
            180f64
        } else {
            0f64
        };
    }

    let angle = f64::atan(xx as f64 / yy as f64);
    let angle = angle.to_degrees();

    let angle = match (xx.is_positive(), yy.is_positive()) {
        (true, true) => angle + 90f64,
        (true, false) => angle + 270f64,
        (false, true) => angle * -1f64,
        (false, false) => angle + 270f64,
    };

    angle
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
    fn test_angle_calculation() {
        let data = [
            ( (-2, 0), 0f64, 1f64 ),
            ( (-2, 2), 44f64, 46f64 ),
            ( (0, 2), 0f64, 90f64 ),
            ( (2, 2), 134f64, 136f64 ),
            ( (2, 0), 179f64, 181f64 ),
            ( (2, -2), 224f64, 226f64 ),
            ( (0, -2), 269f64, 271f64 ),
            ( (-2, -2), 314f64, 316f64 ),
            ( (-2, 0), 0f64, 1f64 ),
        ];

        for &(pos, lower, upper) in data.iter() {
            let angle = calc_winkel(pos);
            assert!(angle >= lower && angle <= upper, format!("{:?} -> {}", pos, angle));
        }
    }

    #[test]
    fn example_3() {
        test_case_helper("example3", (210, (13, 11)), (802, (2, 8)))
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
        let result_1 = part1(&puzzle_options);
        bb.iter(|| test::black_box(part2(&puzzle_options, Some(result_1))));
    }
}
