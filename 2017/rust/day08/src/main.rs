extern crate aocutils;
mod part1;
mod part2;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Instruction {
    operation: Operation,
    condition: Condition,
}

impl Instruction {
    fn new_from(desc: String) -> Instruction {
        let parts = desc.split(" if ").map(|x| x.to_string()).collect::<Vec<String>>();
        assert_eq!(parts.len(), 2);
        let operation = Operation::new_from(&parts[0]);
        let condition = Condition::new_from(&parts[1]);

        Instruction{ operation: operation, condition: condition }
    }

    fn perform(&self, registers: &mut HashMap<String, i32>) -> i32 {
        if self.condition.check(registers) {
            self.operation.perform(registers)
        } else {
            0
        }
    }
}

#[derive(Debug)]
pub enum Operation {
    INC(String, i32),
    DEC(String, i32)
}

impl Operation {
    fn new_from(desc: &String) -> Operation {
        let elements: Vec<&str> = desc.split_whitespace().collect();
        assert_eq!(elements.len(), 3);

        let reg = elements[0].to_string();
        let val = elements[2].parse::<i32>().unwrap();
        match elements[1] {
            "inc" => {
                Operation::INC(reg, val)
            },
            "dec" => {
                Operation::DEC(reg, val)
            },
            _ => panic!("Invalid operation: {}", elements[1]),
        }
    }

    fn perform(&self, registers: &mut HashMap<String, i32>) -> i32 {
        match *self {
            Operation::INC(ref reg, ref val) => {
                let reg = registers.entry(reg.clone()).or_insert(0);
                *reg += val;
                *reg
            },
            Operation::DEC(ref reg, ref val) => {
                let reg = registers.entry(reg.clone()).or_insert(0);
                *reg -= val;
                *reg
            }
        }
    }
}

#[derive(Debug)]
pub enum Condition {
    EQUAL(String, i32),
    NOTEQUAL(String, i32),
    LESS(String, i32),
    LESSEQUAL(String, i32),
    GREATER(String, i32),
    GREATEREQUAL(String, i32),
}

impl Condition {
    fn new_from(desc: &String) -> Condition {
        let elements: Vec<&str> = desc.split_whitespace().collect();
        assert_eq!(elements.len(), 3);

        let reg = elements[0].to_string();
        let cond = elements[1];
        let val = elements[2].parse::<i32>().unwrap();

        match cond {
            "==" => Condition::EQUAL(reg, val),
            "!=" => Condition::NOTEQUAL(reg, val),
            "<" =>  Condition::LESS(reg, val),
            "<=" => Condition::LESSEQUAL(reg, val),
            ">" =>  Condition::GREATER(reg, val),
            ">=" => Condition::GREATEREQUAL(reg, val),
            _ =>    panic!("Invalid condition: {}", cond),
        }
    }

    fn check(&self, registers: &mut HashMap<String, i32>) -> bool {
        match *self {
            Condition::EQUAL(ref reg, ref val) => {
                let regval = Condition::get_regval(reg, registers);
                regval == *val
            },
            Condition::NOTEQUAL(ref reg, ref val) => {
                let regval = Condition::get_regval(reg, registers);
                regval != *val
            },
            Condition::LESS(ref reg, ref val) => {
                let regval = Condition::get_regval(reg, registers);
                regval < *val
            },
            Condition::LESSEQUAL(ref reg, ref val) => {
                let regval = Condition::get_regval(reg, registers);
                regval <= *val
            },
            Condition::GREATER(ref reg, ref val) => {
                let regval = Condition::get_regval(reg, registers);
                regval > *val
            },
            Condition::GREATEREQUAL(ref reg, ref val) => {
                let regval = Condition::get_regval(reg, registers);
                regval >= *val
            },
        }
    }

    fn get_regval(reg: &String, registers: &mut HashMap<String, i32>) -> i32 {
        match registers.get(reg) {
            Some(val) => *val,
            None => 0,
        }
    }
}

fn main() {
    let day: i32 = 8;
    let input = aocutils::import(day, Some("puzzle1"));

    let mut instructions: Vec<Instruction> = Vec::new();
    for line in input {
        instructions.push(Instruction::new_from(line));
    }

    let res1 = part1::solve(&instructions);
    let res2 = part2::solve(&instructions);

    println!("Results: {} and {}", res1, res2);
}
