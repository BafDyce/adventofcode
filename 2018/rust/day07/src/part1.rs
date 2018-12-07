use super::*;

use std::collections::HashMap;

pub type OutputType = String;

pub fn solve(deps: &HashMap<char, Vec<char>>, visited: &HashMap<char, bool>) -> OutputType {
    let mut visited = visited.clone();
    let mut result = Vec::new();


    let mut keys = deps.keys().collect::<Vec<_>>();
    keys.sort();

    while visited.values().any(|xx| !xx) {
        for kk in keys.iter() {
            if *visited.get(kk).unwrap() {
                continue;
            }

            let node_deps = deps.get(kk).unwrap();
            if node_deps.len() == 0 || node_deps.iter().all(|dep| *visited.get(dep).unwrap()) {
                result.push(*kk);
                visited.insert(**kk, true);
                break;
            }
        }
    }


    result.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solve_example(name: &str) -> OutputType {
        let input = parse_input(name, false);
        solve(&input.0, &input.1)
    }

    #[test]
    fn examples() {
        assert_eq!(solve_example("example1"), "CABDFE");
    }
}
