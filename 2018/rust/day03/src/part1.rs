use super::*;

pub type OutputType = usize;

pub fn solve(input: &InputType) -> OutputType {
    let mut grid = vec![vec![0usize; 1000]; 1000];

    for claim in input {
        for ii in claim.left..claim.left+claim.width {
            for jj in claim.top..claim.top + claim.height {
                grid[ii][jj] += 1;
            }
        }
    }

    let mut counter = 0;
    for row in grid.iter() {
        for cell in row.iter() {
            if *cell > 1 {
                counter += 1;
            }
        }
    }

    counter
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
        assert_eq!(solve_example("example1"), 4);
    }
}
