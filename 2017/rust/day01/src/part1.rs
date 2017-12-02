use super::*;

pub fn solve(input: String) -> i32 {

    let mut last = '\0';
    let mut sum = 0;
    for ch in input.chars() {
        if ch == last {
            let x: i32 = ch.to_string().parse().unwrap();
            sum = sum + x;
        }

        last = ch;
    }

    // last char
    if let Some(x) = input.chars().nth(0) {
        if x == last {
            let x: i32 = x.to_string().parse().unwrap();
            sum = sum + x;
        }
    }

    sum
}
