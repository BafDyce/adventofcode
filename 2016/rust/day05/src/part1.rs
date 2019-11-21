use super::*;

use md5::{Md5, Digest};

pub type OutputType = String;

pub fn solve(input: &InputType, config: &PuzzleConfig) -> OutputType {
    let mut counter = 0;
    let mut hasher = Md5::new();
    let mut result = String::new();

    while result.len() < 8 {
        hasher.input(&format!("{}{}", input, counter));
        let hash = hasher.result_reset();

        if &hash[..2] == [0; 2] && hash[2] <= 16 {
            let ch = format!("{:x}", hash[2] & 0xf);
            result.push_str(&ch);
        }

        counter += 1;
    }

    result
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
