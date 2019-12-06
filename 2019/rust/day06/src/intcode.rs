fn run_intcode_program(program: &Vec<i32>, input: i32) -> OutputType1 {
    let mut memory = program.to_owned();
    let mut output = 0;

    let mut ip = 0;
    loop {
        let modes = [
            memory[ip] / 10_000,
            (memory[ip] % 10_000) / 1_000,
            (memory[ip] % 1_000) / 100,
        ];

        // Tbh, I created these three closures when cleaning up the code in the evening.
        // In my original solution I had copy & pasted these `match modes[ip] {}` blocks all over
        // the place (and fortunately changed all offsets correctly on first try :D)
        let get_mode_idx = |param_idx| match param_idx {
            1 => 2,
            2 => 1,
            3 => 0,
            _ => panic!("Invalid mode idx"),
        };

        let get_value_of_parameter = |param_idx| {
            let mode_idx = get_mode_idx(param_idx);

            match modes[mode_idx] {
                0 => {
                    let addr = memory[ip + param_idx] as usize;
                    memory[addr]
                }
                1 => memory[ip + param_idx],
                other => panic!("get_value_of_parameter: Invalid mode ({})", other),
            }
        };

        let get_addr_from_param = |param_idx| {
            let mode_idx = get_mode_idx(param_idx);

            match modes[0] {
                0 => {
                    memory[ip + param_idx] as usize
                }
                other => panic!("get_addr_from_param: Invalid mode ({})", other),
            }
        };

        ip += match memory[ip] % 100 {
            1 => {
                // add
                let param_1 = get_value_of_parameter(1);
                let param_2 = get_value_of_parameter(2);

                let dst = get_addr_from_param(3);
                memory[dst] = param_1 + param_2;

                4
            }
            2 => {
                // multiply
                let param_1 = get_value_of_parameter(1);
                let param_2 = get_value_of_parameter(2);

                let dst = get_addr_from_param(3);
                memory[dst] = param_1 * param_2;

                4
            }
            3 => {
                // store input
                let addr = get_addr_from_param(1);
                memory[addr] = input;

                2
            }
            4 => {
                // get output
                output = get_value_of_parameter(1);

                2
            }
            5 => {
                // jump if true
                let param_1 = get_value_of_parameter(1);
                let param_2 = get_value_of_parameter(2);

                if param_1 != 0 {
                    ip = param_2 as usize;
                    0
                } else {
                    3
                }
            }
            6 => {
                // jump if false
                let param_1 = get_value_of_parameter(1);
                let param_2 = get_value_of_parameter(2);

                if param_1 == 0 {
                    ip = param_2 as usize;
                    0
                } else {
                    3
                }
            }
            7 => {
                // less than
                let param_1 = get_value_of_parameter(1);
                let param_2 = get_value_of_parameter(2);

                let addr = get_addr_from_param(3);
                memory[addr] = if param_1 < param_2 { 1 } else { 0 };

                4
            }
            8 => {
                // less than
                let param_1 = get_value_of_parameter(1);
                let param_2 = get_value_of_parameter(2);

                let addr = get_addr_from_param(3);
                memory[addr] = if param_1 == param_2 { 1 } else { 0 };

                4
            }
            99 => {
                break output;
            }
            other => {
                panic!(
                    "Invalid opcode {} @ {} ({})",
                    other, ip, memory[ip]
                );
            }
        }
    }
}