#![feature(drain_filter)]
extern crate aocutils;
extern crate regex;

mod part1;
mod part2;

use std::cmp::Ordering;
use regex::Regex;

#[derive(Debug,Clone)]
pub struct Particle {
    position:       Coordinate,
    velocity:       Coordinate,
    acceleration:   Coordinate,
}

impl Particle {
    fn get_pos_at(&self, time: i64) -> Coordinate {
        let xx = self.position.xx
            + time * self.velocity.xx
            + (time * (time + 1) * self.acceleration.xx) / 2;

        let yy = self.position.yy
            + time * self.velocity.yy
            + (time * (time + 1) * self.acceleration.yy) / 2;

        let zz = self.position.zz
            + time * self.velocity.zz
            + (time * (time + 1) * self.acceleration.zz) / 2;

        Coordinate {xx: xx, yy: yy, zz: zz}
    }

    fn update(&mut self) {
        self.velocity.xx += self.acceleration.xx;
        self.velocity.yy += self.acceleration.yy;
        self.velocity.zz += self.acceleration.zz;

        self.position.xx += self.velocity.xx;
        self.position.yy += self.velocity.yy;
        self.position.zz += self.velocity.zz;
    }
}

impl Ord for Particle {
    fn cmp(&self, other: &Particle) -> Ordering {
        match self.position.xx.cmp(&other.position.xx) {
            Ordering::Less      => Ordering::Less,
            Ordering::Greater   => Ordering::Greater,
            Ordering::Equal     => match self.position.yy.cmp(&other.position.yy) {
                Ordering::Less      => Ordering::Less,
                Ordering::Greater   => Ordering::Greater,
                Ordering::Equal     => match self.position.zz.cmp(&other.position.zz) {
                    Ordering::Less      => Ordering::Less,
                    Ordering::Greater   => Ordering::Greater,
                    Ordering::Equal     =>  Ordering::Equal,
                }
            }
        }
    }
}

impl PartialOrd for Particle {
    fn partial_cmp(&self, other: &Particle) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Particle {
    fn eq(&self, other: &Particle) -> bool {
        self.position.xx == other.position.xx
        && self.position.yy == other.position.yy
        && self.position.zz == other.position.zz
    }
}

impl Eq for Particle {}

#[derive(Debug,Clone,Eq,PartialEq)]
pub struct Coordinate {
    xx: i64,
    yy: i64,
    zz: i64,
}

fn main() {
    let day: i32 = 20;

    let input = aocutils::import(day, Some("puzzle1"));

    let mut particles: Vec<Particle> = Vec::with_capacity(input.len());

    let re = Regex::new(r"(?x)
        p=<
        (?P<pxx>[-\d]+)
        ,
        (?P<pyy>[-\d]+)
        ,
        (?P<pzz>[-\d]+)
        >,\x20v=<
        (?P<vxx>[-\d]+)
        ,
        (?P<vyy>[-\d]+)
        ,
        (?P<vzz>[-\d]+)
        >,\x20a=<
        (?P<axx>[-\d]+)
        ,
        (?P<ayy>[-\d]+)
        ,
        (?P<azz>[-\d]+)
        >
    ").unwrap();
    for line in input {
        let things = re.captures(&line).unwrap();

        let pxx = things["pxx"].parse::<i64>().unwrap();
        let pyy = things["pyy"].parse::<i64>().unwrap();
        let pzz = things["pzz"].parse::<i64>().unwrap();

        let vxx = things["vxx"].parse::<i64>().unwrap();
        let vyy = things["vyy"].parse::<i64>().unwrap();
        let vzz = things["vzz"].parse::<i64>().unwrap();

        let axx = things["axx"].parse::<i64>().unwrap();
        let ayy = things["ayy"].parse::<i64>().unwrap();
        let azz = things["azz"].parse::<i64>().unwrap();

        particles.push(
            Particle {
                position: Coordinate {xx: pxx, yy: pyy, zz: pzz},
                velocity: Coordinate {xx: vxx, yy: vyy, zz: vzz},
                acceleration: Coordinate {xx: axx, yy: ayy, zz: azz},
            }
        );
    }

    //println!("{:?}", particles);

    let aa = Particle {
        position: Coordinate {xx: 0, yy: 0, zz: 1},
        velocity: Coordinate {xx: 1, yy: 2, zz: 3},
        acceleration: Coordinate {xx: 4, yy: 5, zz: 6},
    };
    let bb = Particle {
        position: Coordinate {xx: 1, yy: 0, zz: 0},
        velocity: Coordinate {xx: 7, yy: 8, zz: 9},
        acceleration: Coordinate {xx: 0, yy: 1, zz: 2},
    };
    println!("aa == bb: {:?}", aa == bb);
    println!("aa < bb: {:?}", aa < bb);
    println!("aa > bb: {:?}", aa > bb);

    let res1 = part1::solve(&particles);
    let res2 = part2::solve(&particles);

    println!("Results: {} and {}", res1, res2);
}
