use super::*;

use md5::{Md5, Digest};
// use sha1::{Sha1, Digest}; // just in case
use regex::Regex;

pub type OutputType = usize;

pub fn solve(input: &InputType) -> OutputType {
    input.iter().fold(1, |acc, xx| acc * xx)
}
