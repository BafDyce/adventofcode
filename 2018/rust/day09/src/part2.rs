use super::*;

type OutputType = super::part1::OutputType;

pub fn solve(input: &InputType, _config: &PuzzleConfig) -> OutputType {
    let mut players = Vec::with_capacity(input.0);
    for __ in 0.. input.0 {
        players.push(0);
    }
    let points = input.1;

    let mut circle = Circle::new();
    let mut current_player = 0;
    for ii in 1 ..= points * 100 {
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
    fn examples() {
        assert_eq!(solve_example("example1"), 22563);
    }
}
