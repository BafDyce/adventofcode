use super::*;
use std::collections::HashMap;

type OutputType = super::part1::OutputType;

pub fn solve(input: &InputType, _config: &PuzzleConfig) -> OutputType {
    let mut area = input.to_owned();

    let mut lookup = HashMap::new();
    lookup.insert(input.to_owned(), 0 );

    let mut time = 0;
    loop {
        time += 1;
        area = update(area);

        match lookup.get(&area) {
            Some(minute) => {
                let cycle_time = time - minute;
                let remaining_minutes = (1000000000 - minute) % cycle_time;
                for __ in 0 .. remaining_minutes {
                    area = update(area);
                }
                return calc_score(&area);
            }
            _ => {}
        }

        lookup.insert(area.to_owned(), time);
    }
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
