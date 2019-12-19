use std::collections::{HashMap, VecDeque};

pub type IntcodeNumber = i128;

#[derive(Debug, Default)]
pub struct IntcodeManager {
    io: IntcodeIOManager,
    cpus: Vec<IntcodeProcessor>,
}

impl IntcodeManager {
    pub fn new(code: &Vec<IntcodeNumber>, amount_cpus: usize) -> Self {
        IntcodeManager {
            io: IntcodeIOManager::new(amount_cpus),
            cpus: {
                let mut cpus = Vec::new();
                for id in 0 .. amount_cpus {
                    cpus.push(IntcodeProcessor::new(code, IntcodeProcessorId::new(id)));
                }

                cpus
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct IntcodeProcessorId {
    id: usize,
}

impl IntcodeProcessorId {
    pub fn new(id: usize) -> Self {
        IntcodeProcessorId {
            id,
        }
    }
}

#[derive(Debug, Default)]
pub struct IntcodeProcessor {
    id: IntcodeProcessorId,
    stack: Vec<IntcodeNumber>,
    ram: HashMap<usize, IntcodeNumber>,
    ip: usize,
    relbase: IntcodeNumber,
}

impl IntcodeProcessor {
    pub fn new(code: &Vec<IntcodeNumber>, id: IntcodeProcessorId) -> Self {
        IntcodeProcessor {
            id,
            stack: code.to_owned(),
            .. Default::default()
        }
    }

    pub fn run(&mut self, input: IntcodeNumber) -> IntcodeNumber {
        let mut output = 0;

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
                    output = self.get_value_of_parameter(1);
                    println!("out: {}", output);

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
                    break output;
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

#[derive(Debug, Default)]
pub struct IntcodeIOManager {
    ios: Vec<VecDeque<IntcodeNumber>>,
}

impl IntcodeIOManager {
    fn new(amount: usize) -> Self {
        let mut manager = Self::default();

        for __ in 0 .. amount {
            manager.ios.push(VecDeque::new());
        }

        manager
    }

    fn get_input(&mut self, cid: IntcodeProcessorId) -> Option<IntcodeNumber> {
        let idx_input = cid.id;
        self.ios[idx_input].pop_front()
    }

    fn save_output(&mut self, cid: IntcodeProcessorId, out: IntcodeNumber) {
        let idx_output = (cid.id + 1) % self.ios.len();
        self.ios[idx_output].push_back(out)
    }
}