/*

BENCHMARK RESULTS

*/

// allow bench feature when using unstable flag
// use: $ rustup run nightly cargo bench --features unstable
#![cfg_attr(feature = "unstable", feature(test))]


use aoc_import_magic::{import_magic, PuzzleOptions};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    io,
};

const DAY: u32 = 22;
type InputType = Game;
type OutputType1 = usize;
type OutputType2 = OutputType1;
type TodaysPuzzleOptions = PuzzleOptions<InputType>;

#[derive(Clone, Debug, Default, PartialEq)]
struct Game {
    player_one: VecDeque<usize>,
    player_two: VecDeque<usize>,
}

impl Game {
    fn play_round(&mut self) -> Option<usize> {
        let card_one = self.player_one.pop_front().unwrap();
        let card_two = self.player_two.pop_front().unwrap();

        if card_one > card_two {
            self.player_one.push_back(card_one);
            self.player_one.push_back(card_two);
        } else {
            self.player_two.push_back(card_two);
            self.player_two.push_back(card_one);
        }

        if self.player_one.is_empty() {
            Some(Self::calc_score(&self.player_two))
        } else if self.player_two.is_empty() {
            Some(Self::calc_score(&self.player_one))
        } else {
            None
        }
    }

    fn calc_score(player: &VecDeque<usize>) -> usize {
        player.iter().rev().enumerate().map(|(idx, card)| card * (idx + 1)).sum()
    }
}

impl From<Vec<String>> for Game {
    fn from(from: Vec<String>) -> Self {
        let mut iter = from.into_iter();
        iter.next().unwrap(); // "Player 1:"

        let mut player_one = VecDeque::new();
        while let Some(line) = iter.next() {
            if line.is_empty() {
                break;
            }

            player_one.push_back(line.parse().unwrap());
        }

        iter.next().unwrap();
        let player_two = iter.map(|line| line.parse().unwrap()).collect();

        Game {
            player_one,
            player_two,
        }
    }
}

fn parse_input(input: Vec<String>, _config: &HashMap<String, String>, _verbose: bool) -> InputType {
    Game::from(input)
}

fn part1(po: &TodaysPuzzleOptions) -> OutputType1 {
    let mut game = po.get_data().clone();

    loop {
        if let Some(result) = game.play_round() {
            break result;
        }
    }
}

fn part2(po: &TodaysPuzzleOptions) -> OutputType2 {
    let Game { player_one, player_two} = po.get_data().clone();

    play_recursive(player_one, player_two).1
}

enum Player {
    One,
    Two,
}

fn play_recursive(mut player_one: VecDeque<usize>, mut player_two: VecDeque<usize>) -> (Player, usize) {
    //println!("Starting game with decks:");
    //println!("Player 1: {:?}", player_one);
    //println!("Player 2: {:?}", player_two);

    let mut history = HashSet::new();

    //let mut round = 1;
    loop {
        //println!("\n-- Round {} --", round);
        //println!("Player 1: {:?}", player_one);
        //println!("Player 2: {:?}", player_two);

        let game_state = (player_one.clone(), player_two.clone());
        if history.contains(&game_state) {
            //println!("We ended up in a cycle");
            break (Player::One, 0);
        } else {
            history.insert(game_state);
        }

        let card_one = player_one.pop_front().unwrap();
            let card_two = player_two.pop_front().unwrap();

            //println!("Player 1 plays: {}", card_one);
            //println!("Player 2 plays: {}", card_two);

        let winner = if player_one.len() >= card_one && player_two.len() >= card_two {
            //println!("Playing a sub-game to determine winner");
            let player_one_subgame = player_one.iter().take(card_one).map(ToOwned::to_owned).collect();
            let player_two_subgame = player_two.iter().take(card_two).map(ToOwned::to_owned).collect();
            play_recursive(player_one_subgame, player_two_subgame).0
        } else {
            if card_one > card_two {
                Player::One
            } else {
                Player::Two
            }
        };

        match winner {
            Player::One => {
                //println!("Player 1 wins round {}", round);
                player_one.push_back(card_one);
                player_one.push_back(card_two);
            }
            Player::Two => {
                //println!("Player 2 wins round {}", round);
                player_two.push_back(card_two);
                player_two.push_back(card_one);
            }
        }

        if player_one.is_empty() {
            break (Player::Two, Game::calc_score(&player_two));
        } else if player_two.is_empty() {
            break (Player::One, Game::calc_score(&player_one));
        }

        //round += 1;
    }

}


// =================================================================================================
// End of actual logic
// What follows is the main function glue as well as tests + benchmarking code
// =================================================================================================
fn main() -> Result<(), io::Error> {
    println!("AoC 2020 | Day {}", DAY);

    // This function is pure magic (see ../../aoc_import_magic/lib.rs) because it
    // 1. parses command line arguments
    // 2. reads the input file for the correct day
    // 3. uses `parse_input` as a parsing function
    // 4. returns a nice usable struct which contains everything which we need for the actual puzzle
    let puzzle = import_magic(DAY, parse_input)?;
    if !puzzle.skip_p1 {
        let res1 = part1(&puzzle);
        println!("Part 1 result: {}", res1);
    };

    let res2 = part2(&puzzle);
    println!("Part 2 result: {}", res2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_import_magic::{import_magic_with_params, PuzzleOptions};

    pub(super) fn import_helper(inputname: &str) -> PuzzleOptions<InputType> {
        let params = ["appname", "--input", inputname];
        import_magic_with_params(DAY, parse_input, &params).unwrap()
    }

    fn test_case_helper(inputname: &str, sol1: Option<OutputType1>, sol2: Option<OutputType2>) {
        let po = import_helper(inputname);

        if let Some(sol1) = sol1 {
            let res1 = part1(&po);
            assert_eq!(sol1, res1, "part1");
        }

        if let Some(sol2) = sol2 {
            let res2 = part2(&po);
            assert_eq!(sol2, res2, "part2");
        }
    }

    #[test]
    fn example_1() {
        test_case_helper("example1", Some(306), Some(291))
    }
}

#[cfg(all(feature = "unstable", test))]
mod bench {
    extern crate test;

    use super::*;
    use aoc_import_magic::test_helper_import_config;
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };
    use test::Bencher;

    fn helper_read_file(fname: &str) -> Vec<String> {
        BufReader::new(File::open(fname).unwrap())
            .lines()
            .map(|line| line.unwrap())
            .collect()
    }

    #[bench]
    fn bench_parsing(bb: &mut Bencher) {
        let input = helper_read_file(&format!("../../_inputs/day{:02}/real1.input", DAY));
        let config = test_helper_import_config(DAY, "real1");

        bb.iter(|| test::black_box(parse_input(input.to_owned(), &config, false)));
    }

    #[bench]
    fn bench_part1(bb: &mut Bencher) {
        let puzzle_options = tests::import_helper("real1");
        bb.iter(|| test::black_box(part1(&puzzle_options)));
    }

    #[bench]
    fn bench_part2(bb: &mut Bencher) {
        let puzzle_options = tests::import_helper("real1");
        bb.iter(|| test::black_box(part2(&puzzle_options, None)));
    }
}
