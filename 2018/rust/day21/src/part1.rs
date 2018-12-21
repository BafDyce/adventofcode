use super::*;

pub type OutputType = usize;

pub fn solve(_input: &InputType, _config: &PuzzleConfig) -> OutputType {
    // Improved/Reduced/optimized code:
    // We can ignore the frist few instructions (= outer loop) as 123 & 456 will always be 72.

    let mut dd = 0;
    'jmp_6: loop {
        let mut cc = dd | 65536;
        dd = 1397714;

        'jmp_8: loop {
            dd += cc & 255;
            dd &= 16777215;
            dd *= 65899;
            dd &= 16777215;

            if 256 > cc {
                return dd;
            } else {
                /*
                ff = 0;
                'jmp_18: loop {
                    bb = ff + 1;
                    bb *= 256;

                    if bb > cc {
                        cc = ff;

                        break; // continue 'jmp_18;
                    } else {
                        ff += 1;
                    }
                }
                */

                // -> rewritten:
                /*
                for ff in 0 .. {
                    //loop_counter_cc += 1;
                    let bb = (ff + 1) * 256;
                    if bb > cc {
                        cc = ff;
                        break; // continue 'jmp_8;
                    }
                }
                */

                // -> rewritten:
                cc /= 256;
            }
        }
    }
    /*


// Translated code:
    // 0
    //    seti 123 0 3
    dd = 123;
// JUMP FROM 4
    // 1
    //    bani 3 456 3
    dd &= 456;
    // 2
    //    eqri 3 72 3
    // 3
    //    addr 3 4 4
    // 4
    //    seti 0 0 4
    // 5
    //    seti 0 2 3
    // 2 + 3
    if dd == 72 {
        // 5
        dd = 0;
// JUMP FROM 30
        // 6
        //    bori 3 65536 2
        cc = dd | 65536;
        // 7
        //    seti 1397714 1 3
        dd = 1397714;

// JUMP FROM 27
        // 8
        //    bani 2 255 5
        ff = cc & 255;
        // 9
        //    addr 3 5 3
        dd += ff;
        // 10
        //    bani 3 16777215 3
        dd &= 16777215;
        // 11
        //    muli 3 65899 3
        dd *= 65899;
        // 12
        //    bani 3 16777215 3
        dd &= 16777215;
        // 13
        //    gtir 256 2 5
        // 14
        //    addr 5 4 4
        // 15
        //    addi 4 1 4
        // 16
        //    seti 27 6 4
        if 256 > cc {
            // 16
            //jmp(28);
            // 28
            //    eqrr 3 0 5
            // 29
            //    addr 5 4 4
            // 30
            //    seti 5 8 4
            if dd == aa {
                exit();
            }

            // 30
            jmp(6);
        } else {
            // 15
            //jmp(17);
            // 17
            //    seti 0 6 5
            ff = 0;
// JUMP FROM 25
            // 18
            //    addi 5 1 1
            bb = ff + 1;
            // 19
            //    muli 1 256 1
            bb *= 256;
            // 20
            //    gtrr 1 2 1
            // 21
            //    addr 1 4 4
            // 22
            //    addi 4 1 4
            // 23
            //    seti 25 2 4
            if bb > cc {
                // 23
                //jmp(26);
                // 26
                //    setr 5 7 2
                cc = ff;
                // 27
                //    seti 7 4 4
                jmp(8);
            } else {
                //jmp(24);
                // 24
                //    addi 5 1 5
                ff += 1;
                // 25
                //    seti 17 0 4
                jmp(18);
            }
        }

    } else {
        // 4
        //    seti 0 0 4
        jmp(1);
    }

    0
    */

    /*
x    seti 123 0 3
x    bani 3 456 3
x    eqri 3 72 3
    addr 3 4 4
    seti 0 0 4
    seti 0 2 3
    bori 3 65536 2
    seti 1397714 1 3
    bani 2 255 5
    addr 3 5 3
    bani 3 16777215 3
    muli 3 65899 3
    bani 3 16777215 3
    gtir 256 2 5
    addr 5 4 4
    addi 4 1 4
    seti 27 6 4
    seti 0 6 5
    addi 5 1 1
    muli 1 256 1
    gtrr 1 2 1
    addr 1 4 4
    addi 4 1 4
    seti 25 2 4
    addi 5 1 5
    seti 17 0 4
    setr 5 7 2
    seti 7 4 4
    eqrr 3 0 5
    addr 5 4 4
    seti 5 8 4


    ip = 4;
    rs[3] = 123;
    rs[3] = rs[3] & 546;
    if rs[3] == 72 {
        rs[3] = 0;
    } else {
        rs[4] = 1
    }
    // when i reached this point of reversing the program, my stupid brute force (see below)
    // already found the first solution candidate, which I promptly tried and it was correct :P
    */


    // original solution, which got me rank 86 on the leaderboard
    /*
    let mut program = input.program.to_owned();
    let set_ip = program.pop_front().unwrap();


    'outer: for start in 5000 .. {
        let mut regs = RegisterSet::new();
        set_ip.execute(&mut regs);
        regs.rs[0] = start;

        let mut ip = 0;
        let mut counter = 0;
        while ip < program.len() {
            ip = program[ip].execute(&mut regs);
            //println!("{:?}", regs.rs);
            counter += 1;

            if counter == 10_000 {
                continue 'outer;
            }
        }

        return start;
    }
    */
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
