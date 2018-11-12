use super::*;

use std::collections::VecDeque;
use std::collections::HashMap;

#[derive(Debug)]
struct Program {
    instructions: Vec<Instruction>,
    registers: HashMap<char, Integer>,
    ii: usize,
    queue: VecDeque<Integer>,
    terminated: bool,
}

#[derive(Debug)]
struct TerminationError {}

impl Program {
    pub fn new(instructions: &[Instruction], pid: Integer) -> Program {
        Program {
            instructions: (*instructions).to_vec(),
            registers: {
                let mut registers = HashMap::new();
                registers.insert('p', pid);
                registers
            },
            ii: 0,
            queue: VecDeque::new(),
            terminated: false,
        }
    }

    pub fn exec_next(&mut self) -> Result<(bool, Option<Integer>), TerminationError> {
        if self.terminated || self.ii >= self.instructions.len() {
            return Err(TerminationError{});
        }

        let mut send_item = None;
        let mut waiting = false;
        match self.instructions[self.ii] {
            Instruction::Add(Reference::Register(aa), bb) => {
                let value = self.eval(bb);
                let entry = self.registers.entry(aa).or_insert(0);
                *entry += value;
                self.ii += 1;
            }
            Instruction::Jgz(aa, bb) => {
                if self.eval(aa) > 0 {
                    let mut jump = self.eval(bb);
                    let new_ii = self.ii as Integer + jump;
                    if new_ii < 0 {
                        self.terminated = true;
                    }
                    self.ii = new_ii as usize;
                } else {
                    self.ii += 1;
                }
            }
            Instruction::Mod(Reference::Register(aa), bb) => {
                let value = self.eval(bb);
                let entry = self.registers.entry(aa).or_insert(0);
                *entry %= value;
                self.ii += 1;
            }
            Instruction::Mul(Reference::Register(aa), bb) => {
                let value = self.eval(bb);
                let entry = self.registers.entry(aa).or_insert(0);
                *entry *= value;
                self.ii += 1;
            }
            Instruction::Rcv(Reference::Register(aa)) => {
                if let Some(item) = self.queue.pop_front() {
                    let entry = self.registers.entry(aa).or_insert(0);
                    *entry = item;
                    self.ii += 1;
                } else {
                    waiting = true;
                }
            }
            Instruction::Set(Reference::Register(aa), bb) => {
                let value = self.eval(bb);
                let entry = self.registers.entry(aa).or_insert(0);
                *entry = value;
                self.ii += 1;
            }
            Instruction::Snd(aa) => {
                send_item = Some(self.eval(aa));
                self.ii += 1;
            }
            other => panic!("Invalid instruction: {:?}", other)
        }
        Ok((waiting, send_item))
    }

    fn eval(&self, reference: Reference) -> Integer {
        match reference {
            Reference::Number(number) => number,
            Reference::Register(ref register) => *self.registers.get(register).unwrap_or(&0)
        }
    }

    pub fn queue_add(&mut self, items: &[Integer]) {
        self.queue.extend(items)
    }
}

#[derive(Debug)]
struct ProgramContainer {
    pub prog: Program,
    pub dead: bool,
    pub waiting: bool,
    pub send_count: usize,
}

pub(crate) fn solve(instructions: &[Instruction]) -> usize {

    let mut programs = vec![
        ProgramContainer {
            prog: Program::new(instructions, 0),
            dead: false,
            send_count: 0,
            waiting: false,
        },
        ProgramContainer {
            prog: Program::new(instructions, 1),
            dead: false,
            send_count: 0,
            waiting: false,
        },
    ];
    let mut ii = 0;

    let mut queue = Vec::new();
    loop {
        {
            ii = (ii + 1) % programs.len();
            let prog = &mut programs[ii];
            prog.prog.queue_add(&queue);
            queue = Vec::new();

            if !prog.dead {
                loop {
                    if let Ok((waiting, send_item)) = prog.prog.exec_next() {
                        if let Some(send_item) = send_item {
                            queue.push(send_item);
                        }

                        prog.waiting = waiting;

                        if waiting {
                            break;
                        }
                    } else {
                        prog.dead = true;
                        break;
                    }
                }
            }

            prog.send_count += queue.len();
        }

        if all_dead(&programs) || (all_waiting(&programs) && queue.len() == 0) {
            return programs[1].send_count;
        }
    }
}

fn all_dead(progs: &[ProgramContainer]) -> bool {
    progs.iter().all(|prog| prog.dead)
}

fn all_waiting(progs: &[ProgramContainer]) -> bool {
    progs.iter().all(|prog| prog.waiting)
}
