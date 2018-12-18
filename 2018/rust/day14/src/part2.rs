use super::*;

use regex::Regex;
use std::collections::HashMap;

type OutputType = usize;

// TODO: Generalize (no hardcoding of slice lengths..); see NOTE's below
pub fn solve(input: InputType, config: &PuzzleConfig) -> OutputType {
    let mut elves = [0usize, 1];
    let mut recipes = [3, 7].to_vec();

    // NOTE: hardcoded to a fixed size of 6 elements, so that we can easily compare
    let mut input_digits = format!("{:06}", input)
        .chars()
        .map(|digit| format!("{}", digit)
            .parse::<usize>()
            .unwrap()
        ).collect::<Vec<usize>>()
        .into_iter();
    let goal: [usize; 6] = [
        input_digits.next().unwrap(),
        input_digits.next().unwrap(),
        input_digits.next().unwrap(),
        input_digits.next().unwrap(),
        input_digits.next().unwrap(),
        input_digits.next().unwrap(),
    ];
    //let goal: [usize; 6] = [0, 4, 7, 8, 0, 1];

    // only check the new numbers (+ a few to the left)
    let mut last_idx_to_check = 0;
    'result: loop {
        // enhance
        let next: usize = elves.iter().map(|&idx| recipes[idx]).sum();
        let nexts: Vec<usize> = format!("{}", next)
            .chars()
            .map(|digit| format!("{}", digit)
                .parse::<usize>()
                .unwrap()
            ).collect();
        recipes.extend(nexts);

        // step forward
        for ii in 0 .. elves.len() {
            elves[ii] = (elves[ii] + 1 + recipes[elves[ii]]) % recipes.len();
        }

        // NOTE: windows size is here hardcoded too!
        for (ii, check) in recipes[last_idx_to_check .. recipes.len()].windows(6).enumerate() {
            if check == goal {
                break 'result last_idx_to_check + ii;
            }
        }

        // length check to avoid negative (= overflown) indices
        last_idx_to_check = if recipes.len() < 10 {
            0
        } else {
            // NOTE: -6 also depends on the input length
            recipes.len() - 1 - 6
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solve_example(name: &str) -> OutputType {
        let (input, config) = parse_input(name, false);
        solve(input, &config)
    }

    #[test]
    fn examples() {
        assert_eq!(
            "No tests implemented",
            "Because we first need to generalize the implementation"
        );
    }
}
