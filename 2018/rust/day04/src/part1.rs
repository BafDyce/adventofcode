use super::*;

pub type OutputType = usize;

pub fn solve(input: &InputType) -> OutputType {
    let max_sleeper = {
        let mut max_sleeptime: usize = 0;
        let mut max_sleeper: usize = 0;
        for (id, guard) in input.iter() {
            let sleeptime = guard.get_sleeptime_total();
            if sleeptime > max_sleeptime {
                max_sleeptime = sleeptime;
                max_sleeper = *id;
            }
        }

        max_sleeper
    };

    max_sleeper * input.get(&max_sleeper).unwrap().get_max_sleepminute()
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
        assert_eq!(solve_example("example1"), 240);
    }
}
