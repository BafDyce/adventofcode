use super::*;

use md5::{Md5, Digest};
// use sha1::{Sha1, Digest}; // just in case
use regex::Regex;
use std::collections::HashMap;

pub type OutputType = String;

pub fn solve(input: InputType, config: &PuzzleConfig) -> OutputType {
    let mut elves = [0usize, 1];
    let mut recipes = [3, 7].to_vec();

    while recipes.len() < 2 + input + 10 {
        // enhance
        let next: usize = elves.iter().map(|&idx| recipes[idx]).sum();
        let nexts: Vec<usize> = format!("{}", next).chars().map(|digit| format!("{}", digit).parse::<usize>().unwrap()).collect();
        recipes.extend(nexts);

        // step forward
        for ii in 0 .. elves.len() {
            elves[ii] = (elves[ii] + 1 + recipes[elves[ii]]) % recipes.len();
        }
    }

    format!("{}{}{}{}{}{}{}{}{}{}",
        recipes[input + 0],
        recipes[input + 1],
        recipes[input + 2],
        recipes[input + 3],
        recipes[input + 4],
        recipes[input + 5],
        recipes[input + 6],
        recipes[input + 7],
        recipes[input + 8],
        recipes[input + 9],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solve_example(name: &str) -> OutputType {
        let (input, config) = parse_input(name, false);
        solve(input, &config)
    }

    #[test]
    fn example_1() {
        assert_eq!(solve_example("example1"), String::from("5158916779"));
    }

    #[test]
    fn example_2() {
        assert_eq!(solve_example("example2"), String::from("0124515891"));
    }

    #[test]
    fn example_3() {
        assert_eq!(solve_example("example3"), String::from("9251071085"));
    }

    #[test]
    fn example_4() {
        assert_eq!(solve_example("example4"), String::from("5941429882"));
    }
}
