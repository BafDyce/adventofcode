use super::*;

use std::collections::HashMap;
use std::iter;

type OutputType = super::part1::OutputType;
//type OutputType = i64; // <-- IF part 2 needs a different output

pub fn solve(input: &InputType, _config: &PuzzleConfig) -> OutputType {
    let mut state: Vec<char> = input.0.chars().collect();
    let mut leftest = 0;

    let mut rules: HashMap<[char; 5], char> = HashMap::new();
    for rule in &input.1 {
        rules.insert(rule.from, rule.to);
    }

    let mut last_diffs = VecDeque::with_capacity(5);
    last_diffs.push_back(0);
    last_diffs.push_back(0);
    last_diffs.push_back(0);
    last_diffs.push_back(0);
    last_diffs.push_back(0);

    let mut iterations = 0;
    let mut sum_total = 0;
    let mut diff_to_add = 0;
    let mut last_sum = 0;
    for counter in 1 ..= 50000000000 as usize {
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

        let mut sum = 0isize;
        for (idx, cc) in state.iter().enumerate() {
            if *cc == '#' {
                sum += leftest + idx as isize;
            }
        }

        last_diffs.pop_front().unwrap();
        let diff = sum - last_sum;
        if last_diffs.iter().any(|&item| item != diff) {
            last_diffs.push_back(diff);
            last_sum = sum;
        } else {
            iterations = counter as isize;
            sum_total = sum;
            diff_to_add = diff;
            break;
        }
    };

    (sum_total + (50000000000 - iterations) * diff_to_add) as isize
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
        // Calculated by the very same implementation which we test :P
        assert_eq!(solve_example("example1"), 999999999374);
    }
}
