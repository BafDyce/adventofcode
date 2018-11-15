use super::*;

pub fn solve(rules: &[ExpansionRule]) -> u64 {

    let mut grid = Grid::new();

    for _ in 1..=18 {
        grid.expand_with_rules(rules);
    }

    grid.count_on()
}
