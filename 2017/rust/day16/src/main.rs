extern crate aocutils;
extern crate time;

mod part1;
mod part2;

#[derive(Debug)]
pub struct Performance {
    dancers: Vec<char>,
    moves: Vec<Move>,
}

impl Performance {
    fn new(dancers: String, moves: Vec<Move>) -> Performance {
        Performance {
            dancers: dancers.chars().collect(),
            moves: moves
        }
    }

    fn dance(&mut self) {
        for mov in &self.moves {
            match mov {
                &Move::Spin(nn) => {
                    let old_dancers = self.dancers.clone();
                    for ii in (16-nn)..16 {
                        let idx = 0 + (ii - (16-nn));
                        self.dancers[ idx ] = old_dancers[ii];
                    }

                    for ii in 0..(16-nn) {
                        self.dancers[ii+nn] = old_dancers[ii];
                    }
                },
                &Move::Exchange(aa, bb) => {
                    self.dancers.swap(aa, bb);
                },
                &Move::Partner(aa, bb) => {
                    let mut idx_aa = 0;
                    let mut idx_bb = 0;
                    for ii in 0..self.dancers.len() {
                        if self.dancers[ii] == aa {
                            idx_aa = ii;
                        } else if self.dancers[ii] == bb {
                            idx_bb = ii;
                        }
                    }

                    self.dancers.swap(idx_aa, idx_bb);
                }
            }
        }
    }

    fn get_position(&self) -> String {
        self.dancers.clone().into_iter().collect()
    }
}

#[derive(Debug, Clone)]
pub enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char)
}

impl Move {
    fn new(desc: &str) -> Move {
        let (mov, att) = desc.split_at(1);
        match mov {
            "s" => Move::Spin(att.parse::<usize>().unwrap()),
            "x" => {
                let positions: Vec<usize> = att.split("/")
                                                .filter_map(|x| x.parse::<usize>().ok())
                                                .collect();
                assert!(positions.len() >= 2);
                Move::Exchange(positions[0], positions[1])
            },
            "p" => {
                assert_eq!(att.len(), 3);
                let mut chars = att.chars();
                let p1 = chars.next().unwrap();
                assert_eq!(chars.next(), Some('/'));
                let p2 = chars.next().unwrap();
                Move::Partner(p1, p2)
            },
            _   => panic!("Invalid move: {}", desc),
        }
    }
}

fn main() {
    let day: i32 = 16;

    let dance: Vec<Move> = aocutils::import(day, Some("puzzle1"))
                                        .remove(0)
                                        .split(",")
                                        .map(|x| Move::new(x))
                                        .collect();
    //println!("{:?}", dance);

    let res1 = part1::solve(dance.clone(), "abcdefghijklmnop".to_string());
    let res2 = part2::solve(dance, "abcdefghijklmnop".to_string());

    println!("Results: {} and {}", res1, res2);
}
