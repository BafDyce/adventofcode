use super::*;

enum State {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl State {
    fn modify(&mut self) {
        *self = match self {
            State::Clean => {
                State::Weakened
            }
            State::Weakened => {
                State::Infected
            }
            State::Infected => {
                State::Flagged
            }
            State::Flagged => {
                State::Clean
            }
        }
    }
}

pub fn solve(input: &[String]) -> i32 {
    let mut grid: HashMap<Position, State> = HashMap::new();

    let mut xx = 0 - (input.len() as i32 / 2);
    for line in input {
        //println!("xx = {}", xx);
        let mut yy = 0 - (line.len() as i32 / 2);
        for cc in line.chars() {
            //println!("  yy = {} --> {}", yy, cc);
            grid.insert(Position::new(xx, yy), if cc == '#' { State::Infected } else { State::Clean } );
            yy += 1;
        }
        xx += 1;
    }

    let mut virus = Virus::new();
    let mut counter = 0;

    for __ in 1..=10000000 {
        let infected = grid.entry(virus.get_pos()).or_insert(State::Clean);
        match infected {
            State::Clean => {
                virus.turn_left();
            }
            State::Weakened => {
                counter += 1;
            }
            State::Infected => {
                virus.turn_right();
            }
            State::Flagged => {
                virus.reverse();
            }
        }
        infected.modify();
        //println!("steps = {:2} | infections = {:2} | {:?}", ii, counter, virus.get_pos());
        virus.forward();
    }

    counter
}
