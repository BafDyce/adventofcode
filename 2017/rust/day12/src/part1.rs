use super::*;

use regex::Regex;
use std::collections::VecDeque;

pub fn solve(input: &Vec<String>) -> usize {
    let mut connections: Vec<Vec<usize>> = Vec::with_capacity(input.len());

    let re = Regex::new(r"(?P<src>\d+) <-> (?P<others>.*)").unwrap();
    for line in input {
        let things = re.captures(line).unwrap();
        let src = things["src"].parse::<usize>().unwrap();
        let mut others = Vec::new();
        for other in things["others"].split(", ") {
            let other = other.parse::<usize>().unwrap();
            others.push(other);
        }

        connections.push(others);
    }

    let mut group0: Vec<usize> = Vec::new();
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(0);

    while let Some(prog) = queue.pop_front() {
        if ! group0.contains(&prog) {
            for other in connections[prog].iter() {
                queue.push_back(*other);
            }

            group0.push(prog);
        }
    }

    group0.len()
}
