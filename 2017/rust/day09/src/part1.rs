use super::*;

pub fn solve(input: &String) -> i32 {

    let mut stream: Vec<Packet> = Vec::new();

    let mut score = 1i32;
    let mut in_garbage = false;
    let mut ignore = false;

    for character in input.chars() {
        match character {
            '{' if ! in_garbage => {
                stream.push(Packet::Group(score));
                score += 1;
            },
            '}' if ! in_garbage => {
                    score -= 1;
            },
            '!' if in_garbage && ! ignore => {
                ignore = true;
            },
            '<' if ! in_garbage => {
                    stream.push(Packet::Garbage(0));
                    in_garbage = true;
            },
            '>' if in_garbage && ! ignore => {
                in_garbage = false;
            },
            _ if ignore => {
                ignore = false;
            },
            _ => {},
        }
    }

    let mut sum = 0;
    for packet in stream {
        sum += match packet {
            Packet::Group(val) => val,
            _ => 0,
        };
    }

    sum
}
