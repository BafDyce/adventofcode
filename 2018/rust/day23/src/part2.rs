use super::*;

use regex::Regex;
use std::collections::HashMap;

type OutputType = super::part1::OutputType;

pub fn solve(bots: &InputType, config: &PuzzleConfig) -> OutputType {
    let mut xx_min = 0;
    let mut xx_max = 0;
    let mut yy_min = 0;
    let mut yy_max = 0;
    let mut zz_min = 0;
    let mut zz_max = 0;

    for bot in bots {
        xx_min = isize::min(xx_min, bot.xx());
        xx_max = isize::max(xx_max, bot.xx());

        yy_min = isize::min(yy_min, bot.yy());
        yy_max = isize::max(yy_max, bot.yy());

        zz_min = isize::min(zz_min, bot.zz());
        zz_max = isize::max(zz_max, bot.zz());
    }

    println!("xx: {} / {} / {}", xx_min, xx_max, xx_max - xx_min);
    println!("yy: {} / {} / {}", yy_min, yy_max, yy_max - yy_min);
    println!("zz: {} / {} / {}", zz_min, zz_max, zz_max - zz_min);

    OutputType::default()
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
