use super::*;

pub type OutputType = usize;

pub fn solve(input: &InputType, _config: &PuzzleConfig) -> OutputType {
    input.metasum()
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
        assert_eq!(solve_example("example1"), 138);
    }
}
