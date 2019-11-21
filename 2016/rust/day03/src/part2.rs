use super::*;

type OutputType = super::part1::OutputType;

pub fn solve(input: &InputType, config: &PuzzleConfig) -> OutputType {
    input.chunks(3).map(|chunks| {
        let Triangle { aa: aa1, bb: aa2, cc: aa3 } = chunks[0];
        let Triangle { aa: bb1, bb: bb2, cc: bb3 } = chunks[1];
        let Triangle { aa: cc1, bb: cc2, cc: cc3 } = chunks[2];

        let mut valid_triangles = 0;
        if Triangle::new(aa1, bb1, cc1).is_valid() {
            valid_triangles += 1;
        }
        if Triangle::new(aa2, bb2, cc2).is_valid() {
            valid_triangles += 1;
        }
        if Triangle::new(aa3, bb3, cc3).is_valid() {
            valid_triangles += 1;
        }

        valid_triangles
    }).sum()
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
