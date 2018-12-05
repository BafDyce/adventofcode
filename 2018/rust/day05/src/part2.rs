use super::*;

type OutputType = super::part1::OutputType;

pub fn solve(input: InputType) -> OutputType {
    let mut smallest = 100000;

    for cc in "abcdefghijklmnopqrstuvwxyz".chars() {
        let mut work: InputType = input.iter().filter_map(|xx| {
            if *xx == cc || *xx == cc.to_ascii_uppercase() {
                None
            } else {
                Some(*xx)
            }
        })
        .map(|xx| xx)
        .collect();

        let (size, _) = part1::solve(&work);
        smallest = std::cmp::min(smallest, size);
    }

    smallest
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solve_example(name: &str) -> OutputType {
        let input = parse_input(name, false);
        let (_, input) = part1::solve(&input);
        solve(input)
    }

    #[test]
    fn examples() {
        assert_eq!(solve_example("example1"), 4);
    }
}
