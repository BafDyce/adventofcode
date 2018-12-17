use super::*;

use md5::{Md5, Digest};
// use sha1::{Sha1, Digest}; // just in case
use regex::Regex;
use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use std::cell::RefCell;

pub type OutputType = usize;

fn set_water(old: &Element) -> Element {
    if let Element::Sand = old {
        Element::WaterFlowing
    } else {
        *old
    }
}

pub fn solve(input: &InputType, config: &PuzzleConfig) -> OutputType {
    let mut grid = input.clone();

    let mut start = Location2D::new(500, 1);
    let mut queue = VecDeque::new();
    queue.push_front(start);

    while !queue.is_empty() {
        let next = queue.pop_front().unwrap();

        {
            let element = grid.get_value_ref(&next);
            match element {
                Element::Sand => *element = Element::WaterFlowing,
                Element::WaterFlowing => *element = Element::WaterStill,
                _ => {}
            }
        }

        //grid.set_value(&next, Element::WaterFlowing);

        {
            let down = Location2D::new(next.xx(), next.yy() + 1);
            let move_down = match grid.get_value(&down) {
                None | Some(Element::Sand) => true,
                _ => false
            };

            if move_down {
                queue.push_back(down);
                continue;
            }
        }

        // we cannot move down, so we might be able to move left/right
        let left = Location2D::new(next.xx() - 1, next.yy());
        let right = Location2D::new(next.xx() + 1, next.yy());
        let move_left = match grid.get_value(&left) {
            None | Some(Element::Sand) => true,
            _ => false
        };

        if move_left {
            queue.push_back(left);
        }

        let move_right = match grid.get_value(&right) {
            None | Some(Element::Sand) => true,
            _ => false
        };

        if move_right {
            queue.push_back(right);
        }

        if !move_right && !move_left {
            // we reached a dead end -> do magic
        }
    }


    fancy_print(&grid);

    /*if false {
        let grid_container = RefCell::new(Mutex::new(&mut grid as &mut Grid<Element>));

        let mut water: InfiniteGridWalker<Element> = InfiniteGridWalker::new_with_particle(Particle2D::new(500, 1, Direction2D::Right));
        water.assign_grid(&grid_container);

        water.operate(set_water);
        water.step_forward();
        water.operate(set_water);
        water.step_forward();
        water.operate(set_water);
        water.step_forward();
        water.operate(set_water);
        water.step_forward();
        water.operate(set_water);
        water.step_forward();
        water.operate(set_water);
        water.step_forward();
        water.operate(set_water);
        water.step_forward();
        water.operate(set_water);
    }


    let mut pos = Location2D::new(500, 1);
    {
        println!("pos: {:?}", pos);
        // flow down
        'flow_down: loop {
            let value_ref = grid.get_value_ref(&pos);
            match value_ref {
                Element::Sand => *value_ref = Element::WaterFlowing,
                Element::Clay => break 'flow_down,
                _ => {}
            }

            *(pos.yy_mut()) += 1;
        }

        *(pos.yy_mut()) -= 1;
        let pos_left =
        'flow_down: loop {
            let value_ref = grid.get_value_ref(&pos);
            match value_ref {
                Element::Sand => *value_ref = Element::WaterFlowing,
                Element::Clay => break 'flow_down,
                _ => {}
            }

            *(pos.yy_mut()) += 1;
        }

        fancy_print(&grid);
        //enter_to_continue();
    }*/

    OutputType::default()
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
