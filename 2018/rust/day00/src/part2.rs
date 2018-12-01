use super::*;

use regex::Regex;

type OutputType = super::part1::OutputType;
//type OutputType = i64; // <-- IF part 2 needs a different output

pub fn solve(input: &InputType) -> OutputType {
    input.iter().fold(0, |acc, xx| acc + xx)
}
