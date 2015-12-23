// adventofcode - day 23
// part 0

use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

enum Instruction {
    Half(char),
    Tripple(char),
    Inc(char),
    Jump(i32),
    Jie(char, i32),
    Jio(char, i32),
    Nothing,
}

impl Instruction {
    #[allow(dead_code)]
    fn print(&self) {
        match *self {
            Instruction::Half(ref reg)  => {
                println!("Halves {}", reg);
            },
            Instruction::Tripple(ref reg)  => {
                println!("Tripples {}", reg);
            },
            Instruction::Inc(ref reg)  => {
                println!("Increments {}", reg);
            },
            Instruction::Jump(ref offset)  => {
                println!("Jumps {} positions", offset);
            },
            Instruction::Jie(ref reg, ref offset)  => {
                println!("Jumps {} positions if {} is even", offset, reg);
            },
            Instruction::Jio(ref reg, ref offset)  => {
                println!("Jumps {} positions if {} is one", offset, reg);
            },
            Instruction::Nothing  => {
                println!("Nothing");
            },
        };
    }

    fn run(&self, registers: &mut HashMap<char, Register>) -> i32 {
        match *self {
            Instruction::Half(ref reg)  => {
                if let Some(x) = registers.get_mut(&reg) {
                    x.value /= 2;
                };
                1
            },
            Instruction::Tripple(ref reg)  => {
                if let Some(x) = registers.get_mut(&reg) {
                    x.value *= 3;
                };
                1
            },
            Instruction::Inc(ref reg)  => {
                if let Some(x) = registers.get_mut(&reg) {
                    x.value += 1;
                };
                1
            },
            Instruction::Jump(ref offset)  => {
                *offset
            },
            Instruction::Jie(ref reg, ref offset)  => {
                if let Some(x) = registers.get_mut(&reg) {
                    if x.value % 2 == 0 {
                        return *offset;
                    } else {
                        return 1;
                    }
                };
                1
            },
            Instruction::Jio(ref reg, ref offset)  => {
                if let Some(x) = registers.get_mut(&reg) {
                    if x.value == 1 {
                        return *offset;
                    } else {
                        return 1;
                    }
                };
                1
            },
            Instruction::Nothing  => {
                1
            },
        }
    }
}

struct Register{
    #[allow(dead_code)]
    id: char,
    value: u32,
}

fn main(){
    println!("Advent of Code - day 23 | part 2");

    // import data
    let data = import_data();

    let instructions = parse_data(data);
    let mut registers = HashMap::new();

    registers.insert('a', Register{id: 'a', value: 1} );
    registers.insert('b', Register{id: 'b', value: 0} );

    let mut ptr = 0;
    loop {
        let x = ptr as i32 + instructions[ptr].run(&mut registers);

        if x < 0 || x as usize >= instructions.len() {
            break;
        } else {
            ptr = x as usize;
        }
    }

    if let Some(x) = registers.get_mut(&'b') {
        println!("Value of b is {}", x.value);
    };
}

fn parse_data(data: String) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    for mut line in data.lines() {
        let instr = if eat(&mut line, "hlf "){
            Instruction::Half(match line.chars().nth(0){
                Some(x) => x,
                None    => panic!("Help!"),
            })
        } else if eat(&mut line, "tpl "){
            Instruction::Tripple(match line.chars().nth(0){
                Some(x) => x,
                None    => panic!("Help!"),
            })
        } else if eat(&mut line, "inc "){
            Instruction::Inc(match line.chars().nth(0){
                Some(x) => x,
                None    => panic!("Help!"),
            })
        } else if eat(&mut line, "jmp "){
            Instruction::Jump(line.parse::<i32>().unwrap())
        } else if eat(&mut line, "jie "){
            let x = line.split(", ").collect::<Vec<_>>();

            Instruction::Jie(match x[0].chars().nth(0) {
                Some(x) => x,
                None    => panic!("Help!"),
            }, x[1].parse::<i32>().unwrap())
        } else if eat(&mut line, "jio "){
            let x = line.split(", ").collect::<Vec<_>>();

            Instruction::Jio(match x[0].chars().nth(0) {
                Some(x) => x,
                None    => panic!("Help!"),
            }, x[1].parse::<i32>().unwrap())
        } else {
            Instruction::Nothing
        };

        instructions.push(instr);
    }

    instructions
}

fn eat(s: &mut &str, expect: &str) -> bool {
    if s.starts_with(expect) {
        *s = &s[expect.len()..];
        true
    } else {
        false
    }
}

// This function simply imports the data set from a file called input.txt
fn import_data() -> String {
    let mut file = match File::open("../../inputs/23.txt") {
        Ok(f) => f,
        Err(e) => panic!("file error: {}", e),
    };

    let mut data = String::new();
    match file.read_to_string(&mut data){
        Ok(_) => {},
        Err(e) => panic!("file error: {}", e),
    };

    data.pop();
    data
}
