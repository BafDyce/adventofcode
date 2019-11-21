use super::*;

use regex::Regex;

pub type OutputType = usize;

pub fn solve(input: &InputType, config: &PuzzleConfig) -> OutputType {
    input.iter().filter(|triangle| triangle.is_valid()).count()
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
