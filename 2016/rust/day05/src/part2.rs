use super::*;

use md5::{Md5, Digest};

type OutputType = super::part1::OutputType;

pub fn solve(input: &InputType, config: &PuzzleConfig) -> OutputType {
    let mut counter = 0;
    let mut hasher = Md5::new();
    let mut result = [' '; 8];
    let mut hits = 0;

    while hits < 8 {
        hasher.input(&format!("{}{}", input, counter));
        let hash = hasher.result_reset();

        if &hash[..2] == [0; 2] && hash[2] <= 7 {
            let pos = hash[2] as usize;
            if result[pos] == ' ' {
                let ch = format!("{:x}", hash[3] & 0xf0);
                result[pos] = ch.chars().nth(0).unwrap();
                hits += 1;
            }

        }

        counter += 1;
    }

    result.into_iter().collect()
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
