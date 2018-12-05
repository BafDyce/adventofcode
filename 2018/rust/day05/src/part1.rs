use super::*;

pub type OutputType = usize;

pub fn dedup(data: &mut String) -> bool {
    let mut res: Option<String> = None;
    for (aa, bb) in data.chars().zip(data.chars().skip(1)) {
        if aa != bb && aa.to_lowercase().next().unwrap() == bb.to_lowercase().next().unwrap()  {
            res =  Some(format!("{}{}", aa, bb));
            break;
        }
    }

    if let Some(aabb) = res {
        if let Some(idx) = data.find(&aabb) {
            data.remove(idx);
            data.remove(idx);
        }

        true
    } else {
        false
    }
}

pub fn solve(input: &InputType) -> OutputType {
    let mut data = input.to_owned();
    while dedup(&mut data) {}

    data.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solve_example(name: &str) -> OutputType {
        let input = parse_input(name, false);
        solve(&input)
    }

    #[test]
    fn examples() {
        assert_eq!(solve_example("example1"), "dabCBAcaDA".len());
    }
}
