use super::*;

use aoc_utils::grid2d::*;

use std::sync::{Mutex, Arc};
use std::cell::RefCell;

use md5::{Md5, Digest};
// use sha1::{Sha1, Digest}; // just in case
use regex::Regex;
use std::collections::HashMap;
use std::io;
use std::cmp;

pub type OutputType = isize;

#[derive(Debug, PartialEq)]
pub enum State {
    None,
    Nearest(usize, i64),
    Conflicted,
}

impl State {
    fn print(&self) {
        match self {
            State::None => print!("."),
            State::Conflicted => print!("."),
            State::Nearest(val, _) => print!("{}", val)
        }
    }
}

impl Default for State {
    fn default() -> State {
        State::None
    }
}

pub fn solve(input: &InputType) -> OutputType {
    let mut stdinbuffer = String::new();

    //let mut grid: HashMap<Position, usize> = HashMap::new();
    let mut grid: InfiniteGrid<State> = InfiniteGrid::new();
    let starts = input.clone();

    let mut sizes = Vec::new();
    let mut queue: VecDeque<(usize, Location2D)> = VecDeque::new();
    for (xx, loc) in input.into_iter().enumerate() {
        //queue.push_back((xx, loc.to_owned()));
        grid.set_value(&loc, State::Nearest(xx, loc.distance(&starts[xx])));
        for item in vec![
            (xx, Location2D::new(loc.xx() + 0, loc.yy() + 1)),
            (xx, Location2D::new(loc.xx() + 1, loc.yy() + 1)),
            (xx, Location2D::new(loc.xx() + 1, loc.yy() + 0)),

            (xx, Location2D::new(loc.xx() - 0, loc.yy() - 1)),
            (xx, Location2D::new(loc.xx() - 1, loc.yy() - 1)),
            (xx, Location2D::new(loc.xx() - 1, loc.yy() - 0)),

            (xx, Location2D::new(loc.xx() + 1, loc.yy() - 1)),
            (xx, Location2D::new(loc.xx() - 1, loc.yy() + 1)),
        ].into_iter() {
            queue.push_back(item);
        }
        sizes.push(1);
    }

    //println!("START QUEUE: {:?}", queue);
    //io::stdin().read_line(&mut stdinbuffer).unwrap();

    let mut cnt = 0;
    while cnt < 200_000 && !queue.is_empty() {

        cnt += 1;
        //print!("{} ", cnt);
        let (xx, loc) = queue.pop_front().unwrap();
        //println!("grid before: {:?}", grid);
        //println!("(xx: {}, loc: {:?})", xx, loc);
        //io::stdin().read_line(&mut stdinbuffer).unwrap();
        //fancy_print(&grid);
        //println!("next = {:?}", (&xx, &loc));
        //io::stdin().read_line(&mut stdinbuffer).unwrap();

        let next = {
            let entry = grid.get_value_ref(&loc);
            let (new_entry, nexts) = match entry {
                State::None => {
                    sizes[xx] += 1;
                    (State::Nearest(xx, loc.distance(&starts[xx])), Some(vec![
                        (xx, Location2D::new(loc.xx() + 0, loc.yy() + 1)),
                        (xx, Location2D::new(loc.xx() + 1, loc.yy() + 1)),
                        (xx, Location2D::new(loc.xx() + 1, loc.yy() + 0)),

                        (xx, Location2D::new(loc.xx() - 0, loc.yy() - 1)),
                        (xx, Location2D::new(loc.xx() - 1, loc.yy() - 1)),
                        (xx, Location2D::new(loc.xx() - 1, loc.yy() - 0)),

                        (xx, Location2D::new(loc.xx() + 1, loc.yy() - 1)),
                        (xx, Location2D::new(loc.xx() - 1, loc.yy() + 1)),
                    ]))
                },
                State::Conflicted => {
                    (State::Conflicted, None)
                },
                State::Nearest(ref to, ref dist) if *to == xx => {
                    (State::Nearest(xx, *dist), None)
                },
                State::Nearest(to, dist) => {
                    let mydist = loc.distance(&starts[xx]);
                    if mydist < *dist {
                        sizes[*to] -= 1;
                        sizes[xx] += 1;
                        (State::Nearest(xx, mydist), Some(vec![
                            (xx, Location2D::new(loc.xx() + 0, loc.yy() + 1)),
                            (xx, Location2D::new(loc.xx() + 1, loc.yy() + 1)),
                            (xx, Location2D::new(loc.xx() + 1, loc.yy() + 0)),

                            (xx, Location2D::new(loc.xx() - 0, loc.yy() - 1)),
                            (xx, Location2D::new(loc.xx() - 1, loc.yy() - 1)),
                            (xx, Location2D::new(loc.xx() - 1, loc.yy() - 0)),

                            (xx, Location2D::new(loc.xx() + 1, loc.yy() - 1)),
                            (xx, Location2D::new(loc.xx() - 1, loc.yy() + 1)),
                        ]))
                    } else if mydist == *dist {
                        sizes[*to] -= 1;
                        (State::Conflicted, None)
                    } else {
                        (State::Nearest(*to, *dist), None)
                    }
                }
            };

            *entry = new_entry;
            nexts
        };

        //println!("grid after: {:?}", grid);
        //println!("nexts: {:?}", next);
        //io::stdin().read_line(&mut stdinbuffer).unwrap();


        if let Some(nexts) = next {
            for (xx, next) in nexts {
                add_to_queue(&mut queue, next, xx);
                continue;
                let tmp = grid.get_value_ref(&next);
                if *tmp == State::None {
                    add_to_queue(&mut queue, next, xx);
                }
            }
        }

        //println!("sizes: {:?}", sizes);
        //println!("queue: {:?}", queue);
        //io::stdin().read_line(&mut stdinbuffer).unwrap();
    }

    //fancy_print(&grid);
    sizes.sort();
    println!("{:?}", sizes);

    0
}

pub fn find_in_queue(queue: &VecDeque<(usize, Location2D)>, loc: &Location2D) -> bool {
    queue.iter().any(|(_, ll)| *ll == *loc)
}

pub fn add_to_queue(queue: &mut VecDeque<(usize, Location2D)>, loc: Location2D, val: usize) {
    queue.push_back((val, loc));
    return;

    if !find_in_queue(queue, &loc) {
        queue.push_back((val, loc))
    } else {
        let mut new_queue = VecDeque::new();
        while let Some((xx, next)) = queue.pop_front() {
            if next == loc {
                new_queue.push_back((xx, next))
            }
        }

        *queue = new_queue;
    }
}

fn fancy_print(grid: &InfiniteGrid<State>) {
    if grid.iter().count() == 0 {
        println!("Grid is empty!")
    } else {
        let mut xx_max = i64::min_value();
        let mut xx_min = i64::max_value();
        let mut yy_max = i64::min_value();
        let mut yy_min = i64::max_value();

        for (pos, _) in grid.iter() {
            xx_max = cmp::max(xx_max, pos.xx());
            xx_min = cmp::min(xx_min, pos.xx());
            yy_max = cmp::max(yy_max, pos.yy());
            yy_min = cmp::min(yy_min, pos.yy());
        }

        println!("{:?} | {:?} | {:?} | {:?}", xx_max, xx_min, yy_max, yy_min);
        for yy in yy_min ..= yy_max {
            for xx in xx_min ..= xx_max {
                match grid.get_value(&Location2D::new(xx, yy)) {
                    None => print!("_"),
                    Some(state) => state.print()
                }
            }
            println!()
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn solve_example(name: &str) -> OutputType {
        let input = parse_input(name, false);
        solve(&input)
    }

    #[test]
    fn examples() {
        assert_eq!(solve_example("example1"), 1);
    }
}
