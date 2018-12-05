extern crate aoc_utils;
#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate md5;
extern crate sha1;

mod part1;
mod part2;

use aoc_utils::prelude::*;
use regex::Regex;
use std::{collections::HashMap, env};

const DAY: u32 = 4;
type ParsingTypeSingle = Event;
type ParsingType = Vec<ParsingTypeSingle>;

type InputType = HashMap<usize, Guard>;

#[derive(Debug)]
pub struct Event {
    pub month: usize,
    pub day: usize,
    pub minute: usize,
    pub etype: EventType,
}

#[derive(Debug)]
pub enum EventType {
    BeginShift(usize),
    WakeUp,
    FallAsleep,
}

#[derive(Debug, Clone)]
pub struct Guard {
    id: usize,
    state: GuardState,
    minutes: HashMap<usize, usize>,
    fallen_asleep: Option<usize>,
}

impl Guard {
    pub fn new(id: usize) -> Guard {
        Guard {
            id,
            state: GuardState::Awake,
            minutes: HashMap::new(),
            fallen_asleep: None,
        }
    }

    pub fn fall_asleep(&mut self, ts: usize) {
        if self.state != GuardState::Asleep {
            self.fallen_asleep = Some(ts);
            self.state = GuardState::Asleep;
        }
    }

    pub fn wake_up(&mut self, ts: usize) {
        if self.state != GuardState::Awake {
            if let Some(minute) = self.fallen_asleep {
                for ii in minute .. ts {
                    *self.minutes.entry(ii).or_insert(0) += 1;
                }
            }
        }

        self.fallen_asleep = None;
        self.state = GuardState::Awake;
    }

    pub fn begin_shift(&mut self) {
        self.end_shift();
        self.state = GuardState::Awake;
        self.fallen_asleep = None;
    }

    pub fn end_shift(&mut self) {
        self.wake_up(60)
    }

    pub fn get_sleeptime_total(&self) -> usize {
        self.minutes.iter().fold(0, |acc, xx| acc + xx.1)
    }

    pub fn get_max_sleepminute(&self) -> usize {
        let mut max_kk = 0;
        let mut max_vv = 0;
        for (kk, vv) in self.minutes.iter() {
            if *vv > max_vv {
                max_vv = *vv;
                max_kk = *kk;
            }
        }

        max_kk.clone()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum GuardState {
    Awake,
    Asleep,
}

fn main() {
    // READ input
    let args: Vec<String> = env::args().collect();

    // Parse command line arguments
    let input_name = if args.len() > 1 {
        &args[1]
    } else {
        "puzzle1"
    };
    let verbose = args.contains(&String::from("-v")) || args.contains(&String::from("--verbose"));

    if verbose {
        println!("Loading data from input file {}", input_name);
    }
    // READ & PARSE input
    let input = parse_input(input_name, verbose);

    // PRECOMPUTATION
    let guards = precompute(input);

    // SOLVE puzzles
    let res1 = part1::solve(&guards);
    let res2 = part2::solve(&guards);

    println!("results: {} and {}", res1, res2);
}

fn parse_input(input_name: &str, verbose: bool) -> ParsingType {
    let config = ImportConfig::new(2018, DAY, "../../_inputs/day{day}/");
    let mut input = import(&config, input_name).unwrap();
    if verbose {
        println!("raw input: {:?}", input);
    }

    // PARSE input
    input.sort();
    let data: ParsingType = input.into_iter().filter_map(|line| {
        // Parsing logic
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"\[1518-(?P<month>\d+)-(?P<day>\d+) \d+:(?P<minute>\d\d)\] (Guard \#(?P<id>\d+) begins shift|(?P<wakesup>wakes up)|(?P<sleep>falls asleep))"
            ).unwrap();
        }

        if let Some(caps) = RE.captures(&line) {
            let month = caps["month"].parse::<usize>().unwrap();
            let day = caps["day"].parse::<usize>().unwrap();
            let minute = caps["minute"].parse::<usize>().unwrap();

            let etype = if let Some(id) = caps.name("id") {
                EventType::BeginShift(id.as_str().parse::<usize>().unwrap())
            } else if let Some(_) = caps.name("wakesup") {
                EventType::WakeUp
            } else if let Some(_) = caps.name("sleep") {
                EventType::FallAsleep
            } else {
                panic!("should not happen");
            };

            Some(Event {
                month,
                day,
                minute,
                etype
            })
        } else {
            None
        }

    })
    .collect();

    if verbose {
        println!("input parsed: {:?}", data);
    }
    data
}

fn precompute(input: ParsingType) -> InputType {
    let mut guards: HashMap<usize, Guard> = HashMap::new();
    let mut active_guard = None;

    for event in input {
        match event.etype {
            EventType::BeginShift(id) => {
                if let Some(active_guard) = active_guard {
                    let guard = guards.entry(active_guard).or_insert(Guard::new(active_guard));
                    guard.end_shift();
                }
                active_guard = Some(id);
                let guard = guards.entry(active_guard.unwrap()).or_insert(Guard::new(active_guard.unwrap()));
                guard.begin_shift();
            },
            EventType::WakeUp => {
                let guard = guards.entry(active_guard.unwrap()).or_insert(Guard::new(active_guard.unwrap()));
                guard.wake_up(event.minute);
            },
            EventType::FallAsleep => {
                let guard = guards.entry(active_guard.unwrap()).or_insert(Guard::new(active_guard.unwrap()));
                guard.fall_asleep(event.minute);
            }
        }

    }

    {
        let guard = guards.entry(active_guard.unwrap()).or_insert(Guard::new(active_guard.unwrap()));
        guard.end_shift();
    }

    guards
}
