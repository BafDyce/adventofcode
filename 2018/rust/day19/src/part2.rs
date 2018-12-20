use super::*;

use regex::Regex;
use std::collections::HashMap;

type OutputType = super::part1::OutputType;

// 363871 -> too low
pub fn solve(input: &InputType, config: &PuzzleConfig) -> OutputType {
    let mut aa = 1; // rs[0]
    let mut bb = 0; // rs[1]
    let mut cc = 0; // rs[2]
    let mut __ = 0; // rs[3]
    let mut dd = 0; // rs[4]
    let mut ee = 0; // rs[5]

    let jmp = |_|{};

    // calculate target value
    /*
    dd += 2;
    dd *= dd;
    dd *= 19;
    dd *= 11;
    bb += 6;
    bb *= 22;
    bb += 21;
    dd += bb;
    */
    // bb = 153; // = 6 * 22 + 21;
    // dd = 989; // = ((2 * 2) * 19) * 11 + bb;

    /*
    bb = 27;
    bb *= 28;
    bb += 29;
    bb *= 30;
    bb *= 14;
    bb *= 32;
    dd += bb;
    aa = 0;
    */

    bb = 10550400; //(((27 * 28 + 29) * 30) * 14) * 32;
    dd = 10551389; //989 + 10550400;
    aa = 0;

    for ee in 1 ..= dd {
        // the '/ ee' optimization was naturally NOT part of the input and was the entire point
        // of this challenge ;)
        for cc in 1 ..= dd / ee {
        // looking for sweet divisors
            if ee * cc == dd {
                // accumulate
                aa += ee;
            }
        }
    }

    return aa;
    // ================
    // I will keep my

// 0
    jmp(7);
// 1
// JUMP FROM 26
// JUMP FROM 35
    ee = 1;
// 2
// JUMP FROM 13
    cc = 1;
// 3
// JMP FROM 9
    bb = ee * cc;
    if bb == dd {
        jmp(4);
    } else {
        jmp(5);
    }
// 4 (7)
// JUMP FROM 4
    aa += ee;
// 5 (8)
// JUMP FROM 4
    cc += 1;
    if cc > dd {
        jmp(6);
    } else {
        jmp(3);
    }
// 6 (12)
// JUMP FROM 9
    ee += 1;
    if ee > dd {
        return aa;
    } else {
        jmp(2);
    }
// 7 (17)
// JUMP FROM 0
    dd += 2;
    dd *= dd;
    dd *= 19;
    dd *= 11;
    bb += 6;
    bb *= 22;
    bb += 21;
    dd += bb;


    //jmp(8 + aa);
    if aa < 9 {
        jmp(8 + aa);
    } else if aa == 9 {
        jmp(1);
    } else {
        return aa;
    }
// 8
// jump from 25 if aa == 0
    jmp(1);
// 9
// jump from 25 if aa == 1
    bb = 27;
// 10
// jump from 25 if aa == 2
    bb *= 28;
// 11
// jump from 25 if aa == 3
    bb += 29;
// 12
// jump from 25 if aa == 4
    bb *= 30;
// 13
// jump from 25 if aa == 5
    bb *= 14;
// 14
// jump from 25 if aa == 6
    bb *= 32;
// 15
// jump from 25 if aa == 7
    dd += bb;
// 16
// jump from 25 if aa == 8
    aa = 0;
// 17
// jump from 25 if aa == 9
    jmp(1);

    0
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
