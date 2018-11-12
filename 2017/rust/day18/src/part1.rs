use super::*;

use std::collections::HashMap;

pub(crate) fn solve(instructions: &[Instruction]) -> Integer {
//pub fn solve(input: &Vec<i32>) -> i32 {

    let mut registers: HashMap<char, Integer> = HashMap::new();
    let mut ii = 0;
    let mut last = 0;

    loop {
        match instructions[ii] {
            Instruction::Add(Reference::Register(aa), bb) => {
                let value = eval(&registers, bb);
                let entry = registers.entry(aa).or_insert(0);
                *entry += value;
            }
            Instruction::Jgz(aa, bb) => {
                if eval(&registers, aa) > 0 {
                    let jump = eval(&registers, bb) - 1;
                    let new_ii = ii as Integer + jump;
                    if new_ii < 0 {
                        break last;
                    }
                    ii = new_ii as usize;
                }
            }
            Instruction::Mod(Reference::Register(aa), bb) => {
                let value = eval(&registers, bb);
                let entry = registers.entry(aa).or_insert(0);
                *entry %= value;
            }
            Instruction::Mul(Reference::Register(aa), bb) => {
                let value = eval(&registers, bb);
                let entry = registers.entry(aa).or_insert(0);
                *entry *= value;
            }
            Instruction::Rcv(aa) => {
                if eval(&registers, aa) != 0 {
                    break last;
                }
            }
            Instruction::Set(Reference::Register(aa), bb) => {
                let value = eval(&registers, bb);
                let entry = registers.entry(aa).or_insert(0);
                *entry = value;
            }
            Instruction::Snd(aa) => {
                last = eval(&registers, aa);
            }
            other => panic!("Invalid instruction: {:?}", other)
        }

        ii += 1;
    }
}

fn eval(registers: &HashMap<char, Integer>, reference: Reference) -> Integer {
    match reference {
        Reference::Number(number) => number,
        Reference::Register(ref register) => *registers.get(register).unwrap_or(&0)
    }
}
