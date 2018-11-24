use super::*;

use regex::Regex;

type OutputType = usize;

pub fn solve(input: &InputType) -> OutputType {
    input.iter().fold(0, |acc, xx| acc + xx)
}
