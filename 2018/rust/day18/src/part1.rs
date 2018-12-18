use super::*;

pub type OutputType = usize;

pub fn solve(input: &InputType, _config: &PuzzleConfig) -> OutputType {

    let mut area = input.to_owned();
    println!("START:");
    print(&area);
    for __ in 0 .. 10 {
        area = update(area);
    }

    calc_score(&area)
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
        assert_eq!(solve_example("example1"), 1147);
    }
}
