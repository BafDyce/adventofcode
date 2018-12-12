use super::*;
use std::collections::HashMap;
use std::iter;

pub type OutputType = isize;

pub fn solve(input: &InputType, _config: &PuzzleConfig) -> OutputType {
    let mut state: Vec<char> = input.0.chars().collect();
    let mut leftest = 0;

    let mut rules: HashMap<[char; 5], char> = HashMap::new();
    for rule in &input.1 {
        rules.insert(rule.from, rule.to);
    }

    for __ in 0 .. 20 {
        let mut new_state = Vec::with_capacity(state.len()+4);

        let pots: Vec<_> =
            iter::repeat('.').take(4)
            .chain(state.clone().into_iter())
            .chain(iter::repeat('.').take(4))
            .collect();
        for pot in pots.windows(5) {
            match rules.get(pot) {
                Some(cc) => {
                    new_state.push(*cc);
                }
                _ => {
                    new_state.push('.')
                }
            }
        }
        // trimming
        leftest -= 2;

        let start = new_state.iter().position(|&cc| cc =='#').unwrap();
        if start > 0 {
            state = new_state[start .. ].to_vec();
            leftest += start as isize;
        } else {
            state = new_state;
        }

        let end = state.iter().rposition(|&cc| cc =='#').unwrap();
        if end < state.len() {
            for ii in (state.len() - 1) .. end {
                state.remove(ii);
            }
        }
    }

    let mut sum = 0isize;
    for (idx, cc) in state.into_iter().enumerate() {
        if cc == '#' {
            sum += leftest + idx as isize;
        }
    }

    sum
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
        assert_eq!(solve_example("example1"), 325);
    }
}
