#![feature(vec_remove_item)]
extern crate aocutils;
#[macro_use]
extern crate lazy_static;
extern crate regex;

mod part1;
mod part2;

use regex::Regex;


struct Bridge {
    last_port: usize,
    strength: usize,
    length: usize,
}

#[derive(Clone, Debug, PartialEq)]
struct Component {
    ports: [usize; 2],
}

impl Component {
    pub fn parse(spec: &str) -> Result<Component, ()> {
        lazy_static!{
            static ref RE: Regex = Regex::new(
                r"(?P<aa>[0-9]+)/(?P<bb>[0-9]+)"
            ).unwrap();
        }

        let caps = RE.captures(spec).unwrap();
        match (caps.name("aa"), caps.name("bb")) {
            (Some(aa), Some(bb)) => {
                Ok(Component {
                    ports: {
                        let mut ports = [
                            aa.as_str().parse().unwrap(),
                            bb.as_str().parse().unwrap(),
                        ];
                        ports.sort();
                        ports
                    }
                })
            }
            _ => Err(())
        }
    }

    pub fn has_port(&self, port: &usize) -> bool {
        self.ports.contains(port)
    }

    pub fn weight(&self) -> usize {
        self.ports[0] + self.ports[1]
    }
}

fn main() {
    let day: i32 = 24;

    let components = aocutils::import(day, Some("puzzle1"))
        .iter()
        .filter_map(|line| Component::parse(line).ok())
        .collect::<Vec<Component>>();

    let res1 = part1::solve(&components);
    let res2 = part2::solve(&components);

    println!("Results: {} and {}", res1, res2);
}
