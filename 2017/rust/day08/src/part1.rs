use super::*;
use std::collections::HashMap;

pub fn solve(instructions: &Vec<Instruction>) -> i32 {
    let mut registers: HashMap<String, i32> = HashMap::new();
    for instruction in instructions {
        instruction.perform(&mut registers);
    }

    if let Some(max) = registers.values().max() {
        *max
    } else {
        panic!("No registers??");
    }
}
