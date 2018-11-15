use super::*;

pub fn solve(input: String) -> i32 {

    let mut sum = 0;
    let len = input.len();
    let mut ii = 0;
    for ch in input.chars() {
        if ch == input.chars().nth( (ii + len/2) % len).unwrap() {
            let x: i32 = ch.to_string().parse().unwrap();
            sum = sum + x;
        }

        ii += 1;
    }

    // check last char
    if let Some(x) = input.chars().rev().nth(0) {
        if x == input.chars().nth( (len/2) % len).unwrap() {
            let x: i32 = x.to_string().parse().unwrap();
            sum = sum + x;
        }
    }

    sum
}
