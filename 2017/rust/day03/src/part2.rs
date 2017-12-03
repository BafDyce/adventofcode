use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Pos {
    xx: i32,
    yy: i32
}

pub fn solve(input: i32) -> i32 {
    /*
    We store each cell on the grid in a HashMap.
    When we calculate the score for a new cell, we get the values of its neighbors and sum them up.
    For this I have translated an algorithm from SO that calculates the coordinates for a given
    number in constant time.
    */
    let mut grid: HashMap<Pos, i32> = HashMap::new();
    grid.insert(Pos{xx:0, yy:0}, 1);

    for ii in 2.. {
        let mut sum = 0;
        let pos = get_pos(ii);

        let neighbors = [   Pos { xx: pos.xx + 1, yy: pos.yy - 1},
                            Pos { xx: pos.xx + 1, yy: pos.yy},
                            Pos { xx: pos.xx + 1, yy: pos.yy + 1},
                            Pos { xx: pos.xx - 1, yy: pos.yy - 1},
                            Pos { xx: pos.xx - 1, yy: pos.yy},
                            Pos { xx: pos.xx - 1, yy: pos.yy + 1},
                            Pos { xx: pos.xx, yy: pos.yy + 1},
                            Pos { xx: pos.xx, yy: pos.yy - 1}
                        ];

        for neighbor in &neighbors {
            if let Some(filled) = grid.get(neighbor) {
                sum += filled;
            }
        }

        if sum >= input {
            return sum;
        } else if ii >= input {
            return -1; // error
        }

        grid.insert(pos, sum);
    }

    -1 // error
}

// translated from https://stackoverflow.com/a/41350703
fn get_pos(nn: i32) -> Pos {
    let kk = (((nn as f32).sqrt() - 1.) / 2.).ceil() as i32;
    let mut tt = 2 * kk + 1;
    let mut mm = tt * tt;

    tt -= 1;

    if nn >= mm - tt{
        return Pos{ xx: kk - (mm - nn), yy: -kk };
    }

    mm -= tt;

    if nn >= mm - tt {
        return Pos { xx: -kk, yy: -kk + (mm - nn) };
    }

    mm -= tt;

    if nn >= mm - tt {
        return Pos{ xx: -kk + (mm - nn), yy: kk};
    }

    Pos { xx: kk, yy:kk - (mm - nn - tt)}
}
