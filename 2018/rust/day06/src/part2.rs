use super::*;

use super::part1::State;
type OutputType = super::part1::OutputType;

pub fn solve(input: &InputType, config: &PuzzleConfig) -> OutputType {
    let threshold = config.get("threshold").unwrap_or(&"10000".to_owned()).parse::<i64>().unwrap();

    let mut grid: InfiniteGrid<State> = InfiniteGrid::new();
    let starts = input.clone();
    let mut count = 0;

    for (idx, start) in input.into_iter().enumerate() {
        grid.set_value(start, State::Nearest(idx, 0));
    }

    let [ _, loc_max, loc_min, _ ] = grid.get_boundaries();

    for yy in loc_min.yy() ..= loc_max.yy() {
        for xx in loc_min.xx() ..= loc_max.xx() {
            let loc = Location2D::new(xx, yy);
            if starts.iter().map(|start| loc.distance(start)).sum::<i64>() < threshold {
                count += 1;
            }
        }
    }

    count
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
        assert_eq!(solve_example("example1"), 0);
    }
}
