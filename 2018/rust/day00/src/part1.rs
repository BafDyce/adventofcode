use super::*;

use md5::{Md5, Digest};
// use sha1::{Sha1, Digest}; // just in case
use regex::Regex;

pub type OutputType = usize;

pub fn solve(input: &InputType) -> OutputType {
    input.iter().fold(1, |acc, xx| acc * xx)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solve_example(name: &str) -> OutputType {
        let input = parse_input(name, false);
        solve(&input)
    }

    #[test]
    fn examples() {
        assert_eq!(solve_example("example1"), 42);
    }
}
