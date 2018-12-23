use super::*;

use md5::{Md5, Digest};
// use sha1::{Sha1, Digest}; // just in case
use regex::Regex;
use std::collections::HashMap;

pub type OutputType = usize;

#[derive(Clone, Copy, Debug)]
pub enum Ctype {
    Rocky,
    Wet,
    Narrow,
}

#[derive(Clone, Copy, Debug)]
pub struct Field {
    pub erosion_level: usize,
    pub ctype: Ctype,
}

#[derive(Clone, Debug)]
pub struct Cave {
    pub depth: usize,
    pub target: (usize, usize),
    pub fields: HashMap<(usize, usize), Field>,
}

//pub fn calc_field(depth: usize, target: Location2D, xx: usize, yy: usize, x1y: Field, xy1: Field) -> Field {
pub fn gen_field(cave: &mut Cave, xx: usize, yy: usize) {
    let geologic_index = if ( xx == 0 && yy == 0 ) || ( xx == cave.target.0 && yy == cave.target.1) {
        0
    } else if yy == 0 {
        xx * 16807
    } else if xx == 0 {
        yy * 48271
    } else {
        cave.fields.get(&(xx-1, yy)).unwrap().erosion_level * cave.fields.get(&(xx, yy-1)).unwrap().erosion_level
    };

    let erosion_level = (geologic_index + cave.depth) % 20183;
    let field = Field {
        erosion_level: erosion_level,
        ctype: match erosion_level % 3 {
            0 => Ctype::Rocky,
            1 => Ctype::Wet,
            2 => Ctype::Narrow,
            _ => panic!(" modulo 3 > 2 oO"),
        }
    };

    cave.fields.insert((xx, yy), field);
}

pub fn solve(input: &InputType, config: &PuzzleConfig) -> OutputType {
    let mut cave = Cave {
        depth: input.depth,
        target: (input.target.xx_as_usize(), input.target.yy_as_usize() ),
        fields: HashMap::new(),
    };


    let mut risk = 0;
    for xx in 0 ..= cave.target.0 {
        for yy in 0 ..= cave.target.1 {
            gen_field(&mut cave, xx, yy);
            risk += match cave.fields.get(&(xx, yy)).unwrap().ctype {
                Ctype::Rocky => 0,
                Ctype::Wet => 1,
                Ctype::Narrow => 2,
            }
        }
    }

    risk
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
