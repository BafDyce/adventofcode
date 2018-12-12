use super::*;

type OutputType = (usize, usize, usize);

// We use a Summed-area table [1] here. For part 1 this will probably be slower? but for part 2
// its definitely faster.
// NOTE: Original solution was a straightforward O(n^5) solution, which ran in ~40 seconds.
// This one runs in ~24 MILLIseconds
fn create_powergrid(serial: InputType) -> [[isize; 300]; 300] {
    let mut grid = [[0isize; 300]; 300];

    grid[0][0] = part1::compute_power_level(1, 1, serial);
    for ii in 1 .. grid.len() {
        grid[0][ii] = grid[0][ii - 1] + part1::compute_power_level(1, ii as isize + 1, serial);
        grid[ii][0] = grid[ii - 1][0] + part1::compute_power_level(ii as isize + 1, 1, serial);
    }

    for ii in 1 .. grid.len() {
        for jj in 1 .. grid[ii].len() {
            grid[ii][jj] = part1::compute_power_level(ii as isize + 1, jj as isize + 1, serial)
                + grid[ii - 1][jj    ]
                + grid[ii    ][jj - 1]
                - grid[ii - 1][jj - 1];
        }
    }

    grid
}

pub fn solve(input: &InputType, _config: &PuzzleConfig) -> OutputType {
    let grid = create_powergrid(*input);

    let mut max = 0;
    let mut coords = (0, 0, 0);
    for size in 1 .. grid.len() {
        for ii in 0 .. grid.len() - size {
            for jj in  0 .. grid.len() - size {
                let aa = grid[ii        ][jj        ];
                let bb = grid[ii + size ][jj        ];
                let cc = grid[ii        ][jj + size ];
                let dd = grid[ii + size ][jj + size ];

                let power = aa + dd - bb - cc;

                if power > max {
                    max = power;
                    // don't know where this off by one error comes from (one +1 comes from the
                    // coords starting at 1,1 for the puzzle).
                    coords = (ii+2, jj+2, size);
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
