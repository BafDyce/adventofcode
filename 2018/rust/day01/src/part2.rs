use super::*;

use super::part1::OutputType;

pub fn solve(input: &InputType) -> OutputType {
    let mut history = Vec::new();
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
