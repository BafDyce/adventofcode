use super::*;

type OutputType = super::part1::OutputType;

pub fn solve(input: &InputType) -> OutputType {
    let data = input.to_owned();

    let mut best_length = 100000;

    for cc in "abcdefghijklmnopqrstuvwxyz".chars() {
        let mut work: String = data.chars().filter_map(|xx| {
            if xx == cc || xx == cc.to_uppercase().next().unwrap() {
                None
            } else {
                Some(xx)
            }
        })
        .map(|xx| xx)
        .collect::<Vec<char>>()
        .into_iter()
        .collect();

        let size = part1::solve(&work);
        if size < best_length {
            best_length = size;
            println!("{} -> {}", cc, best_length);
        }
    }

    best_length
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
        assert_eq!(solve_example("example1"), 4);
    }
}
