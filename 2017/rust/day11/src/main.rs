extern crate aocutils;
extern crate regex;

mod part1;
mod part2;

#[derive(Debug)]
struct Pos {
    row: i32,
    col: i32,
}

impl Pos {
    fn step(&mut self, direction: &str) {
        let change = match direction {
            "n"     => (1, 0),
            "ne"    => ( self.col & 1, 1),
            "se"    => ( if self.col & 1 == 0 { -1 } else { 0 }, 1),
            "s"     => (-1, 0),
            "sw"    => ( if self.col & 1 == 0 { -1 } else { 0 }, -1),
            "nw"    => ( self.col & 1, -1),
            _       => panic!("Invalid move pattern!"),
        };

        self.row += change.0;
        self.col += change.1;
    }

    fn distance(&self, other: Pos) -> i32 {
        self.to_pos_cube().distance( other.to_pos_cube() )
    }

    fn to_pos_cube(&self) -> PosCube {
        let xx = self.col;
        let zz = self.row - (self.col - (self.col & 1)) / 2;
        let yy = -xx - zz;

        PosCube {
            xx: xx,
            yy: yy,
            zz: zz,
        }
    }
}

struct PosCube {
    xx: i32,
    yy: i32,
    zz: i32,
}

impl PosCube {
    fn distance(&self, other: PosCube) -> i32 {
        ( i32::abs(self.xx - other.xx)
        + i32::abs(self.yy - other.yy)
        + i32::abs(self.zz - other.zz) ) / 2
    }
}

fn main() {
    let day: i32 = 11;

    let input = aocutils::import(day, Some("puzzle1")).remove(0);

    let res1 = part1::solve(&input);
    let res2 = part2::solve(&input);

    println!("Results: {} and {}", res1, res2);
}
