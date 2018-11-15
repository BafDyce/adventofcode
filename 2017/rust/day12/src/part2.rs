use super::*;

use regex::Regex;
use std::collections::HashMap;
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

        connections.insert(src, others);
    }

    let mut processed: Vec<usize> = Vec::new();
    let mut groups: Vec<Vec<usize>> = Vec::new();

    let mut ii = 0;
    loop {
        processed.sort();
        let center = {
            let mut iter = processed.iter().enumerate();
            loop {
                if let Some((idx, &val)) = iter.next() {
                    if idx != val {
                        break idx;
                    }
                } else {
                    break processed.len();
                }
            }
        };

        if center == connections.len() {
            break;
        }

        let mut group: Vec<usize> = Vec::new();
        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.push_back(center);

        while let Some(prog) = queue.pop_front() {
            if ! group.contains(&prog) {
                for other in connections[prog].iter() {
                    queue.push_back(*other);
                }

                group.push(prog);
                processed.push(prog);
            }
        }

        groups.push(group);
    }

    groups.len()
}
