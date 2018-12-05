use super::*;

type OutputType = String;

pub fn solve(input: &InputType) -> OutputType {
    // Assumes that it only receives good id candidates (filtered by logic from part 1)

    for (idx, id) in input.iter().enumerate() {
        for id2 in input.iter().skip(idx+1) {
            if id == id2 {
                continue;
            }

            let mut diffs = Vec::new();
            for (idx, (aa, bb)) in id.chars().zip(id2.chars()).enumerate() {
                if aa != bb {
                    diffs.push(idx);
                }
            }

            if diffs.len() == 1 {
                let mut idcopy: String = id.to_owned();
                idcopy.remove(diffs[0]);
                return idcopy;
            }
        }
    }

    String::from("ERROR! NO VALID IDS FOUND!!!")
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
        assert_eq!(solve_example("example2"), "fgij");
    }
}
