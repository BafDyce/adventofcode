use super::*;

pub type OutputType = usize;

pub fn solve(points: &InputType, _config: &PuzzleConfig) -> OutputType {
    let mut iter = points.into_iter();

    let mut constellations = Vec::new();
    constellations.push(Constellation::new(*iter.next().unwrap()));

    for point in iter {
        let mut matches = Vec::new();
        for (ii, cons) in constellations.iter().enumerate() {
            if cons.belongs_to(point) {
                matches.push(ii);
            }
        }

        match matches.len() {
            0 => {
                constellations.push(Constellation::new(*point));
            }
            1 => {
                constellations[matches[0]].add(*point);
            }
            _ => {
                constellations[matches[0]].add(*point);
                // going in reverse, because remove() will shift elements.
                for ii in matches[1 ..].into_iter().rev() {
                    let cons = constellations.remove(*ii);
                    constellations[matches[0]].merge(cons);
                }
            }
        }
    }

    constellations.len()
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
        assert_eq!(solve_example("example1"), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(solve_example("example2"), 4);
    }

    #[test]
    fn example_3() {
        assert_eq!(solve_example("example3"), 3);
    }

    #[test]
    fn example_4() {
        assert_eq!(solve_example("example4"), 8);
    }
}
