use super::*;

type OutputType = (usize, usize, usize);

pub fn solve(input: &InputType, _config: &PuzzleConfig) -> OutputType {
    let grid = part1::create_powergrid(*input);

    let mut max = 0;
    let mut coords = (0, 0, 0);
    for size in 1 .. grid.len() {
        for ii in 0 .. grid.len() - size {
            for jj in  0 .. grid.len() - size {
                let mut power = 0;
                for xx in 0 .. size {
                    for yy in 0 .. size {
                        power += grid[ii + xx][jj + yy];
                    }
                }

                if power > max {
                    max = power;
                    coords = (ii+1, jj+1, size);
                }
            }
        }
    }

    coords
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solve_example(name: &str) -> OutputType {
        let (input, config) = parse_input(name, false);
        solve(&input, &config)
    }

    #[test]
    fn example_1() {
        assert_eq!(solve_example("example1"), (90, 269, 16));
    }

    #[test]
    fn example_2() {
        assert_eq!(solve_example("example2"), (232, 251, 12));
    }
}
