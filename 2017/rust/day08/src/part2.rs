use super::*;
use std::collections::HashMap;

pub fn solve(instructions: &Vec<Instruction>) -> i32 {
    let mut registers: HashMap<String, i32> = HashMap::new();
    let mut max = 0;
    for instruction in instructions {
        let res = instruction.perform(&mut registers);
        if res > max {
            max = res;
        }
    }

    max
}
