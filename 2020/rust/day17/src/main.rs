/*
      -------Part 1--------   -------Part 2--------
Day       Time  Rank  Score       Time  Rank  Score
 17   00:41:22  1745      0   00:50:53  1731      0

test bench::bench_parsing ... bench:       5,770 ns/iter (+/- 26)
test bench::bench_part1   ... bench:  12,157,495 ns/iter (+/- 153,461)
test bench::bench_part2   ... bench: 476,101,918 ns/iter (+/- 23,012,939)

*/

// TODO: Refactor to apply DRY using traits

// allow bench feature when using unstable flag
// use: $ rustup run nightly cargo bench --features unstable
#![cfg_attr(feature = "unstable", feature(test))]

use aoc_import_magic::{import_magic, PuzzleOptions};
use std::{
    collections::HashMap,
    fmt,
    io,
};

const DAY: u32 = 17;
type InputType = HashMap<Coord, Cube>;
type HyperInputType = HashMap<HyperCoord, Cube>;
type OutputType1 = usize;
type OutputType2 = OutputType1;
type TodaysPuzzleOptions = PuzzleOptions<InputType>;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Cube {
    Active,
    Inactive
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coord {
    layer: isize,
    row: isize,
    col: isize,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct HyperCoord {
    layer: isize,
    row: isize,
    col: isize,
    hyp: isize,
}

impl From<char> for Cube {
    fn from(from: char) -> Cube {
        match from {
            '.' => Cube::Inactive,
            '#' => Cube::Active,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Cube {
    fn fmt(&self, ff: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Cube::Active => '#',
            Cube::Inactive => '.',
        };
        write!(ff, "{}", symbol)
    }
}

impl Coord {
    fn neighbors(&self) -> Vec<Coord> {
        let mut neighbors = Vec::new();
        for layer_offset in -1 ..= 1 {
            for row_offset in -1 ..= 1 {
                for col_offset in -1 ..= 1 {
                    if layer_offset == 0 && row_offset == 0 && col_offset == 0 {
                        continue;
                    }

                    neighbors.push(Coord {
                        layer: self.layer + layer_offset,
                        row: self.row + row_offset,
                        col: self.col + col_offset,
                    })
                }
            }
        }

        neighbors
    }
}

impl HyperCoord {
    fn neighbors(&self) -> Vec<HyperCoord> {
        let mut neighbors = Vec::new();
        for layer_offset in -1 ..= 1 {
            for row_offset in -1 ..= 1 {
                for col_offset in -1 ..= 1 {
                    for hyp_offset in -1 ..= 1 {
                        if layer_offset == 0 && row_offset == 0 && col_offset == 0 && hyp_offset == 0 {
                            continue;
                        }

                        neighbors.push(HyperCoord {
                            layer: self.layer + layer_offset,
                            row: self.row + row_offset,
                            col: self.col + col_offset,
                            hyp: self.hyp + hyp_offset,
                        })
                    }

                }
            }
        }

        neighbors
    }
}

impl From<&Coord> for HyperCoord {
    fn from(coord: &Coord) -> HyperCoord {
        Self {
            layer: coord.layer,
            row: coord.row,
            col: coord.col,
            hyp: 0,
        }
    }
}

fn parse_input(input: Vec<String>, _config: &HashMap<String, String>, _verbose: bool) -> InputType {
    input
        .into_iter()
        .map(|line| {
            line.chars().map(Cube::from).enumerate().collect::<Vec<_>>()
        })
        .enumerate()
        .map(|(row, cols)| {
            cols.into_iter().map(move |(col, cube)| {
                (
                    Coord { layer: 0, row: row as isize, col: col as isize},
                    cube,
                )
            })
        })
        .flatten()
        .collect()
}

#[allow(dead_code)]
fn print_dimensions(pocket: &InputType) {
    let mut layer_min = isize::MAX;
    let mut layer_max = isize::MIN;

    let mut row_min = isize::MAX;
    let mut row_max = isize::MIN;

    let mut col_min = isize::MAX;
    let mut col_max = isize::MIN;

    for coord in pocket.keys() {
        layer_max = isize::max(layer_max, coord.layer);
        layer_min = isize::min(layer_min, coord.layer);

        row_max = isize::max(row_max, coord.row);
        row_min = isize::min(row_min, coord.row);

        col_max = isize::max(col_max, coord.col);
        col_min = isize::min(col_min, coord.col);
    }

    for layer in layer_min ..= layer_max {
        println!("z = {}", layer);
        for row in row_min ..= row_max {
            for col in col_min ..= col_max {
                print!("{}", pocket.get(&Coord{ layer, row, col}).unwrap())
            }
            println!()
        }
        println!()
    }
}

fn do_cycle(pocket: &mut InputType) {
    // TODO: refactor calculation of boundaries into separate function
    let mut layer_min = isize::MAX;
    let mut layer_max = isize::MIN;

    let mut row_min = isize::MAX;
    let mut row_max = isize::MIN;

    let mut col_min = isize::MAX;
    let mut col_max = isize::MIN;

    for coord in pocket.keys() {
        layer_max = isize::max(layer_max, coord.layer);
        layer_min = isize::min(layer_min, coord.layer);

        row_max = isize::max(row_max, coord.row);
        row_min = isize::min(row_min, coord.row);

        col_max = isize::max(col_max, coord.col);
        col_min = isize::min(col_min, coord.col);
    }

    layer_min -= 1;
    layer_max += 1;

    row_min -= 1;
    row_max += 1;

    col_min -= 1;
    col_max += 1;

    let mut new_pockets = HashMap::new();
    for layer in layer_min ..= layer_max {
        for row in row_min ..= row_max {
            for col in col_min ..= col_max {
                let position = Coord { layer, row, col };
                let active_neighbors = position.neighbors()
                    .into_iter()
                    .filter_map(|coord| pocket.get(&coord))
                    .filter(|&&cube| cube == Cube::Active)
                    .count();
                let new_state = match (pocket.get(&position), active_neighbors) {
                    (Some(Cube::Active), 2) => Cube::Active,
                    (_, 3) => Cube::Active,
                    _ => Cube::Inactive
                };

                new_pockets.insert(position, new_state);
            }
        }
    }

    *pocket = new_pockets;
}

fn do_hyper_cycle(pocket: &mut HyperInputType) {
    // TODO: refactor calculation of boundaries into separate function
    let mut layer_min = isize::MAX;
    let mut layer_max = isize::MIN;

    let mut row_min = isize::MAX;
    let mut row_max = isize::MIN;

    let mut col_min = isize::MAX;
    let mut col_max = isize::MIN;

    let mut hyp_min = isize::MAX;
    let mut hyp_max = isize::MIN;

    for coord in pocket.keys() {
        layer_max = isize::max(layer_max, coord.layer);
        layer_min = isize::min(layer_min, coord.layer);

        row_max = isize::max(row_max, coord.row);
        row_min = isize::min(row_min, coord.row);

        col_max = isize::max(col_max, coord.col);
        col_min = isize::min(col_min, coord.col);

        hyp_max = isize::max(hyp_max, coord.hyp);
        hyp_min = isize::min(hyp_min, coord.hyp);
    }

    layer_min -= 1;
    layer_max += 1;

    row_min -= 1;
    row_max += 1;

    col_min -= 1;
    col_max += 1;

    hyp_min -= 1;
    hyp_max += 1;

    let mut new_pockets = HashMap::new();
    for layer in layer_min ..= layer_max {
        for row in row_min ..= row_max {
            for col in col_min ..= col_max {
                for hyp in hyp_min ..= hyp_max {
                    let position = HyperCoord { layer, row, col, hyp };
                    let active_neighbors = position.neighbors()
                        .into_iter()
                        .filter_map(|coord| pocket.get(&coord))
                        .filter(|&&cube| cube == Cube::Active)
                        .count();
                    let new_state = match (pocket.get(&position), active_neighbors) {
                        (Some(Cube::Active), 2) => Cube::Active,
                        (_, 3) => Cube::Active,
                        _ => Cube::Inactive
                    };

                    new_pockets.insert(position, new_state);
                }
            }
        }
    }

    *pocket = new_pockets;
}

fn part1(po: &TodaysPuzzleOptions) -> OutputType1 {
    let mut dimension = po.get_data().to_owned();

    //print_dimensions(&dimension);
    for _cycle in 1 ..= 6 {
        do_cycle(&mut dimension);
        //println!("After {} cycle(s): ", cycle);
        //print_dimensions(&dimension);
    }

    dimension.values().filter(|&&cube| cube == Cube::Active).count()
}

fn part2(po: &TodaysPuzzleOptions) -> OutputType2 {
    let mut dimension = HashMap::new();
    for (coord, cube) in po.get_data() {
        dimension.insert(HyperCoord::from(coord), *cube);
    }

    for _cycle in 1 ..= 6 {
        do_hyper_cycle(&mut dimension);
        //println!("After {} cycle(s): ", cycle);
        //print_dimensions(&dimension);
    }

    dimension.values().filter(|&&cube| cube == Cube::Active).count()
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

    fn test_case_helper(inputname: &str, sol1: Option<OutputType1>, sol2: Option<OutputType2>) {
        let po = import_helper(inputname);

        if let Some(sol1) = sol1 {
            let res1 = part1(&po);
            assert_eq!(sol1, res1, "part1");
        }

        if let Some(sol2) = sol2 {
            let res2 = part2(&po);
            assert_eq!(sol2, res2, "part2");
        }
    }

    #[test]
    fn example_1() {
        test_case_helper("example1", Some(112), Some(848))
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
