use super::*;
use std::collections::HashMap;

pub type OutputType = usize;

pub fn solve(input: &InputType, _config: &PuzzleConfig) -> OutputType {
    let start = Location2D::new(0, 0);
    let mut iter = input.chars();
    // remove ^
    iter.next().unwrap();

    let mut map: HashMap<Location2D, Room> = HashMap::new();
    let mut pos = start.to_owned();

    fill(&mut iter, &mut pos, &mut map);
    let furthest = calc_distances(&mut map);

    //println!("{:?}", map);
    //fancy_print(&map, true);
    //println!("Furthest away: {:?}", furthest);

    furthest.1.distance
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
        assert_eq!(solve_example("example1"), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(solve_example("example2"), 10);
    }

    #[test]
    fn example_3() {
        assert_eq!(solve_example("example3"), 18);
    }

    #[test]
    fn example_4() {
        assert_eq!(solve_example("example4"), 23);
    }

    #[test]
    fn example_5() {
        assert_eq!(solve_example("example5"), 31);
    }
}
