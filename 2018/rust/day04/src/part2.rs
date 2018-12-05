use super::*;

type OutputType = super::part1::OutputType;

pub fn solve(input: &InputType) -> OutputType {
    let (max_sleepminute, max_sleeper) = {
        let mut max_sleepminute = 0;
        let mut max_sleeper = 0;
        for (id, guard) in input.iter() {
            let sleepminute = guard.get_max_sleepminute();
            if sleepminute > max_sleepminute {
                max_sleepminute = sleepminute;
                max_sleeper = *id;
            }
        }

        (max_sleepminute, max_sleeper)
    };

    max_sleepminute * max_sleeper
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solve_example(name: &str) -> OutputType {
        let input = parse_input(name, false);
        let guards = precompute(input);
        solve(&guards)
    }

    #[test]
    fn examples() {
        assert_eq!(solve_example("example1"), 4455);
    }
}
