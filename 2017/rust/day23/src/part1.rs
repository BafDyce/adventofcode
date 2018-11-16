use super::*;

use std::collections::HashMap;

pub(crate) fn solve(instructions: &[Instruction]) -> Integer {
    let mut registers: HashMap<char, Integer> = HashMap::new();
    let mut ii = 0;
    let mut counter = 0;

    loop {
        if ii >= instructions.len() {
            break;
        }

        match instructions[ii] {
            Instruction::Set(Reference::Register(aa), bb) => {
                let value = eval(&registers, bb);
                let entry = registers.entry(aa).or_insert(0);
                *entry = value;
            }
            Instruction::Sub(Reference::Register(aa), bb) => {
                let value = eval(&registers, bb);
                let entry = registers.entry(aa).or_insert(0);
                *entry -= value;
            }
            Instruction::Mul(Reference::Register(aa), bb) => {
                let value = eval(&registers, bb);
                let entry = registers.entry(aa).or_insert(0);
                *entry *= value;
                counter += 1;
            }
            Instruction::Jnz(aa, bb) => {
                if eval(&registers, aa) != 0 {
                    let jump = eval(&registers, bb) - 1;
                    let new_ii = ii as Integer + jump;
                    if new_ii < 0 {
                        break;
                    }
                    ii = new_ii as usize;
                }
            }
            other => panic!("Invalid instruction: {:?}", other)
        }

        ii += 1;
    }

    counter
}

fn eval(registers: &HashMap<char, Integer>, reference: Reference) -> Integer {
    match reference {
        Reference::Number(number) => number,
        Reference::Register(ref register) => *registers.get(register).unwrap_or(&0)
    }
}
