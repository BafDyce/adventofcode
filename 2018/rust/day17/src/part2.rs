use super::*;

use regex::Regex;
use std::collections::HashMap;

type OutputType = super::part1::OutputType;
//type OutputType = i64; // <-- IF part 2 needs a different output

pub fn solve(grid: InputType, config: &PuzzleConfig) -> OutputType {

    let [
        _,
        loc_max,
        loc_min,
        _
    ] = grid.get_boundaries();

    let mut count_total = 0;

    for yy in loc_min.yy() ..= loc_max.yy() {
        let mut count_row = 0;
        let mut counting = false;
        let mut last = Element::Sand;

        for xx in loc_min.xx() ..= loc_max.xx() {
            match grid.get_value(&Location2D::new(xx, yy)) {
                None | Some(Element::Sand) => {
                    // reset counter
                    count_row = 0;
                    counting = false;

                    last = Element::Sand;
                },
                Some(Element::WaterFlowing) => {
                    if last == Element::Clay || counting {
                        counting = true;
                        count_row += 1;
                    }

                    last = Element::WaterFlowing;
                }
                Some(Element::Clay) => {
                    if counting {
                        count_total += count_row;
                        counting = false;
                        count_row = 0;
                    }

                    last = Element::Clay;
                }
                _ => {
                    last = Element::Sand;
                }
            }
        }
    }

    count_total
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
