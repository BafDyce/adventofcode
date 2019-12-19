use std::collections::{HashMap, VecDeque};

pub type IntcodeNumber = i128;

#[derive(Debug, Default)]
pub struct IntcodeProcessor {
    stack: Vec<IntcodeNumber>,
    ram: HashMap<usize, IntcodeNumber>,
    ip: usize,
    relbase: IntcodeNumber,
}

impl IntcodeProcessor {
    pub fn new(code: &Vec<IntcodeNumber>) -> Self {
        IntcodeProcessor {
            stack: code.to_owned(),
            .. Default::default()
        }
    }

    pub fn run(&mut self, input: IntcodeNumber, outputs: &mut VecDeque<IntcodeNumber>) -> Option<IntcodeNumber> {
        loop {
            self.ip += match self.stack[self.ip] % 100 {
                1 => {
                    // add
                    let param_1 = self.get_value_of_parameter(1);
                    let param_2 = self.get_value_of_parameter(2);

                    let dst = self.get_addr_from_param(3);
                    self.write_into_addr(dst, param_1 + param_2);

                    4
                }
                2 => {
                    // multiply
                    let param_1 = self.get_value_of_parameter(1);
                    let param_2 = self.get_value_of_parameter(2);

                    let dst = self.get_addr_from_param(3);
                    self.write_into_addr(dst, param_1 * param_2);

                    4
                }
                3 => {
                    // store input
                    let addr = self.get_addr_from_param(1);
                    self.write_into_addr(addr, input);

                    2
                }
                4 => {
                    // get output
                    let output = self.get_value_of_parameter(1);
                    //println!("out: {}", output);
                    outputs.push_back(output);
                    if outputs.len() == 3 {
                        self.ip += 2;
                        return None;
                    }

                    2
                }
                5 => {
                    // jump if true
                    let param_1 = self.get_value_of_parameter(1);
                    let param_2 = self.get_value_of_parameter(2);

                    if param_1 != 0 {
                        self.ip = param_2 as usize;
                        0
                    } else {
                        3
                    }
                }
                6 => {
                    // jump if false
                    let param_1 = self.get_value_of_parameter(1);
                    let param_2 = self.get_value_of_parameter(2);

                    if param_1 == 0 {
                        self.ip = param_2 as usize;
                        0
                    } else {
                        3
                    }
                }
                7 => {
                    // less than
                    let param_1 = self.get_value_of_parameter(1);
                    let param_2 = self.get_value_of_parameter(2);

                    let addr = self.get_addr_from_param(3);
                    self.write_into_addr(
                        addr,
                        if param_1 < param_2 { 1 } else { 0 },
                    );

                    4
                }
                8 => {
                    // equal
                    let param_1 = self.get_value_of_parameter(1);
                    let param_2 = self.get_value_of_parameter(2);

                    let addr = self.get_addr_from_param(3);
                    self.write_into_addr(
                        addr,
                        if param_1 == param_2 { 1 } else { 0 },
                    );

                    4
                }
                9 => {
                    // relbase change
                    let param_1 = self.get_value_of_parameter(1);
                    self.relbase += param_1;

                    2
                }
                99 => {
                    break Some(0);
                }
                other => {
                    panic!("Invalid opcode {} @ {} ({})", other, self.ip, self.stack[self.ip]);
                }
            }
        }
    }

    fn load_from_addr(&mut self, addr: usize) -> IntcodeNumber {
        if addr < self.stack.len() {
            self.stack[addr]
        } else {
            let entry = self.ram.entry(addr).or_insert(0);
            *entry
        }
    }

    fn write_into_addr(&mut self, addr: usize, value: IntcodeNumber) {
        if addr < self.stack.len() {
            self.stack[addr] = value;
        } else {
            let entry = self.ram.entry(addr).or_insert(0);
            *entry = value;
        }
    }

    fn get_value_of_parameter(&mut self, param_idx: usize) -> IntcodeNumber {
        // inverse order than given in description, for easier access via idx
        let modes = [
            (self.stack[self.ip] % 1_000) / 100,
            (self.stack[self.ip] % 10_000) / 1_000,
            self.stack[self.ip] / 10_000,
        ];

        match modes[param_idx - 1] {
            0 => {
                let addr = self.load_from_addr(self.ip + param_idx) as usize;
                self.load_from_addr(addr)
            }
            1 => self.load_from_addr(self.ip + param_idx),
            2 => {
                let addr = self.load_from_addr(self.ip + param_idx) + self.relbase;
                self.load_from_addr(addr as usize)
            }
            other => panic!("get_value_of_parameter: Invalid mode ({})", other),
        }
    }

    fn get_addr_from_param(&mut self, param_idx: usize) -> usize {
        // inverse order than given in description, for easier access via idx
        let modes = [
            (self.stack[self.ip] % 1_000) / 100,
            (self.stack[self.ip] % 10_000) / 1_000,
            self.stack[self.ip] / 10_000,
        ];

        match modes[param_idx - 1] {
            0 => self.load_from_addr(self.ip + param_idx) as usize,
            2 => (self.load_from_addr(self.ip + param_idx) + self.relbase) as usize,
            other => panic!("get_addr_from_param: Invalid mode ({})", other),
        }
    }
}
