use super::*;

pub type OutputType = usize;

pub fn dedup(data: &mut InputType) -> bool {
    let mut res: InputType = InputType::new();

    for cc in data.iter() {
        if res.is_empty() {
            res.push_back(*cc);
            continue;
        }

        let prev = *res.back().unwrap();
        if *cc != prev && cc.to_ascii_lowercase() == prev.to_ascii_lowercase() {
            res.pop_back();
        } else {
            res.push_back(*cc);
        }
    }

    let retval = !(data.len() == res.len());
    *data = res;
    retval
}

pub fn solve(input: &InputType) -> (OutputType, InputType) {
    let mut data = input.to_owned();
    while dedup(&mut data) {}

    (data.len(), data)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solve_example(name: &str) -> OutputType {
        let input = parse_input(name, false);
        solve(&input).0
    }

    #[test]
    fn examples() {
        assert_eq!(solve_example("example1"), "dabCBAcaDA".len());
    }
}
