use super::*;

use regex::Regex;
use std::collections::HashMap;

type OutputType = super::part1::OutputType;

pub fn solve(bots_original: &InputType, config: &PuzzleConfig) -> OutputType {

    let mut precision = 1_000_000;
    let step = 10;

    let mut xx_min = 0;
    let mut xx_max = 0;
    let mut yy_min = 0;
    let mut yy_max = 0;
    let mut zz_min = 0;
    let mut zz_max = 0;

    let mut bots: InputType = bots_original.iter().map(|bot| bot.reduce(precision)).collect();
    for bot in bots.iter() {
        xx_min = isize::min(xx_min, bot.xx() - bot.get_range());
        xx_max = isize::max(xx_max, bot.xx() + bot.get_range());

        yy_min = isize::min(yy_min, bot.yy() - bot.get_range());
        yy_max = isize::max(yy_max, bot.yy() + bot.get_range());

        zz_min = isize::min(zz_min, bot.zz() - bot.get_range());
        zz_max = isize::max(zz_max, bot.zz() + bot.get_range());
    }

    let mut result = 0;
    while true {
        let final_round = precision == 1;
        println!("xx: {} / {} / {}", xx_min, xx_max, xx_max - xx_min);
        println!("yy: {} / {} / {}", yy_min, yy_max, yy_max - yy_min);
        println!("zz: {} / {} / {}", zz_min, zz_max, zz_max - zz_min);
        println!("==> need to check {} positions", (xx_max - xx_min) * (yy_max - yy_min) * (zz_max - zz_min));

        let mut best: (usize, (isize, isize, isize)) = (0, (0, 0, 0));
        for xx in xx_min .. xx_max {
            for yy in yy_min .. yy_max {
                for zz in zz_min .. zz_max {
                    let pos = NanoBot {
                        xx,
                        yy,
                        zz,
                        range: 0,
                    };
                    let number_of_bots = bots.iter().filter(|bot| bot.in_range(&pos)).count();

                    if number_of_bots > best.0 {
                        best = (number_of_bots, (xx, yy, zz));
                        println!("new best: {:?}", best);
                    }
                }
            }
        }

        println!("best of the round (precision = {}): {:?}", precision, best);
        result = NanoBot { xx: 0, yy: 0, zz: 0, range: 0 }.manhatten(
            &NanoBot { xx: (best.1).0, yy: (best.1).1, zz: (best.1).2, range: 0}
        );
        if final_round {
            println!("This was the final round. Result = {}", result);
            break;
        }
        precision /= step;
        xx_min = ((best.1).0 * step) - 100;
        xx_max = ((best.1).0 * step) + 100;
        yy_min = ((best.1).1 * step) - 100;
        yy_max = ((best.1).1 * step) + 100;
        zz_min = ((best.1).2 * step) - 100;
        zz_max = ((best.1).2 * step) + 100;
        bots = bots_original.iter().map(|bot| bot.reduce(precision)).collect();
    }

    result as usize
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
