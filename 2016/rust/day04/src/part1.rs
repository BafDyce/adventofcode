use super::*;

use md5::{Md5, Digest};
// use sha1::{Sha1, Digest}; // just in case
use regex::Regex;
use std::collections::HashMap;

pub type OutputType = usize;

pub fn solve(input: &InputType, config: &PuzzleConfig) -> OutputType {
    input.into_iter().filter(|room| room.is_real()).map(|room| room.get_sid()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solve_example(name: &str) -> OutputType {
        let (input, config) = parse_input(name, false);
        solve(&input, &config)
    }

    #[test]
    fn examples() {
        assert_eq!(solve_example("example1"), OutputType::default());
    }
}
