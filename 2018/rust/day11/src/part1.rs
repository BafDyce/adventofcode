use super::*;

pub type OutputType = (usize, usize);

// While solving, I just had this in the solve function and copied it over to part 2
fn create_powergrid(serial: InputType) -> [[isize; 300]; 300] {
    let mut grid = [[0isize; 300]; 300];

    for ii in 0 .. grid.len() {
        for jj in 0 .. grid[ii].len() {
            grid[ii][jj] = compute_power_level(ii as isize + 1, jj as isize + 1, serial);
        }
    }

    grid
}

// In the original solution, this was also not an extra function (even though I wished because it
// would have made testing easier ;-)
pub fn compute_power_level(xx: isize, yy: isize, serial: isize) -> isize {
    let id: isize = xx + 10;
    let power = (id * yy + serial) * id;
    (power % 1000) / 100 - 5
}

pub fn solve(input: &InputType, _config: &PuzzleConfig) -> OutputType {
    let grid = create_powergrid(*input);

    let mut max = 0;
    let mut coords = (0, 0);
    for ii in 0 .. grid.len() - 2 {
        for jj in 0 .. grid[ii].len() - 2 {
            let power = grid[ii][jj] + grid[ii][jj + 1] + grid[ii][jj + 2]
            + grid[ii + 1][jj] + grid[ii + 1][jj + 1] + grid[ii + 1][jj + 2]
            + grid[ii + 2][jj] + grid[ii + 2][jj + 1] + grid[ii + 2][jj + 2];

            if power > max {
                max = power;
                coords = (ii+1, jj+1);
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
    fn power_lvl_1() {
        assert_eq!(compute_power_level(122, 79, 57), -5);
    }

    #[test]
    fn power_lvl_2() {
        assert_eq!(compute_power_level(217, 196, 39), 0);
    }

    #[test]
    fn power_lvl_3() {
        assert_eq!(compute_power_level(101, 153, 71), 4);
    }

    #[test]
    fn example_1() {
        assert_eq!(solve_example("example1"), (33, 45));
    }

    #[test]
    fn example_2() {
        assert_eq!(solve_example("example2"), (21, 61));
    }
}
