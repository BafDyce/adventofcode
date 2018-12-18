use super::*;

pub type OutputType = usize;

fn matches_more_than_three_opcodes(sample: &Sample) -> bool {
    let mut counter = 0;
    let opcodes = vec![
        Opcode::Addr,
        Opcode::Addi,
        Opcode::Mulr,
        Opcode::Muli,
        Opcode::Banr,
        Opcode::Bani,
        Opcode::Borr,
        Opcode::Bori,
        Opcode::Setr,
        Opcode::Seti,
        Opcode::Gtir,
        Opcode::Gtri,
        Opcode::Gtrr,
        Opcode::Eqir,
        Opcode::Eqri,
        Opcode::Eqrr,
    ];

    for opcode in opcodes {
        let mut registers = sample.before.clone();
        let instruction = sample.instruction.with_opcode(opcode);

        instruction.execute(&mut registers);
        if registers == sample.after {
            //println!("{:?} -> {:?} == {:?}", sample.before, registers, sample.after);
            counter += 1;
        }
    }

    counter >= 3
}

pub fn solve(input: &InputType, _config: &PuzzleConfig) -> OutputType {
    let mut more_than_three_matches = 0;
    for sample in &input.samples {
        if matches_more_than_three_opcodes(sample) {
            more_than_three_matches += 1;
        }
    }

    more_than_three_matches
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solve_example(name: &str) -> OutputType {
        let (input, config) = parse_input(name, false);
        solve(&input, &config)
    }

    #[test]
    fn examples() {
        assert_eq!(solve_example("example1"), 1);
    }
}
