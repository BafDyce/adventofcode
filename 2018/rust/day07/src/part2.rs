use super::*;
use std::collections::HashMap;

type OutputType = isize;

pub fn solve(deps: &HashMap<char, Vec<char>>, visited: &HashMap<char, bool>, config: &PuzzleConfig) -> OutputType {
    let mut resolved: HashMap<char, Option<isize>> = HashMap::new();
    for kk in visited.keys() {
        resolved.insert(*kk, None);
    }

    let mut result = Vec::new();

    let mut keys = deps.keys().collect::<Vec<_>>();
    keys.sort();

    let workers_num = config.get("workers").unwrap_or(&"5".to_owned()).parse::<usize>().unwrap();
    let mut workers = Vec::with_capacity(workers_num);
    for __ in 0 .. workers_num {
        workers.push(0);
    }
    let time_penalty = config.get("time_penalty").unwrap_or(&"60".to_owned()).parse::<isize>().unwrap();

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
        let (input, config) = parse_input(name, false);
        solve(&input.0, &input.1, &config)
    }

    #[test]
    fn examples() {
        assert_eq!(solve_example("example1"), 15);
    }
}
