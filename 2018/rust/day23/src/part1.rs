use super::*;

use md5::{Md5, Digest};
// use sha1::{Sha1, Digest}; // just in case
use regex::Regex;
use std::collections::HashMap;

pub type OutputType = usize;

pub fn solve(bots: &InputType, _config: &PuzzleConfig) -> OutputType {
    let largest_range = bots.iter().max_by(|aa, bb| aa.get_range().cmp(&bb.get_range())).unwrap();
    println!("largest: {:?}", largest_range);
    bots.iter().filter(|bot| largest_range.in_range(bot)).count()
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
