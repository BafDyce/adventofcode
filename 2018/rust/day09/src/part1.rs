use super::*;

pub type OutputType = usize;

pub fn solve(input: &InputType, _config: &PuzzleConfig) -> OutputType {
    let mut players = Vec::with_capacity(input.0);
    for __ in 0.. input.0 {
        players.push(0);
    }
    let points = input.1;

    let mut circle = Circle::new();
    let mut current_player = 0;
    for ii in 1 ..= points {
        players[current_player] += circle.add_marble(ii);
        current_player = (current_player + 1) % players.len();
    }

    players.into_iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solve_example(name: &str) -> OutputType {
        let (input, config) = parse_input(name, false);
        solve(&input, &config)
    }

    #[test]
    fn example_1() {
        assert_eq!(solve_example("example1"), 32);
    }

    #[test]
    fn example_2() {
        assert_eq!(solve_example("example2"), 8317);
    }

    #[test]
    fn example_3() {
        assert_eq!(solve_example("example3"), 146373);
    }

    #[test]
    fn example_4() {
        assert_eq!(solve_example("example4"), 2764);
    }

    #[test]
    fn example_5() {
        assert_eq!(solve_example("example5"), 54718);
    }

    #[test]
    fn example_6() {
        assert_eq!(solve_example("example6"), 37305);
    }
}
