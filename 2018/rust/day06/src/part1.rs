use super::*;

use aoc_utils::grid2d::*;
use std::io;

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
            State::None => print!(" "),
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
    //let mut stdinbuffer = String::new();
    //io::stdin().read_line(&mut stdinbuffer).unwrap();

    //let mut grid: HashMap<Position, usize> = HashMap::new();
    let mut grid: InfiniteGrid<State> = InfiniteGrid::new();
    let starts = input.clone();
    let mut counts = Vec::new();

    for (idx, start) in input.into_iter().enumerate() {
        grid.set_value(start, State::Nearest(idx, 0));
        counts.push(0);
    }

    let [ _, loc_max, loc_min, _ ] = grid.get_boundaries();

    for yy in loc_min.yy() ..= loc_max.yy() {
        for xx in loc_min.xx() ..= loc_max.xx() {
            let loc = Location2D::new(xx, yy);
            let distances: Vec<_> = starts
                .iter()
                .map(|start| loc.distance(start))
                .collect();

            let min = distances.iter().min().unwrap();
            let occurences = distances.iter().filter(|item| *item == min).count();
            match occurences {
                0 => panic!("0 minimal distances oO"),
                1 => {
                    let nearest = distances.iter().position(|xx| *xx == *min).unwrap();
                    grid.set_value(
                        &loc,
                        State::Nearest(
                            nearest,
                            *min,
                        )
                    );
                    counts[nearest] += 1;
                }
                _ => {
                    grid.set_value(&loc, State::Conflicted);
                }
            }
        }
    }

    for xx in loc_min.xx() ..= loc_max.xx() {
        let loc = Location2D::new(xx, loc_min.yy());
        if let Some(State::Nearest(nearest, _)) = grid.get_value(&loc) {
            counts[*nearest] = 0;
        }

        let loc = Location2D::new(xx, loc_max.yy());
        if let Some(State::Nearest(nearest, _)) = grid.get_value(&loc) {
            counts[*nearest] = 0;
        }
    }

    for yy in loc_min.yy() ..= loc_max.yy() {
        let loc = Location2D::new(loc_min.xx(), yy);
        if let Some(State::Nearest(nearest, _)) = grid.get_value(&loc) {
            counts[*nearest] = 0;
        }

        let loc = Location2D::new(loc_max.xx(), yy);
        if let Some(State::Nearest(nearest, _)) = grid.get_value(&loc) {
            counts[*nearest] = 0;
        }
    }

    //fancy_print(&grid);

    *counts.iter().max().unwrap()
}

#[allow(dead_code)]
pub fn fancy_print(grid: &InfiniteGrid<State>) {
    if grid.iter().count() == 0 {
        println!("Grid is empty!")
    } else {
        let [
            _,
            loc_max,
            loc_min,
            _
        ] = grid.get_boundaries();

        for yy in loc_min.yy() ..= loc_max.yy() {
            for xx in loc_min.xx() ..= loc_max.xx() {
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
        let (input, _) = parse_input(name, false);
        solve(&input)
    }

    #[test]
    fn examples() {
        assert_eq!(solve_example("example1"), 17);
    }
}
