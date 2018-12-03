use super::*;

use super::part1::OutputType;

pub fn solve(input: &InputType) -> OutputType {
    let mut grid = vec![vec![0usize; 1000]; 1000];

    let mut broken = Vec::new();
    for claim in input.iter() {
        for ii in claim.left..claim.left+claim.width {
            for jj in claim.top..claim.top + claim.height {
                if grid[ii][jj] != 0 {
                    broken.push(grid[ii][jj]);
                    broken.push(claim.id);
                }
                grid[ii][jj] = claim.id;
            }
        }
    }

    broken.dedup();
    for claim in input {
        if ! broken.contains(&claim.id) {
            return claim.id;
        }
    }

    0
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
        assert_eq!(solve_example("example1"), 3);
    }
}
