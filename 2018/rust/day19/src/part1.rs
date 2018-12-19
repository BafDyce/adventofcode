use super::*;
use regex::Regex;
use std::collections::HashMap;

use chronassembly::*;

pub type OutputType = usize;

pub fn solve(input: &InputType, config: &PuzzleConfig) -> OutputType {
    let mut program = input.program.to_owned();

    let mut regs = RegisterSet::new();
    if let Some(front) = program.pop_front() {
        front.execute(&mut regs);
    }

    let mut ip = 0;
    while ip < program.len() {
        ip = program[ip].execute(&mut regs);
        //println!("{:?}", regs.rs);
    }

    regs.rs[0]
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
