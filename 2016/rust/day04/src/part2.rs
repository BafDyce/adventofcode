use super::*;

use regex::Regex;
use std::collections::HashMap;

type OutputType = super::part1::OutputType;
//type OutputType = i64; // <-- IF part 2 needs a different output

pub fn solve(input: &InputType, config: &PuzzleConfig) -> OutputType {
    for room in input {
        println!("{} | {}", room.get_sid(), room.decrypt_name());
    }

    0
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
