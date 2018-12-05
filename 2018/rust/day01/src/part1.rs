use super::*;

pub type OutputType = i64;

pub fn solve(input: &InputType) -> OutputType {
    input.iter().fold(0i64, |acc: i64, xx| acc + xx)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solve_example(name: &str) -> OutputType {
        let input = parse_input(name, false);
        solve(&input)
    }

    #[test]
    fn examples() {
        assert_eq!(solve_example("example1"), 3);
        assert_eq!(solve_example("example2"), 3);
        assert_eq!(solve_example("example3"), 0);
        assert_eq!(solve_example("example4"), -6);
    }
}
