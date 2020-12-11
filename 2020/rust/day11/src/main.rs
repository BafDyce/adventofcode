/*
      -------Part 1--------   -------Part 2--------
Day       Time  Rank  Score       Time  Rank  Score
 11   00:24:11  1619      0   01:12:48  3618      0


test bench::bench_parsing ... bench:      21,155 ns/iter (+/- 2,816)
test bench::bench_part1   ... bench:   7,889,129 ns/iter (+/- 281,224)
test bench::bench_part2   ... bench:  21,049,814 ns/iter (+/- 796,541)

*/

// allow bench feature when using unstable flag
// use: $ rustup run nightly cargo bench --features unstable
#![cfg_attr(feature = "unstable", feature(test))]

use aoc_import_magic::{import_magic, PuzzleOptions};
use std::{
    collections::HashMap,
    io,
};

const DAY: u32 = 11;
type InputType = Vec<Vec<Tile>>;
type OutputType = usize;
type TodaysPuzzleOptions = PuzzleOptions<InputType>;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Floor,
    Empty,
    Occupied,
}

impl From<char> for Tile {
    fn from(from: char) -> Tile {
        match from {
            '.' => Tile::Floor,
            'L' => Tile::Empty,
            '#' => Tile::Occupied,
            _ => unreachable!(),
        }
    }
}

fn parse_input(input: Vec<String>, _config: &HashMap<String, String>, _verbose: bool) -> InputType {
    input
        .into_iter()
        .map(|line| {
            line.chars().map(|cc| Tile::from(cc)).collect()
        })
        .collect()
}

fn count_people(floor: &Vec<Vec<Tile>>) -> usize {
    floor.iter().map(|row| row.iter().filter(|&&tile| tile == Tile::Occupied)).flatten().count()
}

fn do_tick(floor: &mut Vec<Vec<Tile>>)  {
    let mut new_floor = Vec::new();
    let width = floor[0].len();

    for row in 0 .. floor.len() {
        new_floor.push(Vec::new());
        for col in 0 .. width {
            let mut count_neighbors = 0;
            if row > 0 {
                if col > 0 {
                    if floor[row-1][col-1] == Tile::Occupied {
                        count_neighbors += 1;
                    }
                }
                if floor[row-1][col] == Tile::Occupied {
                    count_neighbors += 1;
                }
                if col < width - 1 {
                    if floor[row-1][col+1] == Tile::Occupied {
                        count_neighbors += 1;
                    }
                }
            }

            if col > 0 {
                if floor[row][col-1] == Tile::Occupied {
                    count_neighbors += 1;
                }
            }
            if col < width - 1 {
                if floor[row][col+1] == Tile::Occupied {
                    count_neighbors += 1;
                }
            }

            if row < floor.len() - 1 {
                if col > 0 {
                    if floor[row+1][col-1] == Tile::Occupied {
                        count_neighbors += 1;
                    }
                }
                if floor[row+1][col] == Tile::Occupied {
                    count_neighbors += 1;
                }
                if col < width - 1 {
                    if floor[row+1][col+1] == Tile::Occupied {
                        count_neighbors += 1;
                    }
                }
            }

            let current = floor[row][col];
            let seat = if count_neighbors == 0 && current == Tile::Empty {
                Tile::Occupied
            } else if count_neighbors >= 4 && current == Tile::Occupied {
                Tile::Empty
            } else {
                current
            };

            new_floor[row].push(seat);
        }
    }

    *floor = new_floor;
}

fn do_tick_2(floor: &mut Vec<Vec<Tile>>)  {
    let mut new_floor = Vec::new();
    let width = floor[0].len();

    for row in 0 .. floor.len() {
        new_floor.push(Vec::new());
        for col in 0 .. width {
            let mut count_neighbors = 0;

            // straight down
            for row in row+1 .. floor.len() {
                if floor[row][col] == Tile::Occupied {
                    count_neighbors += 1;
                    break;
                } else if floor[row][col] == Tile::Empty {
                    break;
                }
            }

            // straight up
            if row > 0 {
                for row in (0 ..= row-1).rev() {
                    if floor[row][col] == Tile::Occupied {
                        count_neighbors += 1;
                        break;
                    } else if floor[row][col] == Tile::Empty {
                        break;
                    }
                }
            }


            // straight right
            for col in col+1 .. width {
                if floor[row][col] == Tile::Occupied {
                    count_neighbors += 1;
                    break;
                } else if floor[row][col] == Tile::Empty {
                    break;
                }
            }

            // straight left
            if col > 0 {
                for col in (0 ..= col-1).rev() {
                    if floor[row][col] == Tile::Occupied {
                        count_neighbors += 1;
                        break;
                    } else if floor[row][col] == Tile::Empty {
                        break;
                    }
                }
            }

            // up left
            if row > 0 && col > 0 {
                let mut row = row - 1;
                let mut col = col -1;
                loop {
                    if floor[row][col] == Tile::Occupied {
                        count_neighbors += 1;
                        break;
                    } else if floor[row][col] == Tile::Empty {
                        break;
                    }

                    if row == 0 || col == 0 {
                        break;
                    }
                    row -= 1;
                    col -= 1;
                }
            }

            // up right
            if row > 0 && col < width - 1 {
                let mut row = row - 1;
                let mut col = col + 1;
                while col < width {
                    if floor[row][col] == Tile::Occupied {
                        count_neighbors += 1;
                        break;
                    } else if floor[row][col] == Tile::Empty {
                        break;
                    }

                    if row == 0 {
                        break;
                    }
                    row -= 1;
                    col += 1;
                }
            }

            // down left
            if row < floor.len() -1 && col > 0 {
                let mut row = row + 1;
                let mut col = col - 1;
                while row < floor.len() {
                    if floor[row][col] == Tile::Occupied {
                        count_neighbors += 1;
                        break;
                    } else if floor[row][col] == Tile::Empty {
                        break;
                    }

                    if col == 0 {
                        break;
                    }
                    row += 1;
                    col -= 1;
                }
            }

            // down right
            if row < floor.len() -1 && col < width -1 {
                let mut row = row + 1;
                let mut col = col + 1;
                loop {
                    if floor[row][col] == Tile::Occupied {
                        count_neighbors += 1;
                        break;
                    } else if floor[row][col] == Tile::Empty {
                        break;
                    }


                    row += 1;
                    col += 1;
                    if row >= floor.len() || col >= width {
                        break;
                    }
                }
            }

            let current = floor[row][col];
            let seat = if count_neighbors == 0 && current == Tile::Empty {
                Tile::Occupied
            } else if count_neighbors >= 5 && current == Tile::Occupied {
                Tile::Empty
            } else {
                current
            };

            new_floor[row].push(seat);
        }
    }

    *floor = new_floor;
}

fn part1(po: &TodaysPuzzleOptions) -> OutputType {
    let mut floor = po.get_data().clone();

    let mut last_count = 1;
    loop {
        let count = count_people(&floor);
        if last_count == count {
            return count;
        }

        do_tick(&mut floor);
        last_count = count;
    }
}

fn part2(po: &TodaysPuzzleOptions, _res1: Option<OutputType>) -> OutputType {
    let mut floor = po.get_data().clone();

    let mut last_count = 1;
    loop {
        let count = count_people(&floor);
        if last_count == count {
            return count;
        }

        do_tick_2(&mut floor);
        last_count = count;
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_import_magic::{import_magic_with_params, PuzzleOptions};

    pub(super) fn import_helper(inputname: &str) -> PuzzleOptions<InputType> {
        let params = ["appname", "--input", inputname];
        import_magic_with_params(DAY, parse_input, &params).unwrap()
    }

    fn test_case_helper(inputname: &str, sol1: OutputType, sol2: OutputType) {
        let po = import_helper(inputname);
        let res1 = part1(&po);
        assert_eq!(sol1, res1, "part1");
        let res2 = part2(&po, Some(res1));
        assert_eq!(sol2, res2, "part2");
    }

    #[test]
    fn example_1() {
        test_case_helper("example1", 37, 26)
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
