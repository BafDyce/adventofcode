use super::*;

pub type OutputType = i64;

pub fn solve(input: &InputType) -> OutputType {
    input.iter().fold(0i64, |acc: i64, xx| acc + xx)
}
