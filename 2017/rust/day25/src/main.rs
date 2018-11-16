extern crate aocutils;
extern crate regex;

mod part1;
mod part2;

use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Action {
    val: usize,
    direction: i32,
    next_state: char,
}

#[derive(Debug)]
struct State {
    actions: [Action; 2],
}

fn main() {
    let day: i32 = 25;

    let input = aocutils::import(day, Some("puzzle1"));

    let start_state = {
        let regex_start_state = Regex::new(
            r"Begin in state (?P<state>[A-Z])."
        ).unwrap();
        let caps = regex_start_state.captures(&input[0]).unwrap();
        caps["state"].chars().next().unwrap()
    };

    let steps = {
        let regex_checksum_steps = Regex::new(
            r"Perform a diagnostic checksum after (?P<steps>[0-9]+) steps."
        ).unwrap();
        let caps = regex_checksum_steps.captures(&input[1]).unwrap();
        caps["steps"].parse::<usize>().unwrap()
    };

    let regex_state = Regex::new(
        r"In state (?P<state>[A-Z]):
  If the current value is 0:
    - Write the value (?P<if0_write>[0|1]).
    - Move one slot to the (?P<if0_dir>[a-z]+).
    - Continue with state (?P<if0_state>[A-Z]).
  If the current value is 1:
    - Write the value (?P<if1_write>[0|1]).
    - Move one slot to the (?P<if1_dir>[a-z]+).
    - Continue with state (?P<if1_state>[A-Z])."
    ).unwrap();

    let mut states: HashMap<char, State> = HashMap::new();
    regex_state.captures_iter(&input.join("\n"))
        .into_iter()
        .map(|caps| {
            (
                caps.name("state").unwrap().as_str().chars().next().unwrap(),
                State {
                    actions: [
                        Action {
                            val: caps.name("if0_write").unwrap().as_str().parse::<usize>().unwrap(),
                            direction: {
                                if caps.name("if0_dir").unwrap().as_str() == "left" {
                                    -1
                                } else {
                                    1
                                }
                            },
                            next_state: caps.name("if0_state").unwrap().as_str().chars().next().unwrap(),
                        },
                        Action {
                            val: caps.name("if1_write").unwrap().as_str().parse::<usize>().unwrap(),
                            direction: {
                                if caps.name("if1_dir").unwrap().as_str() == "left" {
                                    -1
                                } else {
                                    1
                                }
                            },
                            next_state: caps.name("if1_state").unwrap().as_str().chars().next().unwrap(),
                        }
                    ]
                }
            )
        })
        .collect::<Vec<(char, State)>>()
        .into_iter()
        .for_each(|(state_name, state)| {
            states.insert(state_name, state);
        });

    println!("{:#?}", states);

    let res1 = part1::solve(start_state, steps, &states);
    let res2 = part2::solve(start_state, steps, &states);

    println!("Results: {} and {}", res1, res2);
}
