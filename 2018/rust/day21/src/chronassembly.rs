use regex::Regex;
use std::{collections::VecDeque, num::ParseIntError, str::FromStr};

type RegisterValue = usize;

#[derive(Debug, Clone)]
pub enum ParseError {
    InvalidInstructionFormat,
    InvalidNumber,
}

impl From<ParseIntError> for ParseError {
    fn from(__: ParseIntError) -> ParseError {
        ParseError::InvalidNumber
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum Opcode {
    Ipset(usize),
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
    Unknown(usize),
}

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    opcode: Opcode,
    aa: usize,
    bb: usize,
    cc: usize,
}

impl Instruction {
    pub fn with_opcode(&self, opcode: Opcode) -> Instruction {
        Instruction {
            opcode,
            .. *self
        }
    }

    pub fn get_opcode(&self) -> Opcode {
        self.opcode
    }

    pub fn execute(&self, registerset: &mut RegisterSet) -> usize {
        let rs = &mut registerset.rs;
        let ip_idx = registerset.ip_idx;

        rs[ip_idx] = registerset.ip;

        match self.opcode {
            Opcode::Unknown(_) => println!("FATAL ERROR! Tried to execute unknown opcode!"),
            Opcode::Ipset(ip_idx) => {
                registerset.ip_idx = ip_idx;
                return registerset.ip;
            },
            Opcode::Addr => {
                rs[ self.cc ] = rs[ self.aa ] + rs[ self.bb ];
            }
            Opcode::Addi => {
                rs[ self.cc ] = rs[ self.aa ] + self.bb;
            }
            Opcode::Mulr => {
                rs[ self.cc ] = rs[ self.aa ] * rs[ self.bb ];
            }
            Opcode::Muli => {
                rs[ self.cc ] = rs[ self.aa ] * self.bb;
            }
            Opcode::Banr => {
                rs[ self.cc ] = rs[ self.aa ] & rs[ self.bb ];
            }
            Opcode::Bani => {
                rs[ self.cc ] = rs[ self.aa ] & self.bb;
            }
            Opcode::Borr => {
                rs[ self.cc ] = rs[ self.aa ] | rs[ self.bb ];
            }
            Opcode::Bori => {
                rs[ self.cc ] = rs[ self.aa ] | self.bb;
            }
            Opcode::Setr => {
                rs[ self.cc ] = rs[ self.aa ];
            }
            Opcode::Seti => {
                rs[ self.cc ] = self.aa;
            }
            Opcode::Gtir => {
                rs[ self.cc ] = if self.aa > rs[ self.bb ] {
                    1
                } else {
                    0
                }
            }
            Opcode::Gtri => {
                rs[ self.cc ] = if rs [ self.aa ] > self.bb {
                    1
                } else {
                    0
                }
            }
            Opcode::Gtrr => {
                rs[ self.cc ] = if rs[ self.aa ] > rs[ self.bb ] {
                    1
                } else {
                    0
                }
            }
            Opcode::Eqir => {
                rs[ self.cc ] = if self.aa == rs[ self.bb ] {
                    1
                } else {
                    0
                }
            }
            Opcode::Eqri => {
                rs[ self.cc ] = if rs [ self.aa ] == self.bb {
                    1
                } else {
                    0
                }
            }
            Opcode::Eqrr => {
                rs[ self.cc ] = if rs[ self.aa ] == rs[ self.bb ] {
                    1
                } else {
                    0
                }
            }
        }

        registerset.ip = rs[ip_idx] + 1;
        registerset.ip
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(ss: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+|\w+)\s+(\d+)\s+(\d+)\s+(\d+)").unwrap();
            static ref RE_IP: Regex = Regex::new(r"\#ip\s+(\d)").unwrap();
        }

        match RE.captures(ss) {
            Some(caps) => {
                Ok(Instruction {
                    opcode: match &caps[1] {
                        "addr" => Opcode::Addr,
                        "addi" => Opcode::Addi,
                        "mulr" => Opcode::Mulr,
                        "muli" => Opcode::Muli,
                        "banr" => Opcode::Banr,
                        "bani" => Opcode::Bani,
                        "borr" => Opcode::Borr,
                        "bori" => Opcode::Bori,
                        "setr" => Opcode::Setr,
                        "seti" => Opcode::Seti,
                        "gtir" => Opcode::Gtir,
                        "gtri" => Opcode::Gtri,
                        "gtrr" => Opcode::Gtrr,
                        "eqir" => Opcode::Eqir,
                        "eqri" => Opcode::Eqri,
                        "eqrr" => Opcode::Eqrr,
                        _ => Opcode::Unknown(caps[1].parse()?),
                    },
                    aa: caps[2].parse()?,
                    bb: caps[3].parse()?,
                    cc: caps[4].parse()?,
                })
            }
            None => {
                match RE_IP.captures(ss) {
                    Some(caps) => Ok(Instruction {
                        opcode: Opcode::Ipset(caps[1].parse().unwrap()),
                        aa: 0,
                        bb: 0,
                        cc: 0,
                    }),
                    None => Err(ParseError::InvalidInstructionFormat)
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RegisterSet {
    pub ip_idx: usize,
    pub ip: usize,
    pub rs: [RegisterValue; 6],
}

impl RegisterSet {
    pub fn new() -> RegisterSet {
        RegisterSet {
            ip_idx: 0,
            ip: 0,
            rs: [0; 6]
        }
    }
}

impl FromStr for RegisterSet {
    type Err = ParseError;

    fn from_str(ss: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"[Before|After]:\s+\[(\d+),\s+(\d+),\s+(\d+),\s+(\d+)\]"
            ).unwrap();
        }

        match RE.captures(ss) {
            Some(caps) => {
                Ok(RegisterSet{
                    ip_idx: 0,
                    ip: 0,
                    rs: [
                        caps[1].parse()?,
                        caps[2].parse()?,
                        caps[3].parse()?,
                        caps[4].parse()?,
                        0,
                        0,
                ]})
            }
            None => Err(ParseError::InvalidInstructionFormat)
        }
    }
}

#[derive(Debug, Clone)]
pub struct InputData {
    pub samples: Vec<Sample>,
    pub program: VecDeque<Instruction>,
}

#[derive(Debug, Clone, Copy)]
pub struct Sample {
    pub before: RegisterSet,
    pub instruction: Instruction,
    pub after: RegisterSet,
}

impl InputData {
    pub fn from_input(input: Vec<String>) -> InputData {
        enum Cap {
            Regset(RegisterSet),
            Instr(Instruction),
        }

        let mut parsed: VecDeque<Cap> = VecDeque::new();
        let mut samples = Vec::new();
        for line in input {
            let item = if let Ok(instruction) = line.parse::<Instruction>() {
                Some(Cap::Instr(instruction))
            } else if let Ok(registerset) = line.parse::<RegisterSet>() {
                Some(Cap::Regset(registerset))
            } else {
                None
            };

            if let Some(item) = item {
                parsed.push_back(item);
            }/* else if parsed.len() == 3 {
                let mut removed = 0;
                if let Some(Cap::Regset(before)) = parsed.get(0) {
                    if let Some(Cap::Instr(instr)) = parsed.get(1) {
                        if let Some(Cap::Regset(after)) = parsed.get(2) {
                            let sample = Sample {
                                before: before.to_owned(),
                                instruction: instr.to_owned(),
                                after: after.to_owned(),
                            };

                            samples.push(sample);
                            removed = 3;
                        }
                    }
                }

                for __ in 0 .. removed {
                    let __ = parsed.pop_front();
                }
            }*/
        }

        let mut program = VecDeque::new();
        while let Some(Cap::Instr(instruction)) = parsed.pop_front() {
            program.push_back(instruction);
        }

        InputData {
            samples: samples,
            program: program,
        }
    }
}
