
/*
      -------Part 1--------   -------Part 2--------
Day       Time  Rank  Score       Time  Rank  Score
  6   00:17:33   901      0   00:39:39  1181      0

BENCHMARKS
test bench::bench_parsing ... bench:     371,362 ns/iter (+/- 31,447)
test bench::bench_part1   ... bench:   5,426,179 ns/iter (+/- 303,857)
test bench::bench_part2   ... bench:     548,822 ns/iter (+/- 40,359)
*/

// allow bench feature when using unstable flag
// use: $ cargo +nightly bench --features unstable
#![cfg_attr(feature = "unstable", feature(test))]

use aoc_import_magic::{import_magic, PuzzleOptions};
use std::{
    collections::HashMap,
    io,
};

const DAY: i32 = 6;
type InputType = HashMap::<String, Vec<String>>;
type OutputType1 = usize;
type OutputType2 = OutputType1;
type TodaysPuzzleOptions = PuzzleOptions<InputType>;

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
    create_orbit_map(
        input
            .into_iter()
            .map(|line| {
                let data: Vec<&str> = line.split(")").collect();
                (data[0].to_owned(), data[1].to_owned())
            })
            .collect()
    )
}

fn part1(po: &TodaysPuzzleOptions) -> OutputType1 {
    let orbits = po.data.as_ref().unwrap();
    orbits.keys().map(|kk| count_orbits(orbits, kk)).sum()
}

fn part2(po: &TodaysPuzzleOptions, _res1: Option<OutputType1>) -> OutputType2 {
    let orbits = po.data.as_ref().unwrap();

    let santa_orbits_parents: HashMap::<String, usize> = get_parents(orbits, "SAN").into_iter().collect();
    let my_parents: HashMap::<String, usize> = get_parents(orbits, "YOU").into_iter().collect();

    if santa_orbits_parents.len() <= 1 || my_parents.len() <= 1 {
        println!("Someone is missing!");
        return 0;
    }

    my_parents.into_iter().map(|(kk, vv)| {
        match santa_orbits_parents.get(&kk) {
            Some(hit) => hit + vv,
            _ => std::usize::MAX,
        }
    })
    .min().unwrap() - 2
}

fn create_orbit_map(data: Vec<(String, String)>) -> HashMap::<String, Vec<String>> {
    // key = name of object
    // values = name of direct parents
    let mut orbits = HashMap::<String, Vec<String>>::new();

    for (bb, aa) in data.into_iter() {
        let entry = orbits.entry(aa).or_insert(Vec::new());
        if !entry.contains(&bb) {
            entry.push(bb);
        }
    }

    orbits
}

fn count_orbits(orbits: &HashMap<String, Vec<String>>, aa: &str) -> usize {
    match orbits.get(aa) {
        Some(bb) => bb.iter().map(|bb| count_orbits(orbits, bb)).sum::<usize>() + 1,
        _ => 0
    }
}

fn get_parents(orbits: &HashMap::<String, Vec<String>>, child: &str) -> Vec<(String, usize)> {
    match orbits.get(child) {
        Some(parents) => {
            parents
                .iter()
                .map(|pp| {
                    let mut tmp = vec![(child.to_owned(), 0)];
                    tmp.extend(
                        get_parents(orbits, pp)
                            .into_iter()
                            .map(|(pp, distance)| (pp, distance + 1))
                    );
                    tmp
                })
                .flatten()
                .collect()
        }
        None => vec![(child.to_owned(), 0)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_import_magic::{import_magic_with_params, PuzzleOptions};

    pub(in super) fn import_helper(inputname: &str) -> PuzzleOptions<InputType> {
        let params = [
            "appname",
            "--input",
            inputname,
        ];
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
        test_case_helper("example1", 42, 0)
    }

    #[test]
    fn example_2() {
        test_case_helper("example2", 54, 4)
    }
}

#[cfg(all(feature = "unstable", test))]
mod bench {
    extern crate test;

    use super::*;
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };
    use test::Bencher;

    fn helper_read_file(fname: &str) -> Vec<String> {
        BufReader::new(File::open(fname).unwrap()).lines().map(|line| line.unwrap()).collect()
    }

    #[bench]
    fn bench_parsing(bb: &mut Bencher) {
        let input = helper_read_file(&format!("../../_inputs/day{:02}/real1.input", DAY));
        bb.iter(|| parse_input(input.to_owned(), &HashMap::new(), false));
    }

    #[bench]
    fn bench_part1(bb: &mut Bencher) {
        let puzzle_options = tests::import_helper("real1");
        bb.iter(|| part1(&puzzle_options));
    }

    #[bench]
    fn bench_part2(bb: &mut Bencher) {
        let puzzle_options = tests::import_helper("real1");
        bb.iter(|| part2(&puzzle_options, None));
    }
}
