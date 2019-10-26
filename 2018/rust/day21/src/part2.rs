use super::*;
use std::collections::HashMap;

type OutputType = super::part1::OutputType;

pub fn solve(_input: &InputType, _config: &PuzzleConfig) -> OutputType {
    let mut dd_history = HashMap::new();
    let mut last_dd = 0;

    let mut dd = 0;
    'jmp_6: loop {
        let mut cc = dd | 65536;
        dd = 1397714;

        'jmp_8: loop {
            dd += cc & 255;
            dd &= 16777215;
            dd *= 65899;
            dd &= 16777215;

            if 256 > cc {
                // small modification compared to part 1: keep track of all calculated results.
                // as soon as we found the first number twice, we know that we just entered a loop
                // therefore, the last calculated number will be the one which terminates the
                // program but requires the most instructions to be compute
                if dd_history.get(&dd).is_some() {
                    return last_dd;
                } else {
                    dd_history.insert(dd, ());
                    last_dd = dd;
                }

                continue 'jmp_6;
            } else {
                cc /= 256;
            }
        }
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
