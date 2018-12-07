use super::*;
use std::collections::HashMap;

type OutputType = isize;

pub fn solve(deps: &HashMap<char, Vec<char>>, visited: &HashMap<char, bool>) -> OutputType {
    let mut resolved: HashMap<char, Option<isize>> = HashMap::new();
    for kk in visited.keys() {
        resolved.insert(*kk, None);
    }

    let mut result = Vec::new();

    let mut keys = deps.keys().collect::<Vec<_>>();
    keys.sort();

    // These two values must be adapted to the actual puzzle configuration
    let mut workers = [0, 0, 0, 0, 0];
    let time_penalty = 60;

    let mut time = 0;
    while resolved.values().any(|vv| !is_resolved(time, *vv)) {
        println!("{} | {:?}", time, workers);
        for worker in &mut workers {
            // worker is still resolving something
            if time < *worker {
                continue;
            }

            for kk in keys.iter() {
                if resolved.get(kk).unwrap().is_some() {
                    continue;
                }

                let node_deps = deps.get(kk).unwrap();
                if node_deps.len() == 0
                || node_deps.iter().all(|dep| is_resolved(time, *resolved.get(dep).unwrap())) {
                    println!("resolving {} @ {}", kk, time);
                    result.push(*kk);
                    let time_finish = time + (**kk as isize - 'A' as isize) + 1 + time_penalty;
                    *worker = time_finish;
                    resolved.insert(**kk, Some(time_finish));
                    break;
                }
            }
        }


        time += 1;
    }


    time
}

fn is_resolved(time_current: isize, time_resolved: Option<isize>) -> bool {
    if let Some(time_resolved) = time_resolved {
        time_resolved <= time_current
    } else {
        false
    }
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
        // actually 15, bute 253 is the result for the example input and the actual worker config
        assert_eq!(solve_example("example1"), 253);
    }
}
