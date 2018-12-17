use super::*;

use md5::{Md5, Digest};
// use sha1::{Sha1, Digest}; // just in case
use regex::Regex;
use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use std::cell::RefCell;

pub type OutputType = usize;

// too high: 31790
// too low: 31763

// 31788
// manually substract the top | chars xD

fn get_min_max_yy(grid: &InputType) -> (isize, isize) {
    let [ _, loc_max, loc_min, _ ] = grid.get_boundaries();
    (loc_min.yy(), loc_max.yy())
}

fn fill(loc: &Location2D, grid: &mut InputType, over_water: bool) -> (/*full?*/bool, /*#filled*/usize) {
    let (min, max) = get_min_max_yy(grid);
    if loc.yy() < min || loc.yy() > max {
        return (false, 0);
    }

    {
        let element = grid.get_value_ref(&loc);
        match element {
            Element::Sand => *element = Element::WaterFlowing,
            Element::Clay | Element::WaterFlowing => return (true, 0),
            _ => {}
        }
    }
    //println!("{:?} | {}", loc, over_water);
    //fancy_print(&grid);
    //enter_to_continue();

    let below = Location2D::new(loc.xx(), loc.yy() + 1);
    let left = Location2D::new(loc.xx() - 1, loc.yy());
    let right = Location2D::new(loc.xx() + 1, loc.yy());

    {
        let ll = grid.get_value(&left).unwrap_or(&Element::Sand);
        let bb = grid.get_value(&below).unwrap_or(&Element::Sand);
        let rr = grid.get_value(&right).unwrap_or(&Element::Sand);
        match (ll, bb, rr) {
            (Element::Sand, Element::WaterFlowing, Element::Sand) => return (false, 1),
            _ => {}
        }
    }
    let (full, filled) = fill(&below, grid, over_water);

    if full {
        let over_water = over_water || filled == 0;
        // expand to left

        let result_left = fill(&left, grid, over_water);

        // expand to right

        let result_right = fill(&right, grid, over_water);

        (result_left.0 && result_right.0, filled + result_left.1 + result_right.1 + 1)
    } else {
        (full, filled + 1)
    }
}

pub fn solve(input: &InputType, config: &PuzzleConfig) -> (OutputType, InputType) {
    let mut grid = input.clone();

    let mut start = Location2D::new(500, 1);
    let (_, result) = fill(&start, &mut grid, false);

    println!("Grid at the end");
    fancy_print(&grid);
    (result, grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solve_example(name: &str) -> OutputType {
        let (input, config) = parse_input(name, false);
        solve(&input, &config).0
    }

    #[test]
    fn examples() {
        assert_eq!(solve_example("example1"), 57);
    }
}
