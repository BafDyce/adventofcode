use super::*;

type OutputType = super::part1::OutputType;

fn get_candidates(sample: &Sample) -> Vec<Opcode> {
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

    let mut candidates = Vec::new();
    for opcode in opcodes {
        let mut registers = sample.before.clone();
        let instruction = sample.instruction.with_opcode(opcode);

        instruction.execute(&mut registers);
        if registers == sample.after {
            candidates.push(opcode);
        }
    }

    candidates
}

pub fn solve(input: &InputType, _config: &PuzzleConfig) -> OutputType {
    // First: get possible instruction candidates for each opcode number
    // opcode numbers are used as indices
    let mut mapping = [Opcode::Unknown(0); 16];
    let mut available = [true; 16];
    let mut possible_opcodes = vec![Vec::new(); 16];
    for sample in &input.samples {
        if let Opcode::Unknown(number) = sample.instruction.get_opcode() {
            let candidates = get_candidates(sample);
            // if we find only one candidate then we know this one for sure.
            if candidates.len() == 1 {
                mapping[number] = candidates[0];
                available[number] = false;
            } else {
                // Otherwise, we just append them to the list.
                // Right now, we don't care for any duplicates
                possible_opcodes[number].extend(candidates);
            }
        }
    }

    // remove duplicates
    for ii in 0 .. possible_opcodes.len() {
        possible_opcodes[ii].sort();
        possible_opcodes[ii].dedup();
    }

    for __ in 0 .. 16 {
        for ii in 0 .. 16 {
            if !available[ii] {
                let opcode = mapping[ii];
                for jj in 0 .. 16 {
                    possible_opcodes[jj].retain(|&item| item != opcode);
                    if possible_opcodes[jj].len() == 1 {
                        let new_opcode = possible_opcodes[jj][0];
                        mapping[jj] = new_opcode;
                        available[jj] = false;
                    }
                }
            }
        }
    }

    let mut registers = RegisterSet::new();
    for instruction in &input.program {
        if let Opcode::Unknown(number) = instruction.get_opcode() {
            let instruction = instruction.with_opcode(mapping[number]);
            instruction.execute(&mut registers);
        } else {
            panic!("FATAL ERROR");
        }
    }

    registers.rs[0]
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
        assert_eq!(solve_example("example1"), OutputType::default());
    }
}
