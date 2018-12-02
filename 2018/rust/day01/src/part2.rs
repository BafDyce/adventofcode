use super::*;

use super::part1::OutputType;

pub fn solve(input: &InputType) -> OutputType {
    let mut history = vec![0];
    let mut acc = 0;

    for freq in input.iter().cycle() {
        acc += freq;
        if history.contains(&acc) {
            return acc;
        } else {
            history.push(acc);
        }
    }

    0
}

// Original solution
//pub fn solve(input: &InputType) -> OutputType {
//    let mut history = Vec::new();
//    input.iter().cycle().fold(0i64, |acc, xx| {
//        let res = acc + xx;
//        if history.contains(&res) {
//            println!("res: {}", res);
//            panic!()
//        } else {
//            history.push(res);
//        }
//
//        res
//    })
//}

#[cfg(test)]
mod tests {
    use super::*;

    fn solve_example(name: &str) -> OutputType {
        let input = parse_input(name, false);
        solve(&input)
    }

    #[test]
    fn examples() {
        assert_eq!(solve_example("example1"), 2);
        assert_eq!(solve_example("example5"), 0);
        assert_eq!(solve_example("example6"), 10);
        assert_eq!(solve_example("example7"), 5);
        assert_eq!(solve_example("example8"), 14);
    }
}
