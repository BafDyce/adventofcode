extern crate aocutils;
#[macro_use]
extern crate lazy_static;
extern crate regex;

mod part1;
mod part2;

use regex::Regex;

type Integer = i64;

#[derive(Clone, Copy, Debug)]
enum Reference {
    Register(char),
    Number(Integer),
}

impl Reference {
    pub fn from(spec: &str) -> Reference {
        if let Ok(number) = spec.parse::<Integer>() {
            Reference::Number(number)
        } else {
            Reference::Register(spec.chars().next().unwrap())
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Snd(Reference),
    Set(Reference, Reference),
    Add(Reference, Reference),
    Mul(Reference, Reference),
    Mod(Reference, Reference),
    Rcv(Reference),
    Jgz(Reference, Reference),
}

impl Instruction {
    pub fn from(spec: &str) -> Instruction {
        lazy_static!{
            static ref RE: Regex = Regex::new(
                r"(?P<instr>[a-z]{3}) (?P<aa>[a-z0-9]*)( (?P<bb>[a-z0-9\-]*))?").unwrap();
        }

        let things = RE.captures(spec).unwrap();

        let instr = &things["instr"];
        let first = (&things["aa"]).to_string();
        let second = things.name("bb").and_then(|bb| Some(bb.as_str()));

        //println!("instr: {} {} {:?}", instr, first, second);

        match instr {
            "snd" => {
                Instruction::Snd(Reference::from(&first))
            },
            "set" if second.is_some() => {
                Instruction::Set(Reference::from(&first), Reference::from(second.unwrap()))
            },
            "add" if second.is_some() => {
                Instruction::Add(Reference::from(&first), Reference::from(second.unwrap()))
            },
            "mul" if second.is_some() => {
                Instruction::Mul(Reference::from(&first), Reference::from(second.unwrap()))
            },
            "mod" if second.is_some() => {
                Instruction::Mod(Reference::from(&first), Reference::from(second.unwrap()))
            },
            "rcv" => {
                Instruction::Rcv(Reference::from(&first))
            },
            "jgz" if second.is_some() => {
                Instruction::Jgz(Reference::from(&first), Reference::from(second.unwrap()))
            },
            _ => panic!("Invalid spec: {}", spec)
        }
    }
}

fn main() {
    let day: i32 = 18;

    let input = aocutils::import(day, Some("puzzle1"));


    let mut instructions = Vec::new();
    for line in &input {
        instructions.push( Instruction::from(line) );
    }

    let res1 = part1::solve(&instructions);
    let res2 = part2::solve(&instructions);

    println!("Results: {} and {}", res1, res2);
}
