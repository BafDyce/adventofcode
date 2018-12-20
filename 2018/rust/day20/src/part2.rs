use super::*;

use std::collections::HashMap;

use super::part1::OutputType;

pub fn solve(input: &InputType, _config: &PuzzleConfig) -> OutputType {
    let start = Location2D::new(0, 0);
    let mut iter = input.chars();
    // remove ^
    iter.next().unwrap();

    let mut map: HashMap<Location2D, Room> = HashMap::new();
    let mut pos = start.to_owned();

    fill(&mut iter, &mut pos, &mut map);
    calc_distances(&mut map);

    map.iter().filter(|(_, room)| {
        room.distance >= 1000
    }).count()
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
